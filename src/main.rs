#![allow(warnings)]
use std::time::Instant;
pub mod rendering_engine;
use rendering_engine::{renderer, sprite_sheet_generation_abstraction::combine_images, texture, vertex, window};
pub mod game_engine;
use game_engine::{camera, entities, json_parsing::{self, PATH_BUNDLE}, loot, starting_level_generator, stat, ui::UIElementDescriptor, world};
pub mod tests;
use std::env;
use image::{GenericImageView, RgbaImage, Rgba};

fn main() {
    let mut parser = json_parsing::JSON_parser::new();
    let load_time = Instant::now();
    let parsed_data = parser.parse_and_convert_game_data(PATH_BUNDLE);
    let mut camera = camera::Camera::new(1152,720);
    let mut world = starting_level_generator::generate_world_from_json_parsed_data(&parsed_data);
    let sword = world.inventory.add_item(
        world.create_item_with_archetype("test_sword".to_string())
    );
    let spear = world.inventory.add_item(
        world.create_item_with_archetype("test_spear".to_string())
    );
    world.inventory.init_ui(&world.sprites);
    world.inventory.set_hotbar_slot_item(0, sword);
    world.inventory.set_hotbar_slot_item(1, spear);
    camera.add_ui_element(String::from("health_bar_background"), UIElementDescriptor {
        x: 32.0,
        y: 32.0,
        z: 5.0,
        width: 256.0,
        height: 32.0,
        sprite: String::from("health_bar_back"),
        visible: true
    });
    camera.add_ui_element(String::from("health_bar_inside"), UIElementDescriptor {
        x: 35.0,
        y: 35.0,
        z: 6.0,
        width: 250.0,
        height: 26.0,
        sprite: String::from("health"),
        visible: true
    });
    // camera.add_ui_element(String::from("inventory_background"), UIElementDescriptor {
    //     x: 0.0,
    //     y: 0.0,
    //     width: 1152.0,
    //     height: 720.0,
    //     sprite_id: world.sprites.get_sprite_id("inventory_background").expect("couldn't find inventory_background sprite"),
    //     visible: true
    // });
    // camera.add_ui_element(String::from("inventory"), UIElementDescriptor {
    //     x: 326.0,
    //     y: 186.5,
    //     width: 500.0,
    //     height: 347.0,
    //     sprite_id: world.sprites.get_sprite_id("inventory").expect("couldn't find inventory sprite"),
    //     visible: true
    // });

    // camera.add_ui_element(String::from("hslot1"), UIElementDescriptor {
    //     x: 20.0,
    //     y: 652.0,
    //     width: 48.0,
    //     height: 48.0,
    //     sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });
    // camera.add_ui_element(String::from("hslot2"), UIElementDescriptor {
    //     x: 78.0,
    //     y: 652.0,
    //     width: 48.0,
    //     height: 48.0,
    //     sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });
    // camera.add_ui_element(String::from("hslot3"), UIElementDescriptor {
    //     x: 136.0,
    //     y: 652.0,
    //     width: 48.0,
    //     height: 48.0,
    //     sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });
    // camera.add_ui_element(String::from("hslot4"), UIElementDescriptor {
    //     x: 194.0,
    //     y: 652.0,
    //     width: 48.0,
    //     height: 48.0,
    //     sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });
    // camera.add_ui_element(String::from("hslot5"), UIElementDescriptor {
    //     x: 252.0,
    //     y: 652.0,
    //     width: 48.0,
    //     height: 48.0,
    //     sprite_id: world.sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });
   


    // camera.add_ui_element(String::from("tempitem"), UIElementDescriptor {
    //     x: 28.0,
    //     y: 660.0,
    //     width: 32.0,
    //     height: 32.0,
    //     sprite_id: world.sprites.get_sprite_id("sword").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });


    // camera.add_ui_element(String::from("tempitem2"), UIElementDescriptor {
    //     x: 86.0,
    //     y: 660.0,
    //     width: 32.0,
    //     height: 32.0,
    //     sprite_id: world.sprites.get_sprite_id("spear").expect("couldn't find hotbar sprite"),
    //     visible: true
    // });


    // world.player.borrow_mut().holding_texture_sprite = Some(world.sprites.get_sprite_id("sword").unwrap());
    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    pollster::block_on(window::run(world, camera, &parsed_data.sprites_to_load_json));

}
