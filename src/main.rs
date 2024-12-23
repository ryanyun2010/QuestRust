#![allow(warnings)]
use core::arch;
use std::time::Instant;
use image::load;
use rand::prelude::*;
pub mod rendering_engine;
use rendering_engine::window;
use rendering_engine::state;
use rendering_engine::vertex;
use rendering_engine::texture;
use rendering_engine::abstractions;
pub mod game_engine;
use game_engine::world;
use game_engine::camera;
use game_engine::loot;
use game_engine::entities;
use game_engine::stat;
use game_engine::terrain;
use game_engine::magic;
use game_engine::entities::EntityTags;
use game_engine::ui::UIElement;
use game_engine::player::Player;
use game_engine::json_parsing;
use game_engine::starting_level_generator::generate_world_from_json_parsed_data;
use game_engine::pathfinding;
use wgpu::naga::back::Level;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("level_editor")){
        // Do level editor stuff
    }
    let mut parser = json_parsing::JSON_parser::new();
    let load_time = Instant::now();
    let parsed_data = parser.parse_and_convert_game_data("src/game_data/entity_archetypes.json", "src/game_data/entity_attack_patterns.json", "src/game_data/entity_attacks.json", "src/game_data/sprites.json", "src/game_data/starting_level.json");
    
    let mut camera = camera::Camera::new(1152,720);
    camera.add_ui_element(String::from("health_bar_background"), UIElement {
        x: 32.0,
        y: 32.0,
        width: 256.0,
        height: 32.0,
        texture_id: parsed_data.get_texture_id("health_bar_back"),
        visible: true
    });
    camera.add_ui_element(String::from("health_bar_inside"), UIElement {
        x: 35.0,
        y: 35.0,
        width: 250.0,
        height: 26.0,
        texture_id: parsed_data.get_texture_id("health"),
        visible: true
    });
    camera.add_ui_element(String::from("inventory_button"), UIElement {
        x: 1030.0,
        y: 650.0,
        width: 75.0,
        height: 25.0,
        texture_id: parsed_data.get_texture_id("inventory"),
        visible: true
    });

    let (mut world, mut sprites) = generate_world_from_json_parsed_data(&parsed_data);

    world.player.borrow_mut().holding_texture_sprite = Some(sprites.get_sprite("sword"));
    for n in 0..17 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,sprites.get_sprite("outside"));
        }
    }

    for n in 17..35 {
        for m in 0..11 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,sprites.get_sprite("outside"));
        }
    }

    for n in 18..35 {
        let new_terrain = world.add_terrain(n*32,352);
        world.set_sprite(new_terrain,sprites.get_sprite("wall2"));
        world.add_terrain_tag(new_terrain, terrain::TerrainTags::BlocksMovement);
    }
        let new_terrain = world.add_terrain(544,352);
        world.set_sprite(new_terrain,sprites.get_sprite("wall3"));


    for m in 12..70 {
        let new_terrain = world.add_terrain(544,m*32);
        world.set_sprite(new_terrain,sprites.get_sprite("wall"));
        world.add_terrain_tag(new_terrain, terrain::TerrainTags::BlocksMovement);
    }
    for n in 18..35 {
        for m in 12..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            let x: u8 = random();
            if x > 150{
            world.set_sprite(new_terrain,sprites.get_sprite("dirt"));
            } else{
                world.set_sprite(new_terrain,sprites.get_sprite("dirt2")); 
            }
        }
    }

    println!("Time to load: {:?} ms", load_time.elapsed().as_millis());
    pollster::block_on(window::run(&mut world, &mut camera, parsed_data.sprites_to_load_json));
}