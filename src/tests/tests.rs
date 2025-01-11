use crate::game_engine::camera::Camera;
use crate::game_engine::player;
use crate::game_engine::world;

pub async fn basic_world() -> world::World {
    let mut world = world::World::new(player::Player::new(596.0, 400.0, 10.0, 10, 1.0, 0), crate::rendering_engine::abstractions::SpriteContainer::new());
    world.sprites.sprite_id_lookup.insert(String::from("test_sprite"), 0);
    world.sprites.sprites.push(crate::rendering_engine::abstractions::Sprite{
        tex_x: 0.0, tex_y: 0.0, 
        tex_w: 1.0, tex_h: 1.0,
        texture_index: 0
    }
    );
    world.sprites.sprite_id_lookup.insert(String::from("player_front"), 0);
    world.sprites.sprite_id_lookup.insert(String::from("player_right"), 0);
    world.sprites.sprite_id_lookup.insert(String::from("player_left"), 0);
    world.sprites.sprite_id_lookup.insert(String::from("player_back"), 0);
    world.sprites.sprite_id_lookup.insert(String::from("melee_attack"), 0);
    world
}
pub async fn basic_camera() -> Camera {
    Camera::new(1152,720)
}
