#![cfg(test)]
use crate::{create_stat_list, game_engine::{entity_components::CollisionBox, game::MousePosition, item::{Item, ItemType}}, tests::lib::headless::HeadlessGame};

use super::tests::{basic_world, basic_camera};
#[tokio::test]
pub async fn test_melee_player_attack() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_entity_archetype(String::from("test_attackable_entity"), vec![
        crate::game_engine::entities::EntityTags::BaseHealth(100),
        crate::game_engine::entities::EntityTags::Damageable(
            CollisionBox {
                x_offset: 0.0,
                y_offset: 0.0,
                w: 32.0,
                h: 32.0
            }
        )
    ]);
    let item = world.inventory.add_item(
        Item {
            name: String::from("test_sword"),
            attack_sprite: String::from("melee_attack"),
            item_type: ItemType::MeleeWeapon,
            lore: String::from("test"),
            sprite: String::from("sword"),
            stats: create_stat_list!(
                damage => 150.0,
                width => 50.0,
                reach => 65.0
            )
        }
    );
    world.inventory.set_hotbar_slot_item(0, item);
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

#[tokio::test]
pub async fn test_ranged_player_attack() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_entity_archetype(String::from("test_attackable_entity"), vec![
        crate::game_engine::entities::EntityTags::BaseHealth(100),
        crate::game_engine::entities::EntityTags::Damageable(
            CollisionBox {
                x_offset: 0.0,
                y_offset: 0.0,
                w: 32.0,
                h: 32.0
            }
        )
    ]);
    let item = world.inventory.add_item(
        Item {
            name: String::from("test_spear"),
            attack_sprite: String::from("spear"),
            item_type: ItemType::RangedWeapon,
            lore: String::from("test"),
            sprite: String::from("spear"),
            stats: create_stat_list!(
                damage => 150.0,
                lifetime => 400.0,
                speed => 10.0,
                size => 30.0,
                AOE => 30.0
            )
        }
    );
    world.inventory.set_hotbar_slot_item(0, item);
    world.create_entity_with_archetype(689.0, 400.0, String::from("test_attackable_entity"));
    let mut headless = HeadlessGame::new(world, camera);
    headless.world.on_mouse_click(MousePosition {
            x_screen: 689.0,
            y_screen: 400.0,
            x_world: 689.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false, headless.camera.viewpoint_width as f32, headless.camera.viewpoint_height as f32);
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    headless.run(200).await;
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
}