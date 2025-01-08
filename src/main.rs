#![allow(warnings)]
use std::time::Instant;
pub mod rendering_engine;
use rendering_engine::{window, renderer, vertex, texture};
pub mod game_engine;
use game_engine::{camera, entities, entity_components, game, json_parsing::{self, PATH_BUNDLE}, level_editor, loot, starting_level_generator, stat, ui::{UIElement, UIElementDescriptor}, world};
pub mod tests;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("level-editor")){
        let mut parser = json_parsing::JSON_parser::new();
        let parsed_data = parser.parse_and_convert_game_data(PATH_BUNDLE);
        let mut camera = camera::Camera::new(1152,720);
        let (world, sprites, hash) = level_editor::level_editor_generate_world_from_json_parsed_data(&parsed_data);
        camera.set_level_editor();
        pollster::block_on(level_editor::run(world, sprites, parser, hash, camera, parsed_data.sprites_to_load_json));
        return;
    }
    
    let mut parser = json_parsing::JSON_parser::new();
    let load_time = Instant::now();
    let parsed_data = parser.parse_and_convert_game_data(PATH_BUNDLE);
    
    let mut camera = camera::Camera::new(1152,720);
    let (mut world, sprites) = starting_level_generator::generate_world_from_json_parsed_data(&parsed_data);
    camera.add_ui_element(String::from("health_bar_background"), UIElementDescriptor {
        x: 32.0,
        y: 32.0,
        width: 256.0,
        height: 32.0,
        texture_id: parsed_data.get_texture_id("health_bar_back"),
        visible: true
    });
    camera.add_ui_element(String::from("health_bar_inside"), UIElementDescriptor {
        x: 35.0,
        y: 35.0,
        width: 250.0,
        height: 26.0,
        texture_id: parsed_data.get_texture_id("health"),
        visible: true
    });
    camera.add_ui_element(String::from("inventory_button"), UIElementDescriptor {
        x: 1030.0,
        y: 650.0,
        width: 75.0,
        height: 25.0,
        texture_id: parsed_data.get_texture_id("inventory"),
        visible: true
    });

    world.player.borrow_mut().holding_texture_sprite = Some(sprites.get_sprite("sword").unwrap());
    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    
    pollster::block_on(window::run(world, camera, sprites, parsed_data.sprites_to_load_json));
}