
#![cfg(test)]

use crate::{create_stat_list, game_engine::{entity_components::CollisionBox, game::MousePosition, item::{Item, ItemArchetype, ItemType}, loot::{LootTable, LootTableEntry}, stat::{GearStatList, StatC}}, ok_or_panic, tests::{lib::headless::HeadlessGame, test_framework::{basic_camera, basic_world}}};
#[tokio::test]
pub async fn test_enemy_item_drops() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.item_archetype_lookup.insert("test_item".to_string(), ItemArchetype {
        name: "test_item".to_string(),
        stats: GearStatList::default(),
        lore: "d".to_string(),
        item_type: ItemType::MeleeWeapon,
        width_to_length_ratio: None,
        sprite: "spear".to_string(),
        attack_sprite: Some("attack_highlight".to_string())
    });
    world.loot_table_lookup = vec![
        LootTable::new(vec![LootTableEntry {
            item: Some("test_item".to_string()),
            weight: 10
        }])
    ];
    world.add_entity_archetype(String::from("test_attackable_entity"), vec![
        crate::game_engine::entities::EntityTags::BaseHealth(100),
        crate::game_engine::entities::EntityTags::Damageable(
            CollisionBox {
                x_offset: 0.0,
                y_offset: 0.0,
                w: 32.0,
                h: 32.0
            }
        ),
        crate::game_engine::entities::EntityTags::Drops(vec![0])
    ]);
    let item = world.inventory.add_item(
        Item {
            name: String::from("test_sword"),
            attack_sprite: Some(String::from("melee_attack")),
            item_type: ItemType::MeleeWeapon,
            width_to_length_ratio: None,
            lore: String::from("test"),
            sprite: String::from("sword"),
            stats: create_stat_list!(
                damage => StatC { flat: 150.0, percent: 0.0},
                width => StatC { flat: 50.0, percent: 0.0},
                reach => StatC { flat: 65., percent: 0.0},
            ),
            time_til_usable: 0.0
        }
    );
    ok_or_panic!(world.inventory.set_hotbar_slot_item(0, item)); 
    world.create_entity_with_archetype(639.0, 400.0, String::from("test_attackable_entity"));
    let mut headless = HeadlessGame::new(world, camera);
    ok_or_panic!(headless.world.on_mouse_click(MousePosition {
            x_screen: 639.0,
            y_screen: 400.0,
            x_world: 639.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false, headless.camera.viewpoint_width as f32, headless.camera.viewpoint_height as f32));
    assert!(
        headless.world.inventory.get_hotbar_slot(1).unwrap().item.is_none(),
        "Player should not pick up an item prior to killing enemy"
    );
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    ok_or_panic!(headless.run(500).await);
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
    assert!(
        headless.world.inventory.get_hotbar_slot(1).unwrap().item.is_some(),
        "Player should pick up an item"
    )

}
