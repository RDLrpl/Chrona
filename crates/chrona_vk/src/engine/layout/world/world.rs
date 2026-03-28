use crate::engine::layout::loadout::obj::Model;

pub struct World {
    pub scenes: Vec<Scene>
}

pub struct Scene {
    pub models: Vec<Model>,
}