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
use std::{env, path::Path, time::Instant};
pub mod rendering_engine;
use rendering_engine::{renderer, texture, vertex, window};
pub mod game_engine;
use game_engine::{camera, entities, json_parsing::{self, PATH_BUNDLE}, starting_level_generator, stat, ui::UIElementDescriptor, world};
pub mod tests;
pub mod error;


fn main() {
    let username = whoami::username();

    let mut p = format!("/Users/{}/Desktop/QuestRust/",username);
    let args = env::args();
    for arg in args.into_iter() {
        println!("{}", arg);
        if arg.starts_with("path=") {
            let mut d = arg.clone();
            for i in 0..5 {d.remove(0);};
            if d.chars().next().expect("invalid path") == '~' {
                d.remove(0);
                p = format!("/Users/{}{}", username, d);
            }
        }
    }
    let path = Path::new(&p);
    assert!(env::set_current_dir(path).is_ok(), "QuestRust directory not found at path: {}, pass the path of QuestRust with cargo run -- path=<INSERT PATH HERE> or run the executable with -- path=<INSERT PATH HERE>", path.display());
    println!("Successfully changed working directory to {}!", path.display());
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
    world.inventory.init_ui();
    match world.inventory.set_hotbar_slot_item(0, sword) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e)
    };
    match world.inventory.set_hotbar_slot_item(1, spear) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e)
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

