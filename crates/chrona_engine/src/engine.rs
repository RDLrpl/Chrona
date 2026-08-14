use std::{cell::RefCell, rc::Rc, sync::Arc, time::Duration};

use chrona_api::aev::aev::CHAPI;
use chrona_utils::data::AppConfiguration;
use chrona_vk::{pipelines::vertexshader::CameraUBO, vkinit::{devices::GpuDevices, framecontext::FrameContext, pipeline::Executor, render::Render}};
use chrona_world::engine::layout::world::world::World;
use glam::{Mat4, Vec3};
use vulkano::{Version, sync::GpuFuture, VulkanLibrary, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, pipeline::{Pipeline, PipelineBindPoint, graphics::viewport::Viewport}, swapchain::{self, Surface, SwapchainPresentInfo}};
use winit::window::Window;

use crate::{eab::eab::GameData, graphics::state::AppState};

pub struct EngineSFT {
    pub sgr: bool,
    
    pub pending_resize: Option<(u32, u32)>,
}

pub struct Engine {
    pub window: Arc<Window>,
    pub appstate: AppState,

    pub sft: EngineSFT,
    pub mousepos: [f64; 2],
}

impl EngineSFT {
    pub fn new() -> Self {
        Self {
            sgr: false,
            pending_resize: None,
        }
    }
}
impl Engine {
    pub fn init(
        app_configuration: AppConfiguration, 
        window: Arc<Window>, 
        event_loop: &winit::event_loop::ActiveEventLoop
    ) -> Self {
        // VK Instance
        let library = VulkanLibrary::new().expect("[CHRONA]: 'VK-init`NO local VK LIB'panic>");
        let required_extensions = Surface::required_extensions(&event_loop).expect("[CHRONA]: 'VK-init`Surface>required_extensions'panic>");
        let vk_instance = Instance::new(
            library,
            InstanceCreateInfo {
                application_name: Some(app_configuration.projname.clone()),
                application_version: Version { major: app_configuration.app_version[0], minor: app_configuration.app_version[1], patch: app_configuration.app_version[2] },
                engine_name: Some("CHRONA".to_string()),
                engine_version: Version { major: 1, minor: 0, patch: 0 },
                max_api_version: None,
                enabled_extensions: required_extensions,
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        ).expect("[CHRONA]: 'VK-init`FAILED TO CREATE VK_INSTANCE'panic>");

        // GPU Devices
        let gpudevices = GpuDevices::init(vk_instance.clone(), app_configuration.device_extensions);
        // Render:
        let render = Render::init(vk_instance.clone(), gpudevices.clone(), window.clone());

        // viewport:
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [app_configuration.width as f32, app_configuration.height as f32],
            depth_range: 0.0..=1.0,
        };

        // pipeline:
        let executor = Executor::init(gpudevices.logical_device.clone(), render.render_pass.clone(), viewport);

        // framecontext:
        let framecontext = FrameContext::init(gpudevices.clone(), render.clone(), executor.pipeline.layout().clone());

        Self {
            window,
            appstate: AppState { 
                vk_instance, 
                gpudevices, 
                render, 
                executor, 
                framecontext
            },
            mousepos: [0.0, 0.0],
            sft: EngineSFT::new(),
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let window = self.window.clone();
        let state = &mut self.appstate;

        let gpudevices = state.gpudevices.clone();
        let extent: [u32; 2] = [width, height];

        state.render.recreate_swapchain(&gpudevices, window);

        for fence in state.framecontext.frame_fences.iter_mut() {
            *fence = None;
        }

        state.executor.viewport.extent = [extent[0] as f32, extent[1] as f32];
    }

    pub fn render(&mut self, world_link: &Rc<RefCell<World>>, app_data: &GameData, api: &CHAPI) {
        if self.sft.sgr {
            self.sft.sgr = false;

            return;
        }

        if let Some((w, h)) = self.sft.pending_resize.take() {
            self.resize(w, h);
        }

        let window = self.window.clone();
        let state = &mut self.appstate;
        let extent: [u32; 2] = window.inner_size().into();
        
        // state.framecontext.frame_fences[frame_idx].as_mut().unwrap().cleanup_finished();

        let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(
            state.render.swapchain.clone(),  
            Some(Duration::from_millis(16))
        ) {
            Ok(r) => r,
            Err(_e) => {
                let gpudevices = state.gpudevices.clone();
                state.render.recreate_swapchain(&gpudevices, window);

                for fence in state.framecontext.frame_fences.iter_mut() {
                    *fence = None;
                }

                return;
            }
        };

        let frame_idx = image_index as usize;

        if let Some(fence) = state.framecontext.frame_fences[frame_idx].take() {
            fence.wait(None).unwrap();
        }

        let aspect = extent[0] as f32 / extent[1] as f32;

        let view = Mat4::look_at_rh(Vec3::new(0.0, 1.0, 2.4), Vec3::ZERO, Vec3::Y);
        let mut proj = Mat4::perspective_rh(60_f32.to_radians(), aspect, 0.1, 100.0);
        proj.y_axis.y *= -1.0;

        let ubo = CameraUBO {
            model: Mat4::IDENTITY.to_cols_array_2d(),
            view:  view.to_cols_array_2d(),
            proj:  proj.to_cols_array_2d(),
        };

        *state.framecontext.uniform_subbuffers[frame_idx].write().unwrap() = ubo;

        let mut builder = AutoCommandBufferBuilder::primary(
            state.executor.cmd_allocator.clone(),
            state.gpudevices.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();

        // Render START>>
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![
                        Some([0.0, 0.0, 0.0, 1.0].into()), // RGBA
                        Some(1.0f32.into()),               // DEPTH
                    ],
                    ..RenderPassBeginInfo::framebuffer(
                        state.render.framebuffers[image_index as usize].clone()
                    )
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            ).unwrap()
            .set_viewport(0, [state.executor.viewport.clone()].into_iter().collect()).unwrap()
            .bind_pipeline_graphics(state.executor.pipeline.clone()).unwrap()
            .bind_descriptor_sets(   
                PipelineBindPoint::Graphics,
                state.executor.pipeline.layout().clone(),
                0,                       
                state.framecontext.camera_descriptors[frame_idx].clone(),
            ).unwrap();
        
        let scene = {
            let world = world_link.borrow();
            world.return_cur_scene().expect("...").clone()
        };

        (app_data.gamefuncs.on_frame_update)(Rc::clone(&world_link), api);

        state.executor.draw(&mut builder, &scene, &state.framecontext);

        builder.end_render_pass(Default::default()).unwrap();

        let command_buffer = builder.build().unwrap();

        let future = vulkano::sync::now(state.gpudevices.logical_device.clone())
            .join(acquire_future)
            .then_execute(state.gpudevices.queue.clone(), command_buffer).unwrap() 
            .then_swapchain_present(
                state.gpudevices.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(state.render.swapchain.clone(), image_index),
            )
            .boxed()
            .then_signal_fence_and_flush();

        match future {
            Ok(f) => {
                state.framecontext.frame_fences[frame_idx] = Some(f);
            }
            Err(vulkano::Validated::Error(vulkano::VulkanError::OutOfDate)) => {
                let gpudevices = state.gpudevices.clone();
                state.render.recreate_swapchain(&gpudevices, window.clone());
                state.framecontext.frame_fences[frame_idx] = None;
            }
            Err(e) => {
                println!("[CHRONA]: flush'warn>: {e:?}'");
                state.framecontext.frame_fences[frame_idx] = None;
            }
        }

        if suboptimal {
            let gpudevices = state.gpudevices.clone();
            state.render.recreate_swapchain(&gpudevices, window.clone());
        }
    }


    pub fn gpu_for_render(&self) -> &str {
        &self.appstate.gpudevices.device_name
    }
}