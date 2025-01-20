use crate::game_engine::camera::Camera;
use crate::game_engine::player;
use crate::game_engine::ui::UIElementDescriptor;
use crate::game_engine::world;
use crate::game_engine::world::World;

pub async fn basic_world() -> world::World {
    let mut sprites = crate::rendering_engine::abstractions::SpriteContainer::new();
    sprites.sprite_id_lookup.insert(String::from("test_sprite"), 0);
    sprites.sprites.push(crate::rendering_engine::abstractions::Sprite{
        tex_x: 0.0, tex_y: 0.0, 
        tex_w: 1.0, tex_h: 1.0,
        texture_index: 0
    }
    );
    sprites.sprite_id_lookup.insert(String::from("player_front"), 0);
    sprites.sprite_id_lookup.insert(String::from("player_right"), 0);
    sprites.sprite_id_lookup.insert(String::from("player_left"), 0);
    sprites.sprite_id_lookup.insert(String::from("player_back"), 0);
    sprites.sprite_id_lookup.insert(String::from("melee_attack"), 0);
    sprites.sprite_id_lookup.insert(String::from("attack_highlight"), 0);
    sprites.sprite_id_lookup.insert(String::from("sword"), 0);
    sprites.sprite_id_lookup.insert(String::from("spear"), 0);
    sprites.sprite_id_lookup.insert(String::from("slot_highlight"), 0);
    sprites.sprite_id_lookup.insert(String::from("hslot"), 0);
    let mut world = world::World::new(player::Player::new(596.0, 400.0, 10.0, 10, 1.0, 0),sprites);
    
    world

}
pub async fn basic_camera(world: &mut World) -> Camera {
    let mut camera = Camera::new(1152,720);
    world.inventory.init_ui(&mut camera, &world.sprites);
    camera
}
