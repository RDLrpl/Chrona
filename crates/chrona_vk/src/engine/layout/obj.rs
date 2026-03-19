use std::sync::Arc;

use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}};

use crate::vkinit::pipeline::VertexDat;


pub struct Model {
    pub name: String,
    pub vertdat: Vec<VertexDat>,

    pub vertex_buffer: Subbuffer<[VertexDat]>
}

impl Model {
    pub fn load(path: String, memory_allocator: Arc<StandardMemoryAllocator>) -> Self {
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
                    position: [
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

            vertdat: vertices,
            vertex_buffer,
        }
    }
}