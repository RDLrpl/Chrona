use std::sync::Arc;

use chrona_utils::binding::{OptionExt, ResultExt};
use chrona_vk::{engine::{layout::{loadout::obj::{Model, Transform}, world::world::{Scene, World}}, shr::CameraUBO}, vkinit::{devices::GpuDevices, framecontext::FrameContext, pipeline::Executor, render::Render}};
use glam::{Mat4, Vec3};
use vulkano::{Version, VulkanLibrary, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents}, descriptor_set::{DescriptorSet, WriteDescriptorSet}, device::DeviceExtensions, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, pipeline::{Pipeline, PipelineBindPoint, graphics::viewport::Viewport}, swapchain::{self, Surface, SwapchainPresentInfo}, sync::GpuFuture};
use winit::{application::ApplicationHandler, event::WindowEvent, window::{Window}};

use crate::app_struct::{AppConfiguration, AppData, AppState};

pub struct App {
    window: Option<Arc<Window>>,

    // configuration>>
    app_config: AppConfiguration,
    device_exstensions: DeviceExtensions,
    app_data: AppData,

    // vk>>
    appstate: Option<AppState>,

    // world
    world: Option<World>,
}

impl App {
    pub fn new(app_config: AppConfiguration, device_exstensions: DeviceExtensions, app_data: AppData) -> Self {
        Self {
            window: None, 
            
            device_exstensions,
            app_config,
            app_data,
            
            world: None,
            
            appstate: None,
        }
    }
    
    // HELPERS!
    fn hstate(&mut self) -> &mut AppState {
        self.appstate.as_mut().expect_me("[CHRONA]: APPSTATE not initialized'panic>")
    }

    fn hwindow(&self) -> &Arc<Window> {
        self.window.as_ref().expect_me("[CHRONA]: window not initialized'panic>")
    }

    // appstate...
    fn hdevices(&self) -> &GpuDevices {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").gpudevices
    }

    fn hrender(&self) -> &Render {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").render
    }

