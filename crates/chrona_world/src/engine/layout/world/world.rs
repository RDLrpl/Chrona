use std::u32;

use log_once::warn_once;

use crate::engine::layout::loadout::obj::{Model};

#[derive(Clone)]
pub struct World {
    pub curscene: u32,
    pub scenes: Vec<Scene>
}

#[derive(Clone)]
pub struct Scene {
    pub id: u32,
    pub models: Vec<Model>,
}

static NOSCENE: Scene = Scene {
    id: u32::MAX,
    models: Vec::new(),
};

impl World {
    pub fn make(scenes: Vec<Scene>) -> Self {
        Self {
            curscene: 0,
            scenes
        }
    }

    pub fn return_cur_scene(&self) -> &Scene {
        let target_id = self.curscene; 

        if let Some(scene) = self.scenes.iter().find(|s| s.id == target_id) {
            scene
        } else {
            warn_once!("Using empty scene: no available scenes found.");
            &NOSCENE
        }
    }

    pub fn return_mutcur_scene(&mut self) -> Option<&mut Scene> {
        let target_id = self.curscene; 

        self.scenes.iter_mut().find(|s| s.id == target_id)
    }

}

impl Scene {
    pub fn make(id: u32, models: Vec<Model>) -> Self {
        Self { 
            id, 
            models
        }
    }

    pub fn return_object(&mut self, id: u32) -> Option<&mut Model> {
        self.models.iter_mut().find(|m| m.id == id)
    }
}