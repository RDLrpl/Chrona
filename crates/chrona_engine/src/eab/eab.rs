// eab - engine api bridge

use std::{cell::RefCell, rc::Rc};

use chrona_api::aev::aev::CHAPI;
use chrona_utils::data::WorldData;
use chrona_world::engine::layout::world::world::World;

pub struct GameFunc {
    pub on_frame_update: fn(world_link: Rc<RefCell<World>>, &CHAPI),
}

impl GameFunc {
    pub fn load(on_frame_update: fn(Rc<RefCell<World>>, &CHAPI)) -> Self {
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