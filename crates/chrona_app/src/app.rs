use std::{sync::Arc, time::Instant};

use chrona_engine::{eab::eab::GameData, engine::Engine};
use chrona_utils::data::AppConfiguration;
use chrona_world::engine::layout::{loadout::obj::{Model, Transform}, world::world::{Scene, World}};
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};



pub struct App {
    app_config: AppConfiguration,
    last_frame_time: Instant,

    pub engine: Option<Engine>,
    pub app_data: GameData,

    pub render_world: Option<World>,
}

impl App {
    pub fn new(app_config: AppConfiguration, game_content: GameData) -> Self {
        Self {
            app_config,
            engine: None,
            app_data: game_content,
            render_world: None,
            last_frame_time: Instant::now(),
        }
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
        
        let engine = Engine::init(self.app_config.clone(), window, event_loop);
        
        // World>
        let mut scenes = vec![];

        for scensdat in self.app_data.world.scenesdata.clone() {
            let mut models = vec![];

            for modelsdat in scensdat.modelsdata.clone() {
                let model = Model::load(
                    modelsdat.id,
                    modelsdat.path, 
                    engine.appstate.render.memory_allocator.clone(), 
                    engine.appstate.gpudevices.queue.clone(), 
                    Transform::push(
                        modelsdat.transform.p_xyz,
                        modelsdat.transform.r_xyz,
                        modelsdat.transform.s_xyz,
                    ),
                    engine.appstate.executor.cmd_allocator.clone(),
                );
                models.push(model);
            }
            scenes.push(Scene::make(scensdat.id, models));
        }


        self.render_world = Some(World::make(scenes));
        self.engine = Some(engine);

        // Engine Init END<<
        println!("[CHRONA]: GPU [{}] is using for render!'LOG", self.engine.as_ref().unwrap().gpu_for_render());
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(engine) = &self.engine {
            std::thread::sleep(std::time::Duration::from_millis(16));
            let now = Instant::now();
            
            let dt = now.duration_since(self.last_frame_time).as_secs_f32();

            self.last_frame_time = now;
            
            engine.window.request_redraw();
        }
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
                let engine = self.engine.as_mut().unwrap();

                engine.sft.pending_resize = Some((new_size.width, new_size.height));
            }

            WindowEvent::Moved(_) => {
                // let engine = self.engine.as_mut().unwrap();

                // engine.sft.sgr = true;
            }
        
            WindowEvent::RedrawRequested => {
                let engine = self.engine.as_mut().unwrap();

                engine.render(&mut self.render_world.as_mut().unwrap(), &self.app_data);
            }

            _ => ()
        }

        
    }

}