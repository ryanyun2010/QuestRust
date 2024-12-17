
use core::arch;

use rand::prelude::*;
pub mod rendering_engine;
use rendering_engine::window;
use rendering_engine::state;
use rendering_engine::vertex;
use rendering_engine::texture;
pub mod game_engine;
// use game_engine::*;
use game_engine::world;
use game_engine::camera;
use game_engine::loot;
use game_engine::entities;
use game_engine::stat;
use game_engine::terrain;
use game_engine::magic;
use game_engine::entities::EntityTags;
use game_engine::ui::UIElement;

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
    let dirt_sprite = world.add_sprite(5);
    let dirt2_sprite = world.add_sprite(4);
    let wall_sprite = world.add_sprite(7);
    let ghost_sprite = world.add_sprite(8);
    for n in 0..17 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,outside_sprite);
        }
    }
    for m in 0..70 {
        let new_terrain = world.add_terrain(544,m*32);
        world.set_sprite(new_terrain,wall_sprite);
        world.add_terrain_tag(new_terrain, terrain::TerrainTags::BlocksMovement);
    }
    for n in 18..35 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            let x: u8 = random();
            if x > 150{
            world.set_sprite(new_terrain,dirt_sprite);
            } else{
                world.set_sprite(new_terrain,dirt2_sprite); 
            }
        }
    }


    
    let mut archetype: Vec<EntityTags> = Vec::new();
    archetype.push(EntityTags::Aggressive);
    archetype.push(EntityTags::MovementSpeed(2.0));
    archetype.push(EntityTags::MonsterType(entities::MonsterType::Undead));
    archetype.push(EntityTags::FollowsPlayer);
    archetype.push(EntityTags::Range(32));
    archetype.push(EntityTags::AggroRange(1000));
    archetype.push(EntityTags::AttackType(entities::AttackType::Melee));
    let mut attacks_tests = Vec::new();
    attacks_tests.push(entities::EntityAttack::new(3));


    let ghost = world.add_entity(160.0,160.0);
    world.add_entity_tags(ghost, archetype.clone());
    world.add_entity_tag(ghost, EntityTags::Attacks(entities::EntityAttackPattern::new(attacks_tests.clone(), vec![0.1])));
    world.set_sprite(ghost,ghost_sprite);

    for i in 0..10000{
        let ghost2 = world.add_entity(i as f32/10.0,160.0);
        world.add_entity_tags(ghost2, archetype.clone());
        world.add_entity_tag(ghost2, EntityTags::Attacks(entities::EntityAttackPattern::new(attacks_tests.clone(), vec![0.1])));
        // world.add_entity_tag(ghost2, EntityTags::RespectsCollision);
        // COLLISION TAG IS REALLY FUCKING SLOW, THIS RUNS FINE WITH 10000 ghosts normally, but with it, like 50 entities max. Need to fix this REALLY BADLY.
        world.set_sprite(ghost2,ghost_sprite);
    }
    
    pollster::block_on(window::run(&mut world, &mut camera));
}