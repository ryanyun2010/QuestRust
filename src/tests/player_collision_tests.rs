#![cfg(test)]
use crate::tests::tests::{basic_camera, basic_world};
use crate::tests::lib::headless::HeadlessGame;
use crate::game_engine::{terrain::TerrainTags, entities::EntityTags, entity_components};

#[tokio::test]
async fn test_player_terrain_collision_moving_right(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let terrain_blocker = world.add_terrain(638, 384);
    world.set_sprite(terrain_blocker, 0);
    let blocker_archetype = world.add_terrain_archetype(
        vec![
            TerrainTags::BlocksMovement,
        ]
    );
    world.set_terrain_archetype(terrain_blocker, blocker_archetype);
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("d"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().x == player_starting_x + 10.0,
        "Player should not be able to move right into a terrain blocker, but the player should be able to move right all the way up to it"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_left(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let terrain_blocker = world.add_terrain(554, 384);
    world.set_sprite(terrain_blocker, 0);
    let blocker_archetype = world.add_terrain_archetype(
        vec![
            TerrainTags::BlocksMovement,
        ]
    );
    world.set_terrain_archetype(terrain_blocker, blocker_archetype);
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("a"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().x == player_starting_x - 10.0,
        "Player should not be able to move left into a terrain blocker, but the player should be able to move left all the way up to it"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_up(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let terrain_blocker = world.add_terrain(576, 358);
    world.set_sprite(terrain_blocker, 0);
    let blocker_archetype = world.add_terrain_archetype(
        vec![
            TerrainTags::BlocksMovement,
        ]
    );
    world.set_terrain_archetype(terrain_blocker, blocker_archetype);
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("w"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().y == player_starting_y - 10.0,
        "Player should not be able to move up into a terrain blocker, but the player should be able to move up all the way up to it"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_down(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let terrain_blocker = world.add_terrain(576, 442);
    world.set_sprite(terrain_blocker, 0);
    let blocker_archetype = world.add_terrain_archetype(
        vec![
            TerrainTags::BlocksMovement,
        ]
    );
    world.set_terrain_archetype(terrain_blocker, blocker_archetype);
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("s"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().y == player_starting_y + 10.0,
        "Player should not be able to move down into a terrain blocker, but the player should be able to move down all the way up to it"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_down(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let entity_blocker = world.add_entity(576.0, 442.0);
    world.set_sprite(entity_blocker, 0);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::HasCollision,
        ]
    );
    world.set_entity_archetype(entity_blocker, String::from("Test"));
    world.add_collision_box_component(entity_blocker, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("s"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().y == player_starting_y + 10.0,
        "Player should not be able to move down into a entity blocker, but the player should be able to move down all the way up to it"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_up(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let entity_blocker = world.add_entity(576.0, 358.0);
    world.set_sprite(entity_blocker, 0);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::HasCollision,
        ]
    );
    world.set_entity_archetype(entity_blocker, String::from("Test"));
    world.add_collision_box_component(entity_blocker, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("w"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().y == player_starting_y - 10.0,
        "Player should not be able to move up into a entity blocker, but the player should be able to move up all the way up to it"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_left(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let entity_blocker = world.add_entity(554.0, 384.0);
    world.set_sprite(entity_blocker, 0);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::HasCollision,
        ]
    );
    world.set_entity_archetype(entity_blocker, String::from("Test"));
    world.add_collision_box_component(entity_blocker, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("a"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().x == player_starting_x - 10.0,
        "Player should not be able to move left into a entity blocker, but the player should be able to move left all the way up to it"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_right(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    let entity_blocker = world.add_entity(638.0, 384.0);
    world.set_sprite(entity_blocker, 0);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::HasCollision,
        ]
    );
    world.set_entity_archetype(entity_blocker, String::from("Test"));
    world.add_collision_box_component(entity_blocker, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(String::from("d"), true);
    headless.run(200).await;
    assert!(
        headless.world.player.borrow().x == player_starting_x + 10.0,
        "Player should not be able to move right into a entity blocker, but the player should be able to move right all the way up to it"
    )
}