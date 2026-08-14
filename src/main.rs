use chrona_app::{app::App};
use chrona_engine::eab::eab::{GameData, GameFunc};
use vulkano::device::DeviceExtensions;
use winit::event_loop::{ControlFlow, EventLoop};
use chrona_utils::data::{AppConfiguration, ModelData, SceneData, WorldData};

mod game;

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
        true, // fullscreen?

        device_extensions,
    
        0, // 0 - unlimited
    );
    // AppData>>

    // Models
    let models_scene_one = vec![
        // path, position, rotation, scale
        ModelData::init(1, "assets/monkey_with_texture/monkey.obj".to_string(), [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.1, 0.1, 0.1]),
        ModelData::init(2, "assets/greenmonkey/monkey.obj".to_string(), [-1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.5, 0.5, 0.5]),
    ];

    let models_scene_two = vec![
        // path, position, rotation, scale
        ModelData::init(1, "assets/monkey_with_texture/monkey.obj".to_string(), [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.25, 0.25, 0.25])
    ];

    // World
    let world = WorldData::init(
        vec![
            SceneData::init(0, models_scene_one),
            SceneData::init(1, models_scene_two)
        ]
    );


    // Handle Functions
    let functions = GameFunc {
        on_frame_update: game::on_frame_update
    };

    let app_data = GameData::load(world, functions);

    let mut app = App::new(appconfig, app_data);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).expect("[CHRONA]: loop'panic>");
}