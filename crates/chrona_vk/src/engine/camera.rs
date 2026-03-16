use vulkano::buffer::BufferContents;

#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
pub struct CameraUBO {
    pub model: [[f32; 4]; 4],
    pub view:  [[f32; 4]; 4],
    pub proj:  [[f32; 4]; 4],
}
