use crate::game_engine::camera::Camera;
use crate::game_engine::player;
use crate::game_engine::world;

pub async fn basic_world() -> world::World {
    let mut world = world::World::new(player::Player::new(596.0, 400.0, 10.0, 10, 1.0, 0));
    world.add_sprite(0);
    world
}
pub async fn basic_camera() -> Camera {
    Camera::new(1152,720)
}
