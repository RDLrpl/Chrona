use glam::{Quat, Vec3};

pub struct Camera {
    pub orientation: Quat,

    pub vectors: Vectors,
    pub eye: Vec3,

    pub yaw: f32, 
    pub pitch: f32,

}

pub struct Vectors {
    pub look_vec: Vec3,
    pub up_vec: Vec3,
    pub side_vec: Vec3,
}

impl Vectors {
    pub fn rvlook(&self) -> Vec3 { -self.look_vec }
    pub fn rvup(&self) -> Vec3 { -self.up_vec }
    pub fn rvside(&self) -> Vec3 { -self.side_vec }
}

impl Camera {
    pub fn init(eye: Vec3) -> Self {
        let orientation = Quat::IDENTITY;

        Self {
            orientation,
            vectors: Vectors {
                look_vec: orientation * Vec3::new(0.0, 0.0, -1.0),

                up_vec: orientation * Vec3::new(0.0, 1.0, 0.0),

                side_vec: orientation * Vec3::new(1.0, 0.0, 0.0),
            },
            eye,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn crotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw;
        self.pitch += pitch;

        self.pitch = self.pitch.clamp(-1.5, 1.5);

        let q_yaw = Quat::from_axis_angle(Vec3::Y, self.yaw);
        let q_pitch = Quat::from_axis_angle(Vec3::X, self.pitch);

        self.orientation = q_yaw * q_pitch;
        self.update();
    }

    pub fn cmove(&mut self, direction: Vec3, to_move: f32) {
        self.eye += direction * to_move;
    }

    pub fn target(&self) -> Vec3 {
        self.eye + self.vectors.look_vec
    }

    fn update(&mut self) {
        self.vectors.look_vec = self.orientation * Vec3::new(0.0, 0.0, -1.0);

        self.vectors.up_vec = self.orientation * Vec3::new(0.0, 1.0, 0.0);

        self.vectors.side_vec = self.orientation * Vec3::new(1.0, 0.0, 0.0);
    }
}