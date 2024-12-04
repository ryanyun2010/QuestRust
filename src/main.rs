use game_engine::world::Entity;
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
use game_engine::entities::EntityTags;
use game_engine::item::Item;
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


    let ghost = world.add_entity(160.0,160.0);
    world.add_tag(ghost, EntityTags::Aggressive);
    world.add_tag(ghost, EntityTags::MovementSpeed(2.0));
    world.add_tag(ghost, EntityTags::MonsterType(entities::MonsterType::Undead));
    world.add_tag(ghost, EntityTags::FollowsPlayer);
    world.add_tag(ghost, EntityTags::Range(32));
    world.add_tag(ghost, EntityTags::AggroRange(1000));
    world.add_tag(ghost, EntityTags::AttackType(entities::AttackType::Melee));
    let mut attacks_tests = Vec::new();
    attacks_tests.push(entities::EntityAttack::new(3));
    world.add_tag(ghost, EntityTags::Attacks(entities::EntityAttackPattern::new(attacks_tests, vec![0.1])));


    //   game_engine::entities::EntityTags::new(true, game_engine::entities::MonsterType::Undead, true, 0, 1500, game_engine::entities::AttackType::Melee, game_engine::entities::EntityAttackPattern::new(), 3, false, Some(game_engine::loot::Loot::new(Vec::new())), None, 10)


    // println!("{:?}",world.chunks);
    world.set_sprite(ghost,ghost_sprite);
    // println!("{:?}",world.chunks[0]);
    
    pollster::block_on(window::run(&mut world, &mut camera));
}