use std::sync::Arc;

use chrona_utils::binding::{OptionExt, ResultExt};
use chrona_vk::{engine::layout::testobject::cube, vkinit::{devices::GpuDevices, pipeline::{Executor, PushConstants}, render::Render}};
use glam::Mat4;
use vulkano::{Version, VulkanLibrary, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents}, device::DeviceExtensions, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, pipeline::{Pipeline, graphics::viewport::Viewport}, swapchain::{self, Surface, SwapchainPresentInfo}, sync::GpuFuture};
use winit::{application::ApplicationHandler, event::WindowEvent, window::{Window}};

pub struct AppConfiguration {
    width: u32,
    height: u32,
    fullscreen: bool,
    app_version: [u32; 3],
    projname: String,
}

pub struct App {
    window: Option<Arc<Window>>,

    // configuration>>
    app_config: AppConfiguration,
    device_exstensions: DeviceExtensions,

    // vk>>
    appstate: Option<AppState>,
}

struct AppState {
    _vk_instance: Arc<Instance>,
    gpudevices: GpuDevices,
    render: Render,
    executor: Executor,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    rotation: f32,
}

impl AppState {
    fn init(vk_instance: Arc<Instance>, gpudevices: GpuDevices, render: Render, executor: Executor) -> Self {
        let previous_frame_end = Some(
            vulkano::sync::now(gpudevices.logical_device.clone()).boxed()
        );

        Self { 
            _vk_instance: vk_instance, 
            gpudevices,
            render,
            executor,
            previous_frame_end,
            rotation: 0.3,
        }
    }
}
impl AppConfiguration {
    pub fn new(projname: &str, app_version: [u32; 3], width: u32, height: u32, fullscreen: bool) -> Self {
        Self {
            projname: projname.to_string(),
            app_version,
            width,
            height,
            fullscreen
        }
    }
}

impl App {
    pub fn new(app_config: AppConfiguration, device_exstensions: DeviceExtensions) -> Self {
        Self {
            window: None, 
            
            device_exstensions,
            app_config,

            appstate: None,
        }
    }
    
    // HELPERS!
    fn hgpu_devices(&self) -> &GpuDevices {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").gpudevices
    }

    fn hrender(&self) -> &Render {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").render
    }

    fn hexecutor(&self) -> &Executor {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>").executor
    }
    
    fn hwindow(&self) -> &Arc<Window> {
        self.window.as_ref().expect_me("[CHRONA]: window not initialized'panic>")
    }
    
    fn _hinstance(&self) -> &Arc<Instance> {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized'panic>")._vk_instance
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
        // INIT VK in APP!!!! >>

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
        // scene loader:
        let cube = cube();

        // viewport:
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };
        // pipeline:
        let executor = Executor::init(render.memory_allocator.clone(), cube, gpudevices.logical_device.clone(), render.render_pass.clone(), viewport);
        // APPSTATE>>
        self.appstate = Some(AppState::init(
            appinstance, 
            gpudevices,
            render,
            executor
        ));
        // INIT VK END<<

        println!("[CHRONA]: GPU [{}] is using for render!'LOG", self.hgpu_devices().device_name)
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            
            WindowEvent::Resized(_) => {

            }

            WindowEvent::RedrawRequested => {
                let (image_index, _suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(
                        self.hrender().swapchain.clone(), None
                    ).map_err(|e| e.unwrap()) {
                        Ok(r) => r,
                        Err(e) => panic!("failed to acquire next image: {e:?}"),
                    };

                let mut builder = AutoCommandBufferBuilder::primary(
                    self.hexecutor().cmd_allocator.clone(),
                    self.hgpu_devices().queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                ).unwrap();

                let state = self.appstate.as_mut().unwrap();
                state.rotation += 0.01;

                let transform = Mat4::from_rotation_z(state.rotation) * Mat4::from_rotation_x(state.rotation) * Mat4::from_rotation_y(state.rotation);
                let push = PushConstants {
                    transform: transform.to_cols_array_2d(),
                };

                unsafe { builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.01, 0.01, 0.01, 1.0].into())],
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
                    .bind_vertex_buffers(0, self.hexecutor().vertex_buffer.clone()).unwrap()
                    .push_constants(
                        self.hexecutor().pipeline.clone().layout().clone(),
                        0,
                        push,
                    ).unwrap()
                    .draw(self.hexecutor().vertex_buffer.len() as u32, 1, 0, 0).unwrap()
                    .end_render_pass(Default::default()).unwrap()
                    
                    };


                let command_buffer = builder.build().unwrap();
                
                let state = self.appstate.as_mut().unwrap();
                let future = state.previous_frame_end
                    .take().unwrap()
                    .join(acquire_future)
                    .then_execute(state.gpudevices.queue.clone(), command_buffer).unwrap()
                    .then_swapchain_present(
                        state.gpudevices.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            state.render.swapchain.clone(),
                            image_index,
                        ),
                    )
                    .then_signal_fence_and_flush();

                match future {
                    Ok(f) => {
                        state.previous_frame_end = Some(f.boxed());
                    }
                    Err(e) => {
                        state.previous_frame_end = Some(
                            vulkano::sync::now(state.gpudevices.logical_device.clone()).boxed()
                        );
                    }
                }
                self.hwindow().request_redraw();
            }

            _ => ()
        }

        
    }

}