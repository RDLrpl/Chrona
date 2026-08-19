use vulkano::{buffer::BufferContents, device::DeviceExtensions, pipeline::graphics::vertex_input::Vertex};

// minimal units
#[derive(BufferContents, Vertex, Clone)]
#[repr(C)]
pub struct VertexDat {
    #[format(R32G32B32_SFLOAT)]
    pub vecposition: [f32; 3],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}


// Engine
#[derive(Clone)]
pub struct AppConfiguration {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub app_version: [u32; 3],
    pub projname: String,

    pub device_extensions: DeviceExtensions,

    pub max_framerate: usize,
    pub vsync: bool,
}

impl AppConfiguration {
    pub fn new(
        projname: &str, 
        app_version: [u32; 3], 
        width: u32, 
        height: u32, 
        fullscreen: bool, 
        device_extensions: DeviceExtensions,
        max_framerate: usize,
        vsync: bool
    ) -> Self {
        Self {
            projname: projname.to_string(),
            app_version,
            width,
            height,
            fullscreen,
            device_extensions,
            max_framerate,
            vsync
        }
    }
}

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
    
    pub transform: TransformDat,
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
