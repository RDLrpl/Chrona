use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};

use chrona_api::aev::aev::CHAPI;
use chrona_engine::{eab::eab::GameData, engine::Engine};
use chrona_utils::data::AppConfiguration;
use chrona_world::engine::layout::{loadout::obj::{Model, Transform}, world::world::{Scene, World}};
use winit::{application::ApplicationHandler, event::{DeviceEvent, WindowEvent}, window::{Window}};



pub struct App {
    app_config: AppConfiguration,
    last_frame_time: Instant,

    pub engine: Option<Engine>,
    pub app_data: GameData,

    pub world_link: Option<Rc<RefCell<World>>>,

    chapi: CHAPI,
}

impl App {
    pub fn new(app_config: AppConfiguration, game_content: GameData) -> Self {
        Self {
            app_config,
            engine: None,
            app_data: game_content,
            world_link: None,
            last_frame_time: Instant::now(),
            chapi: CHAPI::init(),
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
        
        let engine = Engine::init(self.app_config.clone(), window.clone(), event_loop);
        
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

        let world = World::make(scenes);

        let world_link: Rc<RefCell<World>> = Rc::new(RefCell::new(world));

        self.world_link = Some(world_link);
        self.engine = Some(engine);
        
        // Engine Init END<<
        println!("[CHRONA]: GPU [{}] is using for render!'LOG", self.engine.as_ref().unwrap().gpu_for_render());
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(engine) = &self.engine {
            if self.app_config.max_framerate != 0 {
                let frame_duration = 1.0 / (self.app_config.max_framerate as f32);

                std::thread::sleep(std::time::Duration::from_secs_f32(frame_duration));
            }

            let now = Instant::now();
            
            let dt = now.duration_since(self.last_frame_time).as_secs_f32();

            self.chapi.delta_time = dt;
            self.last_frame_time = now;
            
            engine.window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    )
    {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.chapi.mouse_handler.update_moution(delta);
            }
            _ => {}
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

            WindowEvent::Focused(..)=> {
                let engine = self.engine.as_mut().unwrap();

                engine.focused(&mut self.chapi);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                self.chapi.keyboard_handler.update_key(event.physical_key, event.state);
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.chapi.keyboard_handler.update_modifiers(modifiers.state());
            }

            WindowEvent::Resized(new_size) => {
                let engine = self.engine.as_mut().unwrap();

                engine.sft.pending_resize = Some((new_size.width, new_size.height));
            }
            
            WindowEvent::RedrawRequested => {
                let world_link = self.world_link.as_ref().unwrap();
                let engine = self.engine.as_mut().unwrap();

                engine.render(world_link, &self.app_data, &self.chapi);

                self.chapi.mouse_handler.update();
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.chapi.mouse_handler.update_pos(position);
            }

            _ => ()
        }

        
    }

}