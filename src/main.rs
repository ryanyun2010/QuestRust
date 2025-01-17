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
    let world = starting_level_generator::generate_world_from_json_parsed_data(&parsed_data);
    camera.add_ui_element(String::from("health_bar_background"), UIElementDescriptor {
        x: 32.0,
        y: 32.0,
        width: 256.0,
        height: 32.0,
        sprite_id: world.sprites.get_sprite_id("health_bar_back").expect("couldn't find health_bar_back sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("health_bar_inside"), UIElementDescriptor {
        x: 35.0,
        y: 35.0,
        width: 250.0,
        height: 26.0,
        sprite_id: world.sprites.get_sprite_id("health").expect("couldn't find health sprite"),
        visible: true
    });
    camera.add_ui_element(String::from("inventory_button"), UIElementDescriptor {
        x: 1030.0,
        y: 650.0,
        width: 75.0,
        height: 25.0,
        sprite_id: world.sprites.get_sprite_id("inventory").expect("couldn't find inventory sprite"),
        visible: true
    });

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

    // world.player.borrow_mut().holding_texture_sprite = Some(world.sprites.get_sprite_id("sword").unwrap());
    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    pollster::block_on(window::run(world, camera, &parsed_data.sprites_to_load_json));

}