    fn hexecutor(&self) -> &Executor {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").executor
    }
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes()
            .with_title(self.app_config.projname.clone())
            .with_inner_size(winit::dpi::PhysicalSize::new(self.app_config.width, self.app_config.height))
            .with_fullscreen(self.app_config.fullscreen.then_some(
                winit::window::Fullscreen::Borderless(None)
            ))
        ).unwrap());
        self.window = Some(window.clone());
        // Engine Init >>

        // VK Instance
        let library = VulkanLibrary::new().expect_me("[CHRONA]: 'VK-init`NO local VK LIB'panic>");
        let required_extensions = Surface::required_extensions(&event_loop).expect_me("[CHRONA]: 'VK-init`Surface>required_extensions'panic>");
        let appinstance = Instance::new(
            library,
            InstanceCreateInfo {
                application_name: Some(self.app_config.projname.clone()),
                application_version: Version { major: self.app_config.app_version[0], minor: self.app_config.app_version[1], patch: self.app_config.app_version[2] },
                engine_name: Some("CHRONA".to_string()),
                engine_version: Version { major: 1, minor: 0, patch: 0 },
                max_api_version: None,
                enabled_extensions: required_extensions,
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        ).expect_me("[CHRONA]: 'VK-init`FAILED TO CREATE VK_INSTANCE'panic>");

        // GPU Devices
        let gpudevices = GpuDevices::init(appinstance.clone(), self.device_exstensions);
        // Render:
        let render = Render::init(appinstance.clone(), gpudevices.clone(), window);

        // viewport:
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [self.app_config.width as f32, self.app_config.height as f32],
            depth_range: 0.0..=1.0,
        };

        // pipeline:
        let executor = Executor::init(gpudevices.logical_device.clone(), render.render_pass.clone(), viewport);

        // framecontext:
        let framecontext = FrameContext::init(gpudevices.clone(), render.clone());
        
        // * Scenes>
        let mut scene = Scene { models: vec![] };

        for modelsdat in self.app_data.model_datas.clone() {
            let model = Model::load(
                modelsdat.path, 
                render.memory_allocator.clone(), 
                gpudevices.queue.clone(), 
                Transform::push(
                    modelsdat.transform.p_xyz,
                    modelsdat.transform.r_xyz,
                    modelsdat.transform.s_xyz,
                ),
                executor.cmd_allocator.clone(),
            );
            scene.models.push(model);
        }

        // * World> 
        self.world = Some(World { scenes: vec![scene] });

        // APPSTATE>>
        self.appstate = Some(AppState::init(
            appinstance, 
            gpudevices,
            render,
            executor,
            framecontext
        ));
        
        // Engine Init END<<

        println!("[CHRONA]: GPU [{}] is using for render!'LOG", self.hdevices().device_name)
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            
            WindowEvent::Resized(new_size) => {
                if new_size.width == 0 || new_size.height == 0 {
                    return;
                }

                let window = self.hwindow().clone();
                let gpudevices = self.hstate().gpudevices.clone();
                let extent: [u32; 2] = window.inner_size().into();

                self.hstate().render.recreate_swapchain(&gpudevices, window);

                self.hstate().executor.viewport.extent = [extent[0] as f32, extent[1] as f32];
            }

            WindowEvent::Moved(_) => {
                self.hstate().moving = true;
            }
        
            WindowEvent::RedrawRequested => {
                if self.hstate().moving {
                    self.hstate().moving = false;
                    return;
                }
                
                let window = self.hwindow().clone();
                let extent: [u32; 2] = window.inner_size().into();
                
                self.hstate().framecontext.previous_frame_end.as_mut().unwrap().cleanup_finished();

                let (image_index, _, acquire_future) = 
                match swapchain::acquire_next_image(self.hrender().swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("[CHRONA]: acquire_next_image'warn>: {:?}", e);
                        return;
                    }
                };

                let aspect = extent[0] as f32 / extent[1] as f32;

                let view = Mat4::look_at_rh(Vec3::new(0.0, 1.0, 2.4), Vec3::ZERO, Vec3::Y);
                let mut proj = Mat4::perspective_rh(60_f32.to_radians(), aspect, 0.1, 100.0);
                proj.y_axis.y *= -1.0;

                let ubo = CameraUBO {
                    model: Mat4::IDENTITY.to_cols_array_2d(),
                    view:  view.to_cols_array_2d(),
                    proj:  proj.to_cols_array_2d(),
                };

                let uniform_sub = self.hstate().framecontext.uniform_allocator
                    .allocate_sized::<CameraUBO>().unwrap();
                *uniform_sub.write().unwrap() = ubo;

                let layout = self.hexecutor().pipeline.layout().set_layouts()[0].clone();
                let camera_descriptor = DescriptorSet::new(
                    self.hstate().framecontext.descriptor_allocator.clone(),
                    layout,
                    [
                        WriteDescriptorSet::buffer(0, uniform_sub)
                    ],
                    [],
                ).unwrap();

                let mut builder = AutoCommandBufferBuilder::primary(
                    self.hexecutor().cmd_allocator.clone(),
                    self.hstate().gpudevices.queue.queue_family_index(),
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
                                self.hrender().framebuffers[image_index as usize].clone()
                            )
                        },
                        SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                    ).unwrap()
                    .set_viewport(0, [self.hexecutor().viewport.clone()].into_iter().collect()).unwrap()
                    .bind_pipeline_graphics(self.hexecutor().pipeline.clone()).unwrap()
                    .bind_descriptor_sets(   
                        PipelineBindPoint::Graphics,
                        self.hexecutor().pipeline.layout().clone(),
                        0,                       
                        camera_descriptor,
                    ).unwrap();
                
                let scene = &self.world.as_ref().unwrap().scenes[0];
                
                self.hexecutor().draw(&mut builder, &scene, &self.appstate.as_ref().unwrap().framecontext);

                builder.end_render_pass(Default::default()).unwrap();
                // Render END<<

                let command_buffer = builder.build().unwrap();

                let future = self.hstate().framecontext.previous_frame_end
                    .take().unwrap()
                    .join(acquire_future)
                    .then_execute(self.hstate().gpudevices.queue.clone(), command_buffer).unwrap()
                    .then_swapchain_present(
                        self.hstate().gpudevices.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            self.hrender().swapchain.clone(),
                            image_index,
                        ),
                    )
                    .then_signal_fence_and_flush();

                match future {
                    Ok(f) => {
                        self.hstate().framecontext.previous_frame_end = Some(f.boxed());
                    }
                    Err(e) => {
                        println!("[CHRONA]: flush'warn>: {e:?}'");
                        self.hstate().framecontext.previous_frame_end = Some(
                            vulkano::sync::now(self.hstate().gpudevices.logical_device.clone()).boxed()
                        );
                    }
                }

                window.request_redraw();
            }

            _ => ()
        }

        
    }

}