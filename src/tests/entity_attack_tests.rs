#![cfg(test)]
use crate::game_engine::entity_components;
use crate::tests::tests::{basic_world, basic_camera};
use crate::tests::lib::headless::HeadlessGame;
use crate::game_engine::entities::{EntityAttack, EntityAttackPattern};
use crate::game_engine::entities::EntityTags;

#[tokio::test]
async fn test_entity_can_kill_player(){
    let mut world = basic_world().await;
    let entity = world.add_entity(900.0, 405.0);
    world.set_sprite(entity, 0);
    let attack = EntityAttack::new(100.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world.add_entity_archetype(String::from("Test"), vec![
        EntityTags::MovementSpeed(2.0),
        EntityTags::Range(47),
        EntityTags::AggroRange(1000),
        EntityTags::Aggressive,
        EntityTags::FollowsPlayer,
        EntityTags::Attacks(attack_pattern),
        EntityTags::HasCollision(
            entity_components::CollisionBox{
                w: 32.0,
                h: 32.0,
                x_offset: 0.0,
                y_offset: 0.0
            }
        )
    ]);
    world.set_entity_archetype(entity, String::from("Test"));
    world.add_attack_component(entity, entity_components::EntityAttackComponent::default());
    world.add_health_component(entity, entity_components::HealthComponent{health: 100.0, max_health: 100});
    world.add_pathfinding_component(entity, entity_components::PathfindingComponent::default());
    
    let camera = basic_camera().await;
    let player_starting_position_x = world.player.borrow().x;
    let player_starting_position_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.run(1000).await;

    assert!(
        headless.world.player.borrow().x == player_starting_position_x, 
        "Player X should stay constant without input"
    );
    assert!(
        headless.world.player.borrow().y == player_starting_position_y, 
        "Player Y should stay constant without input"
    );
    assert!(
        headless.world.player.borrow().health < 0.0,
        "Player should die with an entity attacking it"
    );
}