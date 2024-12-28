use core::panic;
use std::fmt::format;
use std::num::ParseIntError;

use super::headless::HeadlessGame;
use crate::game_engine;
use crate::game_engine::entities::Entity;
use crate::game_engine::world;
use crate::game_engine::camera;
use crate::game_engine::json_parsing;
use crate::rendering_engine::abstractions;
use crate::game_engine::player::Player;
use crate::game_engine::entities::EntityTags;
use crate::game_engine::entities::EntityAttackPattern;
use crate::game_engine::entities::EntityAttack;
use crate::game_engine::terrain::TerrainTags;
use colored::Colorize;



pub async fn run(camera: camera::Camera){
    let time_tracker = std::time::Instant::now();
    let mut world = world::World::new(Player::new(596.0, 400.0, 10.0, 10, 1.0, 0));
    world.add_sprite(0);
    let mut headless = HeadlessGame::new(world.clone(), camera.clone());
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;
    headless.run(1000).await;
    let mut failcount = 0;
    let mut fail_list = Vec::new();

    let mut assert_colored_print = |name: &str, assertion: bool| {
        if assertion {
            println!("{}: {}", name, "PASSED".green());
        } else {
            println!("{}: {}", name, "FAILED".red());
            failcount += 1;
            fail_list.push(name.to_string());
        }
    };

    println!("TESTS:");


    assert_colored_print("Player X stays constant in blank world", headless.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in blank world", headless.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player health stays constant in blank world", headless.world.player.borrow().health == headless.world.player.borrow().max_health as f32);
    

    let mut world2 = world.clone(); 
    let entity = world2.add_entity(900.0, 405.0);
    world2.set_sprite(entity, 0);
    world2.add_entity_tag(entity, EntityTags::MovementSpeed(2.0));
    world2.add_entity_tag(entity, EntityTags::Range(47));
    world2.add_entity_tag(entity, EntityTags::AggroRange(1000));
    world2.add_entity_tag(entity, EntityTags::Aggressive);
    world2.add_entity_tag(entity, EntityTags::FollowsPlayer);
    let attack = EntityAttack::new(10.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world2.add_entity_tag(entity, EntityTags::Attacks(attack_pattern));
    let mut headless2 = HeadlessGame::new(world2.clone(), camera.clone());
    let player_starting_position_x = world2.player.borrow().x;
    let player_starting_position_y = world2.player.borrow().y;
    headless2.run(1000).await;


    assert_colored_print("Player X stays constant in world with one entity", headless2.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with one entity", headless2.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player dies in world with entity attacking it", headless2.world.player.borrow().health < 0.0);


    let mut world3 = world.clone();
    for y in 0..25{
        let terrain_blocker = world3.add_terrain(704, y * 32);
        world3.set_sprite(terrain_blocker, 0);
        world3.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    }
    let entity = world3.add_entity(900.0, 405.0);
    world3.set_sprite(entity, 0);
    world3.add_entity_tag(entity, EntityTags::MovementSpeed(2.0));
    world3.add_entity_tag(entity, EntityTags::Range(47));
    world3.add_entity_tag(entity, EntityTags::AggroRange(1000));
    world3.add_entity_tag(entity, EntityTags::Aggressive);
    world3.add_entity_tag(entity, EntityTags::FollowsPlayer);
    world3.add_entity_tag(entity, EntityTags::RespectsCollision);
    world3.add_entity_tag(entity, EntityTags::HasCollision);
    let attack = EntityAttack::new(10.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world3.add_entity_tag(entity, EntityTags::Attacks(attack_pattern));
    
    let mut headless3 = HeadlessGame::new(world3.clone(), camera.clone());
    let player_starting_position_x = world3.player.borrow().x;
    let player_starting_position_y = world3.player.borrow().y;
    headless3.run(1000).await;


    assert_colored_print("Player X stays constant in world with terrain blocker and entity", headless3.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with terrain blocker and entity", headless3.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player survives in world with terrain blocker and entity attacking", headless3.world.player.borrow().health == headless3.world.player.borrow().max_health as f32);



    let mut world4 = world.clone();
    for y in 5..18{
        let terrain_blocker = world3.add_terrain(704, y * 32);
        world4.set_sprite(terrain_blocker, 0);
        world4.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    }
    let entity = world4.add_entity(900.0, 405.0);
    world4.set_sprite(entity, 0);
    world4.add_entity_tag(entity, EntityTags::MovementSpeed(2.0));
    world4.add_entity_tag(entity, EntityTags::Range(47));
    world4.add_entity_tag(entity, EntityTags::AggroRange(1000));
    world4.add_entity_tag(entity, EntityTags::Aggressive);
    world4.add_entity_tag(entity, EntityTags::FollowsPlayer);
    world4.add_entity_tag(entity, EntityTags::RespectsCollision);
    world4.add_entity_tag(entity, EntityTags::HasCollision);
    let attack = EntityAttack::new(10.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world4.add_entity_tag(entity, EntityTags::Attacks(attack_pattern));
    
    let mut headless4 = HeadlessGame::new(world4.clone(), camera.clone());
    let player_starting_position_x = world4.player.borrow().x;
    let player_starting_position_y = world4.player.borrow().y;
    headless4.run(1000).await;

    assert_colored_print("Player X stays constant in world with ineffective terrain blocker and entity", headless4.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with ineffective terrain blocker and entity", headless4.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player dies in world with ineffective terrain blocker and entity attacking (entity pathfinding works)", headless4.world.player.borrow().health <= 0.0);

    if failcount == 0{
        println!("{} {} {}", "All tests passed in".green(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().green(), "seconds".green());
    }else if failcount == 1{
        println!("{} {} {} {} {}", "1".red(), "TEST FAILED".red(), "IN".red(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().red(), "SECONDS".red());
        println!("{} {}", "Failed test:".red(), fail_list[0].red());
        // panic!("{}", "Tests failed".red());
    } else{
        println!("{} {} {} {} {}", failcount.to_string().red(), "TESTS FAILED".red(), "IN".red(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().red(), "SECONDS".red());
        println!("{} {}", "Failed tests:".red(), fail_list.join(", ").red());
        // panic!("{}", "Tests failed".red());
    }

}


