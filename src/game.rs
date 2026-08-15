use std::{cell::RefCell, rc::Rc};

use chrona_api::{aev::aev::CHAPI};
use chrona_world::engine::{camera::camera::Camera, layout::world::world::World};

pub fn on_frame_update(_world_link: Rc<RefCell<World>>, api: &CHAPI, camera: &mut Camera) {
    let dt = api.delta_time;
    let ms = 0.5 * dt;

    camera.rotate([-ms, 0.0]);
}