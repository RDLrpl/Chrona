use chrona_app::{app::App, app_struct::{AppConfiguration, AppData}};
use vulkano::device::DeviceExtensions;
use winit::event_loop::{ControlFlow, EventLoop};
use chrona_utils::{binding::ResultExt, modeldata::ModelData};

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();

    let device_extensions: DeviceExtensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    }; // GPU technologies and features. e.x: Ray Trassing

    let appconfig = AppConfiguration::new(
        "ChronaEngine APP", // app name
        [1, 0, 0], // app version
        800, 600, // width & height
        false  // fullscreen?
    );

    // AppData>>

    // Models
    let modelsdat = vec![
        // path, position, rotation, scale
        ModelData::init("assets/monkey_with_texture/monkey.obj".to_string(), [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.5, 0.5, 0.5]),
        ModelData::init("assets/greenmonkey/monkey.obj".to_string(), [-1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.5, 0.5, 0.5]),
    ];
    
    let app_data = AppData::load(modelsdat);

    let mut app = App::new(appconfig, device_extensions, app_data);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).expect_me("[CHRONA]: loop'panic>");
}