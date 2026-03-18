use crate::vkinit::pipeline::VertexDat;


pub struct Model {
    pub name: String,
    pub vertdat: Vec<VertexDat>
}

impl Model {
    pub fn load(path: String) -> Self {
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
                    color: [0.0, 1.0, 1.0],
                });
            }
        }

        Self { 
            name: model_name.to_string(),
            vertdat: vertices
        }
    }
}