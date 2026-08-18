use std::{cell::RefCell, rc::Rc};

use chrona_api::{aev::aev::CHAPI, models::transform::get_object};
use chrona_world::engine::{camera::camera::Camera, layout::world::world::World};
use winit::keyboard::KeyCode;

pub fn on_frame_update(world_link: Rc<RefCell<World>>, api: &CHAPI, camera: &mut Camera) {
    let dt = api.delta_time;
    let mut ms = 2.5 * dt;
    let mut sense = 0.0008;
    
    let target = get_object(world_link, 1);

    target.rotate_object([1.2 * dt, 1.4 * dt, 0.0]);
    
    if api.keyboard_handler.modifiers.shift_key() {
        ms *= 2.5;
    }

    if api.keyboard_handler.is_pressed(KeyCode::KeyW) {
        camera.cmove(camera.vectors.look_vec, ms);
    }
    if api.keyboard_handler.is_pressed(KeyCode::KeyS) {
        camera.cmove(camera.vectors.rvlook(), ms);
    }
    if api.keyboard_handler.is_pressed(KeyCode::KeyD) {
        camera.cmove(camera.vectors.side_vec, ms);
    }
    if api.keyboard_handler.is_pressed(KeyCode::KeyA) {
        camera.cmove(camera.vectors.rvside(), ms);
    }

    if api.keyboard_handler.modifiers.control_key() {
        sense *= 0.6;
    }
    
    camera.crotate(-(api.mouse_handler.delta_x * sense), -(api.mouse_handler.delta_y * sense));
}