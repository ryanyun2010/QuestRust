#![allow(warnings)]
use core::arch;
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

fn main() {
    let mut world = world::World::new(); // 36 x 22.5 blocks
    let mut camera = camera::Camera::new(1152,720);
    camera.add_ui_element(String::from("health_bar_background"), UIElement {
        x: 32.0,
        y: 32.0,
        width: 256.0,
        height: 32.0,
        texture_id: 9,
        visible: true
    });
    camera.add_ui_element(String::from("health_bar_inside"), UIElement {
        x: 35.0,
        y: 35.0,
        width: 250.0,
        height: 26.0,
        texture_id: 10,
        visible: true
    });
    camera.add_ui_element(String::from("inventory_button"), UIElement {
        x: 1030.0,
        y: 650.0,
        width: 75.0,
        height: 25.0,
        texture_id: 11,
        visible: true
    });

    
    let outside_sprite = world.add_sprite(6);
    let wall2_sprite = world.add_sprite(13);
    let wall3_sprite = world.add_sprite(14);
    let dirt_sprite = world.add_sprite(5);
    let dirt2_sprite = world.add_sprite(4);
    let wall_sprite = world.add_sprite(7);
    let ghost_sprite = world.add_sprite(8);
    let d = world.add_sprite(12);
    for n in 0..17 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,outside_sprite);
        }
    }

    for n in 17..35 {
        for m in 0..11 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,outside_sprite);
        }
    }

    for n in 18..35 {
        let new_terrain = world.add_terrain(n*32,352);
        world.set_sprite(new_terrain,wall2_sprite);
        world.add_terrain_tag(new_terrain, terrain::TerrainTags::BlocksMovement);
    }
        let new_terrain = world.add_terrain(544,352);
        world.set_sprite(new_terrain,wall3_sprite);


    for m in 12..70 {
        let new_terrain = world.add_terrain(544,m*32);
        world.set_sprite(new_terrain,wall_sprite);
        world.add_terrain_tag(new_terrain, terrain::TerrainTags::BlocksMovement);
    }
    for n in 18..35 {
        for m in 12..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            let x: u8 = random();
            if x > 150{
            world.set_sprite(new_terrain,dirt_sprite);
            } else{
                world.set_sprite(new_terrain,dirt2_sprite); 
            }
        }
    }

    let mut parser = json_parsing::JSON_parser::new();
    parser.parse_and_convert_game_data("src/game_data/entity_archetypes.json", "src/game_data/entity_attack_patterns.json", "src/game_data/entity_attacks.json");
    
    let ghost = world.create_entity_from_json_archetype(900.0, 600.0, "ghost", &parser);
    world.set_sprite(ghost, ghost_sprite);

    let ghost2 = world.create_entity_from_json_archetype(1200.0, 600.0, "ghost", &parser);
    world.set_sprite(ghost2, ghost_sprite);


    pollster::block_on(window::run(&mut world, &mut camera));
}