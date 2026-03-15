use vulkano::buffer::BufferContents;

#[derive(BufferContents, Clone, Copy)]
#[repr(C)]
pub struct CameraUBO {
    model: [[f32; 4]; 4],
    view:  [[f32; 4]; 4],
    proj:  [[f32; 4]; 4],
}
