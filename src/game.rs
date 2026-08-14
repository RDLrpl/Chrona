use std::{cell::RefCell, rc::Rc};

use chrona_api::{aev::aev::CHAPI, models::transform::get_object};
use chrona_world::engine::layout::world::world::World;
use winit::keyboard::KeyCode;

thread_local! {
    static VELOCITY_Y: RefCell<f32> = RefCell::new(0.0);
}

const GRAVITY: f32 = -9.8;
const JUMP_FORCE: f32 = 5.0;
const GROUND_Y: f32 = 0.0;

pub fn on_frame_update(world_link: Rc<RefCell<World>>, api: &CHAPI) {
    let kh = &api.keyboard_handler;
    let dt = api.delta_time;
    let mut ms = 2.0 * dt;

    let target_object = get_object(world_link, 1);

    if kh.modifiers.control_key() {
        ms *= 10.0
    }
    if kh.is_pressed(KeyCode::KeyD) {
        target_object.move_object([ms, 0.0, 0.0]);
    }
    if kh.is_pressed(KeyCode::KeyA) {
        target_object.move_object([-ms, 0.0, 0.0]);
    }
    if kh.is_pressed(KeyCode::KeyS) {
        target_object.move_object([0.0, 0.0, ms]);
    }
    if kh.is_pressed(KeyCode::KeyW) {
        target_object.move_object([0.0, 0.0, -ms]);
    }

    if kh.is_pressed(KeyCode::ArrowUp) {
        target_object.rotate_object([0.0, 0.0, (ms * 6.7)]);
    }
    if kh.is_pressed(KeyCode::ArrowDown) {
        target_object.rotate_object([0.0, 0.0, -(ms * 6.7)]);
    }
    if kh.is_pressed(KeyCode::ArrowLeft) {
        target_object.rotate_object([0.0, (ms * 6.7), 0.0]);
    }
    if kh.is_pressed(KeyCode::ArrowRight) {
        target_object.rotate_object([0.0, -(ms * 6.7), 0.0]);
    }

    let is_ground = target_object.position[1] <= GROUND_Y;

    VELOCITY_Y.with(|v| {
        let mut vel = v.borrow_mut();

        if is_ground {
            *vel = 0.0;

            if target_object.position[1] < GROUND_Y {
                target_object.move_object([0.0, GROUND_Y - target_object.position[1], 0.0]);
            }

            if kh.is_pressed(KeyCode::Space) {
                *vel = JUMP_FORCE;
            }
        } else {
            *vel += GRAVITY * dt;
        }

        let dy = *vel * dt;
        target_object.move_object([0.0, dy, 0.0]);
    });
}