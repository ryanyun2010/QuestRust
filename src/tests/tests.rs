use core::panic;
use std::fmt::format;
use std::num::ParseIntError;

use super::headless::HeadlessGame;
use crate::game_engine;
use crate::game_engine::entities::Entity;
use crate::game_engine::player;
use crate::game_engine::world;
use crate::game_engine::camera;
use crate::game_engine::json_parsing;
use crate::rendering_engine::abstractions;
use crate::game_engine::player::Player;
use crate::game_engine::entities::EntityTags;
use crate::game_engine::entities::EntityAttackPattern;
use crate::game_engine::entities::EntityAttack;
use crate::game_engine::terrain::TerrainTags;
use crate::tests::headless;
use colored::Colorize;



pub async fn run(camera: camera::Camera){
    let time_tracker = std::time::Instant::now();

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

    /* TEST WORLD 1 - NOTHING HAPPENS IN BLANK WORLD */

    let mut world = world::World::new(Player::new(596.0, 400.0, 10.0, 10, 1.0, 0));
    world.add_sprite(0);


    let mut headless = HeadlessGame::new(world.clone(), camera.clone());
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;
    headless.run(1000).await;


    assert_colored_print("Player X stays constant in blank world", headless.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in blank world", headless.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player health stays constant in blank world", headless.world.player.borrow().health == headless.world.player.borrow().max_health as f32);    

    /* TEST WORLD 2 - ENTITIES CAN KILL PLAYER */

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

    let mut headless = HeadlessGame::new(world2.clone(), camera.clone());
    let player_starting_position_x = world2.player.borrow().x;
    let player_starting_position_y = world2.player.borrow().y;
    headless.run(1000).await;

    assert_colored_print("Player X stays constant in world with one entity", headless.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with one entity", headless.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player dies in world with entity attacking it", headless.world.player.borrow().health < 0.0);

    /* TEST WORLD 3 - ENTITIES ARE BLOCKED BY TERRAIN */

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
    
    let mut headless = HeadlessGame::new(world3.clone(), camera.clone());
    let player_starting_position_x = world3.player.borrow().x;
    let player_starting_position_y = world3.player.borrow().y;
    headless.run(1000).await;

    assert_colored_print("Player X stays constant in world with terrain blocker and entity", headless.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with terrain blocker and entity", headless.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player survives in world with terrain blocker and entity attacking", headless.world.player.borrow().health == headless.world.player.borrow().max_health as f32);

    /* TEST WORLD 4 - ENTITY PATHFINDING */

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
    
    let mut headless = HeadlessGame::new(world4.clone(), camera.clone());
    let player_starting_position_x = world4.player.borrow().x;
    let player_starting_position_y = world4.player.borrow().y;
    headless.run(1000).await;
    
    assert_colored_print("Player X stays constant in world with ineffective terrain blocker and entity", headless.world.player.borrow().x == player_starting_position_x);
    assert_colored_print("Player Y stays constant in world with ineffective terrain blocker and entity", headless.world.player.borrow().y == player_starting_position_y);
    assert_colored_print("Player dies in world with ineffective terrain blocker and entity attacking (entity pathfinding works)", headless.world.player.borrow().health <= 0.0);

    /* TEST WORLD 5 - PLAYER MOVEMENT */

    let mut world5 = world.clone();

    let mut headless = HeadlessGame::new(world5.clone(), camera.clone());
    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x.clone();
    headless.run(20).await;
    assert_colored_print("D key moves player right in a blank world", headless.world.player.borrow().x > player_starting_x && headless.world.player.borrow().y == player_starting_position_y); 

    let player_right_x = headless.world.player.borrow().x.clone();
    headless.state.keys_down.insert(String::from("a"), true);
    headless.state.keys_down.insert(String::from("d"), false);
    headless.run(20).await; 
    assert_colored_print("A key moves player left in a blank world", headless.world.player.borrow().x < player_right_x && headless.world.player.borrow().y == player_starting_position_y); 
    assert_colored_print("Player moving left speed is equal to player moving right speed", headless.world.player.borrow().x == player_starting_x); 

    headless.world.player.borrow_mut().x = player_starting_x;
    let player_starting_y = headless.world.player.borrow().y.clone();
    headless.state.keys_down.insert(String::from("s"), true);
    headless.state.keys_down.insert(String::from("a"), false);
    headless.run(20).await;
    assert_colored_print("S key moves player down in a blank world", headless.world.player.borrow().y > player_starting_y && headless.world.player.borrow().x == player_starting_x);

    let player_down_y = headless.world.player.borrow().y.clone();
    headless.state.keys_down.insert(String::from("w"), true);
    headless.state.keys_down.insert(String::from("s"), false);
    headless.run(20).await;
    assert_colored_print("W key moves player up in a blank world", headless.world.player.borrow().y < player_down_y && headless.world.player.borrow().x == player_starting_x);
    assert_colored_print("Player moving up speed is equal to player moving down speed", headless.world.player.borrow().y == player_starting_y);

    headless.world.player.borrow_mut().y = player_starting_y;
    headless.state.keys_down.insert(String::from("w"), false);
    headless.state.keys_down.insert(String::from("d"), true);
    headless.state.keys_down.insert(String::from("a"), true);
    headless.run(20).await;
    assert_colored_print("Holding both D and A does not move player", headless.world.player.borrow().y == player_starting_y && headless.world.player.borrow().x == player_starting_x);

    headless.state.keys_down.insert(String::from("w"), true);
    headless.state.keys_down.insert(String::from("s"), true);
    headless.state.keys_down.insert(String::from("d"), false);
    headless.state.keys_down.insert(String::from("a"), false);
    headless.run(20).await;
    assert_colored_print("Holding both W and S does not move player", headless.world.player.borrow().y == player_starting_y && headless.world.player.borrow().x == player_starting_x);

    /* TEST WORLD 6 - PLAYER COLLISION WITH TERRAIN */

    let mut world6 = world.clone();
    let terrain_blocker = world6.add_terrain(638, 384);
    world6.set_sprite(terrain_blocker, 0);
    world6.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);   
    let terrain_blocker = world6.add_terrain(554, 384);
    world6.set_sprite(terrain_blocker, 0);
    world6.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    let terrain_blocker = world6.add_terrain(576, 358);
    world6.set_sprite(terrain_blocker, 0);
    world6.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    let terrain_blocker = world6.add_terrain(576, 442);
    world6.set_sprite(terrain_blocker, 0);
    world6.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);

    let mut headless = HeadlessGame::new(world6.clone(), camera.clone());
    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x.clone();
    headless.run(200).await;
    assert_colored_print("Moving right does not allow player to move into terrain blocker, but player can move right up to the terrain blocker", headless.world.player.borrow().x == player_starting_x + 10.0);

    headless.state.keys_down.insert(String::from("d"), false);
    headless.state.keys_down.insert(String::from("a"), true);
    headless.run(200).await;
    assert_colored_print("Moving left does not allow player to move into terrain blocker, but player can move left up to the terrain blocker", headless.world.player.borrow().x == player_starting_x - 10.0);

    headless.state.keys_down.insert(String::from("a"), false);
    headless.state.keys_down.insert(String::from("w"), true);
    headless.run(200).await;
    assert_colored_print("Moving up does not allow player to move into terrain blocker, but player can move up up to the terrain blocker", headless.world.player.borrow().y == player_starting_y - 10.0);

    headless.state.keys_down.insert(String::from("w"), false);
    headless.state.keys_down.insert(String::from("s"), true);
    headless.run(200).await;
    assert_colored_print("Moving down does not allow player to move into terrain blocker, but player can move down up to the terrain blocker", headless.world.player.borrow().y == player_starting_y + 10.0);

    /* TEST WORLD 7 - PLAYER COLLISION WITH ENTITIES */

    let mut world7 = world.clone();
    let entity_blocker = world7.add_entity(638.0, 384.0);
    world7.set_sprite(entity_blocker, 0);
    world7.add_entity_tag(entity_blocker, EntityTags::HasCollision);
    let entity_blocker = world7.add_entity(554.0, 384.0);
    world7.set_sprite(entity_blocker, 0);
    world7.add_entity_tag(entity_blocker, EntityTags::HasCollision);
    let entity_blocker = world7.add_entity(576.0, 358.0);
    world7.set_sprite(entity_blocker, 0);
    world7.add_entity_tag(entity_blocker, EntityTags::HasCollision);
    let entity_blocker = world7.add_entity(576.0, 442.0);
    world7.set_sprite(entity_blocker, 0);
    world7.add_entity_tag(entity_blocker, EntityTags::HasCollision);

    let mut headless = HeadlessGame::new(world7.clone(), camera.clone());
    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x.clone();
    headless.run(200).await;
    assert_colored_print("Moving right does not allow player to move into entity blocker, but player can move right up to the entity blocker", headless.world.player.borrow().x == player_starting_x + 10.0);

    headless.state.keys_down.insert(String::from("d"), false);
    headless.state.keys_down.insert(String::from("a"), true);
    headless.run(200).await;
    assert_colored_print("Moving left does not allow player to move into entity blocker, but player can move left up to the entity blocker", headless.world.player.borrow().x == player_starting_x - 10.0);

    headless.state.keys_down.insert(String::from("a"), false);
    headless.state.keys_down.insert(String::from("w"), true);
    headless.run(200).await;
    assert_colored_print("Moving up does not allow player to move into entity blocker, but player can move up up to the entity blocker", headless.world.player.borrow().y == player_starting_y - 10.0);

    headless.state.keys_down.insert(String::from("w"), false);
    headless.state.keys_down.insert(String::from("s"), true);
    headless.run(200).await;
    assert_colored_print("Moving down does not allow player to move into entity blocker, but player can move down up to the entity blocker", headless.world.player.borrow().y == player_starting_y + 10.0);

    
    if failcount == 0{
        println!("{} {} {}", "All tests passed in".green(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().green(), "seconds".green());
    }else if failcount == 1{
        println!("{} {} {} {} {}", "1".red(), "TEST FAILED".red(), "IN".red(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().red(), "SECONDS".red());
        println!("{} {}", "Failed test:".red(), fail_list[0].red());
        panic!("{}", "Tests failed \nnote: to test run the game even though the tests fail, use 'cargo run -- no-tests'".red());
    } else{
        println!("{} {} {} {} {}", failcount.to_string().red(), "TESTS FAILED".red(), "IN".red(), ((time_tracker.elapsed().as_millis() as f32)/1000.0).to_string().red(), "SECONDS".red());
        println!("{} {}", "Failed tests:".red(), fail_list.join(", ").red());
        panic!("{}", "Tests failed \nnote: to test run the game even though the tests fail, use 'cargo run -- no-tests'".red());
    }

}


