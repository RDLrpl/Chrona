#[derive(Clone)]
pub struct TransformDat {
    pub p_xyz: [f32; 3],
    pub r_xyz: [f32; 3],
    pub s_xyz: [f32; 3],
}

#[derive(Clone)]
pub struct ModelData {
    pub path: String,
    pub transform: TransformDat
}

impl ModelData {
    pub fn init(path: String, p_xyz: [f32; 3], r_xyz: [f32; 3], s_xyz: [f32; 3]) -> Self {

        Self {
            path,
            transform: TransformDat {
                p_xyz,
                r_xyz,
                s_xyz
            }
        }
    }
}