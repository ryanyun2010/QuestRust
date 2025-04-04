#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::single_match)]
#![allow(clippy::unnecessary_get_then_check)]
use std::{env, time::Instant};
use crate::error::PError;
pub mod rendering_engine;
use rendering_engine::{renderer, texture, vertex, window};
pub mod game_engine;
use game_engine::{camera, json_parsing::{self, PATH_BUNDLE}, starting_level_generator, stat, ui::UIElementDescriptor, world};
use compact_str::CompactString;
pub mod tests;



pub mod error;


fn main() {
    ok_or_panic!(startup());
}



fn startup() -> Result<(), PError> {
    let mut current_dir = match env::current_exe() {
        Ok(val) => val,
        Err(e) => return Err(perror!(NotFound, "failed to find executable path because: {}", e))
    };
    while !current_dir.join("Cargo.toml").exists() {
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return Err(perror!("Cargo.toml not found. Is this a Rust project?"));
        }
    }
    match env::set_current_dir(&current_dir) {
        Ok(_) => (),
        Err(e) => return Err(perror!(NotFound, "failed to change working directory to project root because: {}", e))
    }
    println!("Changed working directory to project root: {:?}", current_dir);
    let mut parser = json_parsing::JSON_parser::new();
    let load_time = Instant::now();
    let parsed_data = parser.parse_and_convert_game_data(PATH_BUNDLE);
    let mut camera = camera::Camera::new(1152,720);
    let mut world = ptry!(starting_level_generator::generate_world_from_json_parsed_data(&parsed_data));
    let sword = world.inventory.add_item(
        ptry!(world.create_item_with_archetype(CompactString::from("basic sword")))
    );
    let spear = world.inventory.add_item(
        ptry!(world.create_item_with_archetype(CompactString::from("basic spear")))
    );
    world.inventory.init_ui();
    ptry!(world.inventory.set_hotbar_slot_item(3, spear));
    ptry!(world.inventory.set_hotbar_slot_item(0, sword));
    camera.level_text_id = Some(camera.add_text(
            "Lv. 0".into(),
            camera::Font::A,
            576.0,
            676.0,
            200.0,
            20.0,
            30.0,
            [1.0, 1.0, 1.0, 1.0],
            wgpu_text::glyph_brush::HorizontalAlign::Center
            ));
    camera.add_ui_element(CompactString::from("health_bar_background"), UIElementDescriptor {
        x: 32.0,
        y: 32.0,
        z: 5.0,
        width: 256.0,
        height: 32.0,
        sprite: CompactString::from("health_bar_back"),
        visible: true
    });
    camera.add_ui_element(CompactString::from("health_bar_inside"), UIElementDescriptor {
        x: 35.0,
        y: 35.0,
        z: 6.0,
        width: 250.0,
        height: 26.0,
        sprite: CompactString::from("health"),
        visible: true
    });
    camera.add_ui_element(CompactString::from("health_bar_background"), UIElementDescriptor {
        x: 832.0,
        y: 32.0,
        z: 5.0,
        width: 256.0,
        height: 32.0,
        sprite: CompactString::from("health_bar_back"),
        visible: true
    });
    camera.add_ui_element(CompactString::from("mana_bar_inside"), UIElementDescriptor {
        x: 838.0,
        y: 38.0,
        z: 6.0,
        width: 244.0,
        height: 20.0,
        sprite: CompactString::from("mana_bar_inner"),
        visible: true
    });


    camera.add_ui_element(CompactString::from("exp_bar_background"), UIElementDescriptor {
        x: 350.0,
        y: 695.0,
        z: 5.0,
        width: 452.0,
        height: 14.0,
        sprite: CompactString::from("level_editor_menu_background"),
        visible: true
    });
    camera.add_ui_element(CompactString::from("exp_bar_inside"), UIElementDescriptor {
        x: 353.0,
        y: 696.8,
        z: 6.0,
        width: 446.0,
        height: 10.0,
        sprite: CompactString::from("exp_bar_inner"),
        visible: true
    });

    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    pollster::block_on(window::run(world, camera, &parsed_data.sprites_to_load_json));
    Ok(())
}

