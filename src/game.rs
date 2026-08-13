use chrona_api::models::transform::rotate_object;
use chrona_world::engine::layout::world::world::World;

pub fn on_frame_update(world: &mut World) {
    let (x, y, z) = (0.01 as f32, 0.0 as f32, 0.0 as f32);

    rotate_object(world, 1, [x, y, z]); 
}