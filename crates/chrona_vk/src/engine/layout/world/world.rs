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

impl World {
    pub fn make(scenes: Vec<Scene>) -> Self {
        Self {
            curscene: 0,
            scenes
        }
    }

    pub fn return_cur_scene(&self) -> Option<&Scene> {
        let target_id = self.curscene; 

        self.scenes.iter().find(|s| s.id == target_id)
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