use crate::engine::layout::obj::Model;

pub struct World {
    pub scenes: Vec<Scene>
}

pub struct Scene {
    pub models: Vec<Model>,
}