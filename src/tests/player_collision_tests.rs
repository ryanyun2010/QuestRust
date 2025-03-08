#![cfg(test)]
use compact_str::CompactString;

use crate::ok_or_panic;
use crate::tests::test_framework::{basic_camera, basic_world};
use crate::tests::lib::headless::HeadlessGame;
use crate::game_engine::{terrain::TerrainTags, entities::EntityTags, entity_components};

#[tokio::test]
async fn test_player_terrain_collision_moving_right(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let terrain_blocker = world.add_terrain(638, 402);
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    world.set_terrain_archetype(terrain_blocker, "test".into());
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("d"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().x < player_starting_x + 50.0,
        "Player should not be able to move right through a terrain blocker"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_left(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let terrain_blocker = world.add_terrain(554, 402);
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    world.set_terrain_archetype(terrain_blocker, "test".into());
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("a"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().x > player_starting_x - 50.0,
        "Player should not be able to move left through a terrain blocker"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_up(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let terrain_blocker = world.add_terrain(576, 358);
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    world.set_terrain_archetype(terrain_blocker, "test".into());
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("w"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().y > player_starting_y - 50.0,
        "Player should not be able to move up through a terrain blocker"
    )
}

#[tokio::test]
async fn test_player_terrain_collision_moving_down(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let terrain_blocker = world.add_terrain(576, 442);
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    world.set_terrain_archetype(terrain_blocker, "test".into());
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("s"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().y < player_starting_y + 50.0,
        "Player should not be able to move down through a terrain blocker"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_down(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let entity_blocker = world.add_entity(576.0, 442.0);
    world.add_entity_archetype(
        CompactString::from("Test"),
        vec![
            EntityTags::HasCollision(
                entity_components::CollisionBox{
                    w: 32.0,
                    h: 32.0,
                    x_offset: 0.0,
                    y_offset: 0.0
                }
            ),
            EntityTags::Sprite(0)
        ]
    );
    world.set_entity_archetype(entity_blocker, CompactString::from("Test"));
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("s"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().y < player_starting_y + 50.0,
        "Player should not be able to move down through an entity blocker"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_up(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let entity_blocker = world.add_entity(576.0, 358.0);
    world.add_entity_archetype(
        CompactString::from("Test"),
        vec![
            EntityTags::HasCollision(
                entity_components::CollisionBox{
                    w: 32.0,
                    h: 32.0,
                    x_offset: 0.0,
                    y_offset: 0.0
                }
            ),
            EntityTags::Sprite(0)
        ]
    );
    world.set_entity_archetype(entity_blocker, CompactString::from("Test"));
    let player_starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("w"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().y > player_starting_y - 50.0,
        "Player should not be able to move through an entity blocker"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_left(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let entity_blocker = world.add_entity(554.0, 402.0);
    world.add_entity_archetype(
        CompactString::from("Test"),
        vec![
            EntityTags::HasCollision(
                entity_components::CollisionBox{
                    w: 32.0,
                    h: 32.0,
                    x_offset: 0.0,
                    y_offset: 0.0
                }
            ),
            EntityTags::Sprite(0)
        ]
    );
    world.set_entity_archetype(entity_blocker, CompactString::from("Test"));
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("a"), true);
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.player.borrow().x > player_starting_x - 50.0,
        "Player should not be able to move left through an entity blocker"
    )
}

#[tokio::test]
async fn test_player_entity_collision_moving_right(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let entity_blocker = world.add_entity(648.0, 402.0);
    world.add_entity_archetype(
        CompactString::from("Test"),
        vec![
            EntityTags::HasCollision(
                entity_components::CollisionBox{
                    w: 32.0,
                    h: 32.0,
                    x_offset: 0.0,
                    y_offset: 0.0
                }
            ),
            EntityTags::Sprite(0)
        ]
    );
    world.set_entity_archetype(entity_blocker, CompactString::from("Test"));
    let player_starting_x = world.player.borrow().x;
    let mut headless = HeadlessGame::new(world, camera);
    headless.state.keys_down.insert(CompactString::from("d"), true);
    ok_or_panic!(headless.run(200).await);
    println!("Player X: {}", headless.world.player.borrow().x);
    assert!(
        headless.world.player.borrow().x < player_starting_x + 50.0,
        "Player should not be able to move right through an entity blocker"
    )
}
