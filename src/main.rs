#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::single_match)]
#![allow(clippy::unnecessary_get_then_check)]
use std::{env, path::PathBuf, time::Instant};
pub mod rendering_engine;
use rendering_engine::{renderer, texture, vertex, window};
pub mod game_engine;
use game_engine::{camera, entities, json_parsing::{self, PATH_BUNDLE}, starting_level_generator, stat, ui::UIElementDescriptor, world};
pub mod tests;
pub mod error;


fn main() {
    let args = env::args();
    let mut p = None;
    for arg in args.into_iter() {
       p = Some(arg); 
    }
    let mut path = PathBuf::from(&p.unwrap());
    path.pop();
    path.pop();
    path.pop();
    if path.parent().is_some() {
        assert!(env::set_current_dir(path.clone()).is_ok(), "QuestRust directory not found at path: {}", path.display());
        println!("Successfully changed working directory to {}!", path.display());
    }
    let mut parser = json_parsing::JSON_parser::new();
    let load_time = Instant::now();
    let parsed_data = parser.parse_and_convert_game_data(PATH_BUNDLE);
    let mut camera = camera::Camera::new(1152,720);
    let mut world = starting_level_generator::generate_world_from_json_parsed_data(&parsed_data);
    let sword = world.inventory.add_item(
        ok_or_panic!(world.create_item_with_archetype("funny spear".to_string()))
    );
    let spear = world.inventory.add_item(
        ok_or_panic!(world.create_item_with_archetype("test_spear".to_string()))
    );
    let h = world.inventory.add_item(
        ok_or_panic!(world.create_item_with_archetype("test helm".to_string()))
    );
    world.inventory.init_ui();
    match world.inventory.set_hotbar_slot_item(0, sword) {
        Ok(_) => {},
        Err(e) => print_error!(e)
    };
    match world.inventory.set_hotbar_slot_item(1, spear) {
        Ok(_) => {},
        Err(e) => print_error!(e)
    }
    match world.inventory.set_slot_item(7, h) {
        Ok(_) => {},
        Err(e) => print_error!(e)
    }

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
    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    pollster::block_on(window::run(world, camera, &parsed_data.sprites_to_load_json));

}

