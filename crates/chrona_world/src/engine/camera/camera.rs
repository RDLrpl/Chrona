pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,

    pub forward: [f32; 3],
    pub eye: [f32; 3],

    pub radius: f32,
}

impl Camera {
    pub fn init(radius: f32) -> Self {
        let yaw: f32 = 0_f32.to_radians();
        let pitch: f32 = 0.0;
        let forward = [
            pitch.cos() * yaw.cos(), // X
            pitch.sin(), // Y
            pitch.cos() * yaw.sin(), // Z
        ];

        let eye: [f32; 3] = [radius * yaw.cos(), 0.0, radius * yaw.sin()];
        

        Self {
            yaw,
            pitch,
            forward,
            eye,
            radius
        }
    }


    pub fn vector_move(&mut self, w: f32) {
        self.eye[0] -= self.forward[0] * w;
        self.eye[1] -= self.forward[1] * w;
        self.eye[2] -= self.forward[2] * w;

        self.radius_update()
    }

    pub fn rotate(&mut self, xy: [f32; 2]) {
        self.pitch += xy[1];
        self.yaw   += xy[0];

        self.pitch = self.pitch.clamp(-1.5, 1.5);

        self.eye[0] = self.radius * self.pitch.cos() * self.yaw.cos();
        self.eye[1] = self.radius * self.pitch.sin();
        self.eye[2] = self.radius * self.pitch.cos() * self.yaw.sin();

        self.forward_update();
    }

    fn forward_update(&mut self) {
        self.forward = [
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        ];
    }

    fn radius_update(&mut self) {
       self.radius = (self.eye[0].powi(2) + self.eye[1].powi(2) + self.eye[2].powi(2)).sqrt();
    }

    pub fn target(&self) -> [f32; 3] {
        [
            0.0, 
            0.0, 
            0.0, 
        ]
    }
}