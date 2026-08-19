use vulkano::buffer::BufferContents;

pub const MAX_RENDER_LIGHTS: u32 = 4096;

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct GpuLight {
    pub position: [f32; 3],
    pub light_type: u32,

    pub color: [f32; 3],
    pub intensity: f32,
}

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct LightGrid {
    pub offset: u32,
    pub count: u32,
}