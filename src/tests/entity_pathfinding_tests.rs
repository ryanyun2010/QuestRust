#![cfg(test)]
use compact_str::{CompactString, ToCompactString};

use crate::game_engine::entities::{AttackType, EntityAttackPattern};
use crate::game_engine::entity_attacks::EntityAttackDescriptor;
use crate::game_engine::terrain::TerrainTags;
use crate::ok_or_panic;
use crate::tests::test_framework::{basic_camera, basic_entity, basic_world};
use crate::tests::lib::headless::HeadlessGame;
use crate::game_engine::entity_components;

#[tokio::test]
async fn test_terrain_should_block_entities(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    for y in 0..25{
        let terrain_blocker = world.add_terrain(704, y * 32);
        world.set_terrain_archetype(terrain_blocker, "test".into());
    }
    world.entity_attack_descriptor_lookup.insert("test_attack".to_compact_string(), EntityAttackDescriptor{
        damage: 100.0,
        reach: 50,
        width: 50,
        r#type: AttackType::Melee,
        max_start_dist_from_entity: None,
        time_to_charge: 5,
        sprite: "attack_highlight".to_compact_string()
    });
    let attack_pattern = EntityAttackPattern::new(vec!["test_attack".to_compact_string()], vec![0.1]);
    world.entity_attack_pattern_lookup.insert("test".into(), attack_pattern);
    world.add_entity_archetype("Test".into(), basic_entity().await);
    let entity = world.create_entity_with_archetype(900.0, 405.0, "Test".into());
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    ok_or_panic!(headless.run(1000).await);

    assert!(
        headless.world.player.borrow().x == player_starting_position_x,
        "Player X should stay constant without input",
        );
    assert!(
        headless.world.player.borrow().y == player_starting_position_y,
        "Player Y should stay constant without input",
    );
    assert!(
        headless.world.player.borrow().health == headless.world.player.borrow().max_health as f32,
        "Player should survive because the attacking entity should be blocked by terrain"
    );
}

#[tokio::test]
async fn test_entities_should_pathfind_around_terrain(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_terrain_archetype(
        "test".into(),
        vec![
            TerrainTags::BlocksMovement,
            TerrainTags::Sprite(0)
        ]
    );
    for y in 5..18{
        let terrain_blocker = world.add_terrain(704, y * 32);
        world.set_terrain_archetype(terrain_blocker, "test".into());
    }


    world.entity_attack_descriptor_lookup.insert("test_attack".to_compact_string(), EntityAttackDescriptor{
        damage: 100.0,
        reach: 50,
        width: 50,
        r#type: AttackType::Melee,
        max_start_dist_from_entity: None,
        time_to_charge: 5,
        sprite: "attack_highlight".to_compact_string()
    });
    let attack_pattern = EntityAttackPattern::new(vec!["test_attack".to_compact_string()], vec![0.1]);
    world.entity_attack_pattern_lookup.insert("test".into(), attack_pattern);
    world.add_entity_archetype("Test".into(), basic_entity().await);
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;

    let mut headless = HeadlessGame::new(world, camera);
    ok_or_panic!(headless.run(1000).await);

    assert!(
        headless.world.player.borrow().x == player_starting_position_x,
        "Player X should stay constant without input", 
    );
    assert!(
        headless.world.player.borrow().y == player_starting_position_y, 
        "Player Y should stay constant without input"
    );
    assert!( 
        headless.world.player.borrow().health <= 0.0,
        "Player should die because the attacking entity should pathfind around the terrain"
    );
}
