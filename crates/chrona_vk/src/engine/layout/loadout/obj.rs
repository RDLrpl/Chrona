use std::{path::Path, sync::Arc};

use chrona_utils::binding::OptionExt;
use glam::{EulerRot, Mat4, Quat, Vec3};
use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}, command_buffer::allocator::StandardCommandBufferAllocator, device::Queue, image::{view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}};

use crate::{engine::layout::loadout::image::{no_texture, upload_texture}, vkinit::pipeline::VertexDat};

pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3]
} 

pub struct Model {
    pub name: String,
    pub vertdat: Vec<VertexDat>,
    pub transf: Transform,

    pub vertex_buffer: Subbuffer<[VertexDat]>,
    pub texture: Arc<ImageView>,
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
    pub fn load(
        path: String, 
        memory_allocator: Arc<StandardMemoryAllocator>,
        queue: Arc<Queue>, 
        transform: Transform,
        cmd_allocator: Arc<StandardCommandBufferAllocator>
    ) -> Self {
        let (models, materials) = tobj::load_obj(path.clone(), &tobj::GPU_LOAD_OPTIONS).expect("OBJ (No Exist?) file ERR");
        let mats = materials.unwrap_or_default();
        
        let mut vertices = Vec::new();
        let mut model_name = "";
        
        let mut texture = no_texture(memory_allocator.clone(), queue.clone(), cmd_allocator.clone());

        for m in models.iter() {
            model_name = &m.name;
            
            let mesh = &m.mesh;

            let img_data;
            let (width, height);


            assert!(mesh.positions.len() % 3 == 0);

            let current_mat = mesh.material_id.and_then(|id| mats.get(id));

            let mesh_color = current_mat
                .and_then(|mat| mat.diffuse)
                .unwrap_or([1.0, 1.0, 1.0]);

            if let Some(mat) = current_mat {
                if let Some(ref tex_name) = mat.diffuse_texture {
                    let flpath = Path::new(&path).parent().expect_me("[CHRONA]: .MTL Path'Err").join(tex_name);

                    img_data = Some(image::open(flpath).expect("Failed to load image").to_rgba8());

                    let (w, h) = img_data.clone().unwrap().dimensions();
                    (width, height) = (Some(w), Some(h));

                    texture = upload_texture(
                        memory_allocator.clone(), 
                        queue.clone(),
                        &img_data.unwrap(), 
                        width.unwrap(), height.unwrap(), 
                        cmd_allocator.clone()
                    );

                }
            }

            for i in mesh.indices.iter() {
                let v = *i as usize;

                let uv = if !mesh.texcoords.is_empty() {
                    [mesh.texcoords[2 * v], 1.0 - mesh.texcoords[2 * v + 1]]
                } else {
                    [0.0, 0.0]
                };

                vertices.push(VertexDat {
                    vecposition: [
                        mesh.positions[3 * v],
                        mesh.positions[3 * v + 1],
                        mesh.positions[3 * v + 2],
                    ],
                    uv,
                    color: mesh_color,
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
            texture,
        }
    }
}