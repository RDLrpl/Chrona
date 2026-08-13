// eab - engine api bridge

use chrona_utils::data::WorldData;
use chrona_world::engine::layout::world::world::World;

pub struct GameFunc {
    pub on_frame_update: fn(&mut World),
}

impl GameFunc {
    pub fn load(on_frame_update: fn(&mut World)) -> Self {
        Self {
            on_frame_update
        }
    }
}

pub struct GameData {
    pub world: WorldData,
    pub gamefuncs: GameFunc
}

impl GameData {
    pub fn load(world: WorldData, gamefuncs: GameFunc) -> Self{
        Self {
            world,
            gamefuncs
        }
    }
}