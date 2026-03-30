use crate::engine::layout::loadout::obj::{Model};

pub struct World {
    pub scenes: Vec<Scene>
}

pub struct Scene {
    pub id: u16,
    pub models: Vec<Model>,
}

impl World {
    pub fn make(scenes: Vec<Scene>) -> Self {
        Self {
            scenes
        }
    }
}

impl Scene {
    pub fn make(id: u16, models: Vec<Model>) -> Self {
        /*let mut models = vec![] ;
        memory_allocator: Arc<StandardMemoryAllocator>, queue: Arc<Queue>, cmd_allocator: Arc<StandardCommandBufferAllocator>
        for modelsdat in modelsdatas {
            let model = Model::load(
                modelsdat.id,
                modelsdat.path, 
                memory_allocator.clone(), 
                queue.clone(), 
                Transform::push(
                    modelsdat.transform.p_xyz,
                    modelsdat.transform.r_xyz,
                    modelsdat.transform.s_xyz,
                ),
                cmd_allocator.clone(),
            );
            models.push(model);
        }
        modelsdatas: Vec<ModelData>
        */ 
        Self { 
            id, 
            models
        }
    }
}