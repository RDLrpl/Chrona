
// World
#[derive(Clone)]
pub struct WorldData {
    pub scenesdata: Vec<SceneData>
}

#[derive(Clone)]
pub struct SceneData {
    pub id: u32,
    pub modelsdata: Vec<ModelData> 
}

// Model
#[derive(Clone)]
pub struct TransformDat {
    pub p_xyz: [f32; 3],
    pub r_xyz: [f32; 3],
    pub s_xyz: [f32; 3],
}

#[derive(Clone)]
pub struct ModelData {
    pub id: u32,
    pub path: String,
    pub transform: TransformDat
}

impl WorldData {
    pub fn init(scenesdata: Vec<SceneData>) -> Self {
        Self {
            scenesdata
        }
    } 
}

impl SceneData { 
    pub fn init(id: u32, modelsdata: Vec<ModelData>) -> Self {
        Self { 
            id, 
            modelsdata 
        }
    }
}

impl ModelData {
    pub fn init(id: u32, path: String, p_xyz: [f32; 3], r_xyz: [f32; 3], s_xyz: [f32; 3]) -> Self {
        Self {
            id,
            path,
            transform: TransformDat {
                p_xyz,
                r_xyz,
                s_xyz
            }
        }
    }
}
