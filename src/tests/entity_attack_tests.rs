use crate::game_engine::entities::EntityAttack;
use crate::game_engine::entities::EntityAttackPattern;
use crate::game_engine::entities::EntityTags;
use crate::tests::tests::basic_world;
use crate::tests::tests::basic_camera;
use super::lib::headless::HeadlessGame;

#[tokio::test]
async fn test_entity_can_kill_player(){
    let mut world = basic_world().await;
    let entity = world.add_entity(900.0, 405.0);
    world.set_sprite(entity, 0);
    world.add_entity_tag(entity, EntityTags::MovementSpeed(2.0));
    world.add_entity_tag(entity, EntityTags::Range(47));
    world.add_entity_tag(entity, EntityTags::AggroRange(1000));
    world.add_entity_tag(entity, EntityTags::Aggressive);
    world.add_entity_tag(entity, EntityTags::FollowsPlayer);
    let attack = EntityAttack::new(100.0);
    let attack_pattern = EntityAttackPattern::new(vec![attack], vec![0.1]);
    world.add_entity_tag(entity, EntityTags::Attacks(attack_pattern));
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