use chrona_app::app::{App, AppConfiguration};
use vulkano::device::DeviceExtensions;
use winit::event_loop::{ControlFlow, EventLoop};
use chrona_utils::binding::ResultExt;

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

    let mut app = App::new(appconfig, device_extensions);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).expect_me("[CHRONA]: loop'panic>");
}