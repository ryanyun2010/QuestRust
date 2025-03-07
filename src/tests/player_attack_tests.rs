#![cfg(test)]
use compact_str::CompactString;

use crate::{create_stat_list, game_engine::{entity_components::CollisionBox, game::MousePosition, item::{Item, ItemType}, stat::StatC}, ok_or_panic, tests::lib::headless::HeadlessGame};

use super::test_framework::{basic_world, basic_camera};
#[tokio::test]
pub async fn test_melee_player_attack() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_entity_archetype(CompactString::from("test_attackable_entity"), vec![
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
            name: CompactString::from("test_sword"),
            attack_sprite: Some(CompactString::from("melee_attack")),
            item_type: ItemType::MeleeWeapon,
            width_to_length_ratio: None,
            lore: String::from("test"),
            sprite: CompactString::from("sword"),
            stats: create_stat_list!(
                damage => StatC { flat: 150.0, percent: 0.0},
                width => StatC { flat: 50.0, percent: 0.0},
                reach => StatC { flat: 65.0, percent: 0.0},
            ),
            time_til_usable: 0.0
        }
    );
    ok_or_panic!(world.inventory.set_hotbar_slot_item(0, item));
    world.create_entity_with_archetype(639.0, 400.0, CompactString::from("test_attackable_entity"));
    let mut headless = HeadlessGame::new(world, camera);
    ok_or_panic!(headless.world.on_mouse_click(MousePosition {
            x_screen: 639.0,
            y_screen: 400.0,
            x_world: 639.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false, headless.camera.viewpoint_width as f32, headless.camera.viewpoint_height as f32));
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    ok_or_panic!(headless.run(20).await);
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
}

#[tokio::test]
pub async fn test_ranged_player_attack() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_entity_archetype(CompactString::from("test_attackable_entity"), vec![
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
            name: CompactString::from("test_spear"),
            attack_sprite: Some(CompactString::from("spear")),
            item_type: ItemType::RangedWeapon,
            width_to_length_ratio: None,
            lore: String::from("test"),
            sprite: CompactString::from("spear"),
            stats: create_stat_list!(
                damage => StatC { flat: 150.0, percent: 0.0},
                lifetime => StatC { flat: 400.0, percent: 0.0},
                speed => StatC { flat: 10.0, percent: 0.0},
                size => StatC { flat: 30.0, percent: 0.0},
            ),
            time_til_usable: 0.0,
        }
    );
    ok_or_panic!(world.inventory.set_hotbar_slot_item(0, item));
    world.create_entity_with_archetype(689.0, 400.0, CompactString::from("test_attackable_entity"));
    let mut headless = HeadlessGame::new(world, camera);
    ok_or_panic!(headless.world.process_mouse_input(MousePosition {
            x_screen: 689.0,
            y_screen: 400.0,
            x_world: 689.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false));
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    ok_or_panic!(headless.run(200).await);
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
}
