#![cfg(test)]
use crate::game_engine::entities::{EntityAttack, EntityAttackPattern, EntityTags};
use crate::game_engine::terrain::TerrainTags;
use crate::tests::tests::{basic_camera, basic_world};
use crate::tests::lib::headless::HeadlessGame;
use crate::game_engine::entity_components;

#[tokio::test]
async fn test_terrain_should_block_entities(){
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    for y in 0..25{
        let terrain_blocker = world.add_terrain(704, y * 32);
        world.set_sprite(terrain_blocker, 0);
        world.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    }
    let entity = world.add_entity(900.0, 405.0);
    world.set_sprite(entity, 0);
    let attack = EntityAttack::new(10.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::MovementSpeed(2.0),
            EntityTags::Range(47),
            EntityTags::AggroRange(1000),
            EntityTags::Aggressive,
            EntityTags::FollowsPlayer,
            EntityTags::RespectsCollision,
            EntityTags::HasCollision,
            EntityTags::Attacks(attack_pattern),
        ]
    );
    world.set_entity_archetype(entity, String::from("Test"));
    world.add_attack_component(entity, entity_components::EntityAttackComponent::default());
    world.add_health_component(entity, entity_components::HealthComponent{health: 100.0, max_health: 100});
    world.add_collision_box_component(entity, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    world.add_pathfinding_component(entity, entity_components::PathfindingComponent::default());
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.run(1000).await;

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
    let camera = basic_camera().await;
    for y in 5..18{
        let terrain_blocker = world.add_terrain(704, y * 32);
        world.set_sprite(terrain_blocker, 0);
        world.add_terrain_tag(terrain_blocker, TerrainTags::BlocksMovement);
    }
    let entity = world.add_entity(900.0, 405.0);
    world.set_sprite(entity, 0);
    let attack = EntityAttack::new(10.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world.add_entity_archetype(
        String::from("Test"),
        vec![
            EntityTags::MovementSpeed(2.0),
            EntityTags::Range(47),
            EntityTags::AggroRange(1000),
            EntityTags::Aggressive,
            EntityTags::FollowsPlayer,
            EntityTags::RespectsCollision,
            EntityTags::HasCollision,
            EntityTags::Attacks(attack_pattern),
        ]
    );
    world.set_entity_archetype(entity, String::from("Test"));
    world.add_attack_component(entity, entity_components::EntityAttackComponent::default());
    world.add_health_component(entity, entity_components::HealthComponent{health: 100.0, max_health: 100});
    world.add_collision_box_component(entity, entity_components::CollisionBox{
        w: 32.0,
        h: 32.0,
        x_offset: 0.0,
        y_offset: 0.0
    });
    world.add_pathfinding_component(entity, entity_components::PathfindingComponent::default());
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;

    let mut headless = HeadlessGame::new(world, camera);
    headless.run(1000).await;
    
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