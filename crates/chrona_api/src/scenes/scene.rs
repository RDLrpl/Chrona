use chrona_vk::engine::layout::world::world::World;

pub fn scene_change(world: &mut World, target: u32) {
    world.curscene = target
}