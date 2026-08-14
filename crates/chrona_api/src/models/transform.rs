use std::{cell::RefCell, rc::Rc};

use chrona_world::engine::layout::world::world::World;

pub struct CHObject {
    pub id: u32,
    pub name: String,

    pub scale: [f32; 3],
    pub rotation: [f32; 3],
    pub position: [f32; 3],

    pub world_link: Rc<RefCell<World>>, 
}

impl CHObject {
    pub fn rotate_object(&self, rotate: [f32; 3]) {
        let mut world = self.world_link.borrow_mut();
        let curscene = world.return_mutcur_scene().unwrap();
        let object = curscene.return_object(self.id).unwrap();

        object.transf.rotation[0] += rotate[0];
        object.transf.rotation[1] += rotate[1];
        object.transf.rotation[2] += rotate[2];
    }

    pub fn move_object(&self, mv: [f32; 3]) {
        let mut world = self.world_link.borrow_mut();
        let curscene = world.return_mutcur_scene().unwrap();
        let object = curscene.return_object(self.id).unwrap();

        object.transf.position[0] += mv[0];
        object.transf.position[1] += mv[1];
        object.transf.position[2] += mv[2];
    }

    pub fn scale_object(&self, scale: [f32; 3]) {
        let mut world = self.world_link.borrow_mut();
        let curscene = world.return_mutcur_scene().unwrap();
        let object = curscene.return_object(self.id).unwrap();

        object.transf.scale[0] += scale[0];
        object.transf.scale[1] += scale[1];
        object.transf.scale[2] += scale[2];
    }

    // temp. solution
    pub fn update_transform(&mut self) {
        let mut world = self.world_link.borrow_mut();
        let curscene = world.return_mutcur_scene().unwrap();
        let object = curscene.return_object(self.id).unwrap();

        object.transf.position = self.position;
        object.transf.scale = self.scale;
        object.transf.rotation = self.rotation;
    }
}


pub fn get_object(world_link: Rc<RefCell<World>>, target: u32) -> CHObject {
    let mut world = world_link.borrow_mut();
    let curscene = world.return_mutcur_scene().unwrap();
    let object = curscene.return_object(target).unwrap();

    CHObject { 
        id: target, 
        name: object.name.clone(), 
        scale: object.transf.scale, 
        rotation: object.transf.rotation, 
        position: object.transf.position,
        world_link: Rc::clone(&world_link),
    }
}

