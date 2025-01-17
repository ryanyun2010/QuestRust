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
pub async fn basic_camera(world: &World) -> Camera {
    let mut camera = Camera::new(1152,720);

    camera.add_ui_element(String::from("hslot1"), UIElementDescriptor {
        x: 20.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("hslot2"), UIElementDescriptor {
        x: 78.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("hslot3"), UIElementDescriptor {
        x: 136.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("hslot4"), UIElementDescriptor {
        x: 194.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("hslot5"), UIElementDescriptor {
        x: 252.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("hhslot"), UIElementDescriptor {
        x: 20.0,
        y: 652.0,
        width: 48.0,
        height: 48.0,
        sprite_id: world.sprites.get_sprite_id("slot_highlight").expect("couldn't find hotbar highlight sprite"),
        visible: true
    });


    camera.add_ui_element(String::from("tempitem"), UIElementDescriptor {
        x: 28.0,
        y: 660.0,
        width: 32.0,
        height: 32.0,
        sprite_id: world.sprites.get_sprite_id("sword").expect("couldn't find hotbar sprite"),
        visible: true
    });


    camera.add_ui_element(String::from("tempitem2"), UIElementDescriptor {
        x: 86.0,
        y: 660.0,
        width: 32.0,
        height: 32.0,
        sprite_id: world.sprites.get_sprite_id("spear").expect("couldn't find hotbar sprite"),
        visible: true
    });

    camera
}
