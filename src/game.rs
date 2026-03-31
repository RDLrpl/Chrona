use chrona_api::models::transform::rotate_object;
use chrona_app::app::App;

pub fn on_update(app: &mut App) {
    let world= app.world.as_mut().unwrap();

    let (x, y, z) = (0.01 as f32, 0.0 as f32, 0.0 as f32);

    rotate_object(world, 1, [x, y, z]);
}