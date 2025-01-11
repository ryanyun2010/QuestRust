#![cfg(test)]
use crate::{game_engine::{entity_components::CollisionBox, game::MousePosition, player_attacks::melee_attack_descriptor}, tests::lib::headless::HeadlessGame};

use super::tests::{basic_world, basic_camera};
#[tokio::test]
pub async fn test_player_attack() {
    // todo!("MAKE IT SO COLLISION IS NOT REQUIRED FOR A THING TO BE DAMAGED");
    let mut world = basic_world().await;
    let camera = basic_camera().await;
    world.add_entity_archetype(String::from("test_attackable_entity"), vec![
        crate::game_engine::entities::EntityTags::BaseHealth(100),
        crate::game_engine::entities::EntityTags::Damageable,
        crate::game_engine::entities::EntityTags::HasCollision(
            CollisionBox{
                x_offset: 0.0,
                y_offset: 0.0,
                w: 32.0,
                h: 32.0
            }
        )
    ]);
    world.add_player_attack_archetype(String::from("test_melee_attack"), crate::game_engine::player_attacks::PlayerAttackDescriptor::Melee(melee_attack_descriptor {
        damage: 150.0,
        width: 50.0,
        reach: 50.0,
        lifetime: 5.0,
        sprite: "melee_attack".to_string(),
    }));
    world.create_entity_with_archetype(639.0, 400.0, String::from("test_attackable_entity"));
    let mut headless = HeadlessGame::new(world, camera);
    headless.world.on_mouse_click(MousePosition {
            x_screen: 639.0,
            y_screen: 400.0,
            x_world: 639.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false, headless.camera.viewpoint_width as f32, headless.camera.viewpoint_height as f32);
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    headless.run(20).await;
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
}