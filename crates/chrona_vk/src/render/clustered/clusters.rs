use vulkano::buffer::BufferContents;

const PX_TILE: u32 = 64;
const Z_TILE_SLICES: u32 = 32;
pub const MAX_CLUSTER_LIGHTS: u32 = 256;

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct ClusterAABB {
    pub min: [f32; 4],
    pub max: [f32; 4],
}

pub struct ClusterResource {
    pub grid_dims: [u32; 3]
}

impl ClusterResource {
    pub fn compute(screen_extent: [u32; 2]) -> Self {
        Self { grid_dims: [
            (screen_extent[0] + PX_TILE - 1) / PX_TILE,
            (screen_extent[1] + PX_TILE - 1) / PX_TILE,
            Z_TILE_SLICES
        ] }
    }
}