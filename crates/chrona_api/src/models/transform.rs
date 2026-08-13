use chrona_world::engine::layout::world::world::World;


pub fn rotate_object(world: &mut World, target: u32, rotate: [f32; 3]) {
    let curscene = world.return_mutcur_scene().unwrap();
    let object = curscene.return_object(target).unwrap();

    object.transf.rotation[0] += rotate[0];
    object.transf.rotation[1] += rotate[1];
    object.transf.rotation[2] += rotate[2];
}

pub fn move_object(world: &mut World, target: u32, mv: [f32; 3]) {
    let curscene = world.return_mutcur_scene().unwrap();
    let object = curscene.return_object(target).unwrap();

    object.transf.position[0] += mv[0];
    object.transf.position[1] += mv[1];
    object.transf.position[2] += mv[2];
}

pub fn scale_object(world: &mut World, target: u32, scale: [f32; 3]) {
    let curscene = world.return_mutcur_scene().unwrap();
    let object = curscene.return_object(target).unwrap();

    object.transf.scale[0] += scale[0];
    object.transf.scale[1] += scale[1];
    object.transf.scale[2] += scale[2];
}