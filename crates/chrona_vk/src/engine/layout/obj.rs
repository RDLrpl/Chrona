use std::sync::Arc;

use glam::{EulerRot, Mat4, Quat, Vec3};
use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer, allocator::SubbufferAllocator}, descriptor_set::allocator::DescriptorSetAllocator, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}};

use crate::{engine::shr::CameraUBO, vkinit::pipeline::VertexDat};

pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3]
} 

pub struct Model {
    pub name: String,
    pub vertdat: Vec<VertexDat>,
    pub transf: Transform,

    pub vertex_buffer: Subbuffer<[VertexDat]>
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl Transform {
    pub fn push(position: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) -> Self{

        Self {
            position,
            rotation,
            scale
        }
    }

    pub fn to_model_matrix(&self) -> Mat4 {
        let t = Vec3::from_slice(&self.position);
        let s = Vec3::from_slice(&self.scale);

        let r = Quat::from_euler(
            EulerRot::XYZ, 
            self.rotation[0], 
            self.rotation[1], 
            self.rotation[2]
        );

        Mat4::from_scale_rotation_translation(s, r, t)
    }
}

impl Model {
    pub fn load(path: String, memory_allocator: Arc<StandardMemoryAllocator>, transform: Transform) -> Self { //, startpos: [f32; 3]
        let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load OBJ file");

        let mut vertices = Vec::new();
        let mut model_name = "";

        for m in models.iter() {
            model_name = &m.name;
            
            let mesh = &m.mesh;

            assert!(mesh.positions.len() % 3 == 0);

            for i in mesh.indices.iter() {
                let v = *i as usize;
                vertices.push(VertexDat {
                    vecposition: [
                        mesh.positions[3 * v],
                        mesh.positions[3 * v + 1],
                        mesh.positions[3 * v + 2],
                    ],
                    uv: [0.0, 0.0],
                    color: [0.341, 0.380, 0.357],
                });
            }
            
        
        }

        let vertex_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices.clone(),
        )
        .unwrap();

        Self { 
            name: model_name.to_string(),
            transf: transform,

            vertdat: vertices,
            vertex_buffer,
        }
    }
}