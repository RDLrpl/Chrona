use std::sync::Arc;

use chrona_utils::binding::{OptionExt, ResultExt};
use chrona_vk::vkinit::GpuDevices;
use vulkano::{Version, VulkanLibrary, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, swapchain::Surface};
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
    app_config: AppConfiguration,

    appstate: Option<AppState>,
}

struct AppState {
    _vk_instance: Arc<Instance>,
    gpudevices: GpuDevices,
}

impl AppState {
    fn init(vk_instance: Arc<Instance>, gpudevices: GpuDevices) -> Self {
        Self { 
            _vk_instance: vk_instance, 
            gpudevices
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
    pub fn new(app_config: AppConfiguration) -> Self {
        Self {
            window: None,
            appstate: None,

            app_config,
        }
    }
    
    // HELPERS!
    fn hgpu_devices(&self) -> &GpuDevices {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized").gpudevices
    }
    
    fn _hwindow(&self) -> &Arc<Window> {
        self.window.as_ref().expect_me("[CHRONA]: window not initialized")
    }
    
    fn _hinstance(&self) -> &Arc<Instance> {
        &self.appstate.as_ref().expect_me("[CHRONA]: APPSTATE not initialized")._vk_instance
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
        let gpudevices = GpuDevices::init(appinstance.clone());

        // APPSTATE>>
        self.appstate = Some(AppState::init(appinstance, gpudevices));

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
            _ => ()
        }

        
    }

}