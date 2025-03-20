#![cfg(test)]
use compact_str::CompactString;

use crate::game_engine::json_parsing::entity_archetype_json;
use crate::{create_stat_list, ok_or_panic};
use crate::game_engine::camera::Camera;
use crate::game_engine::item::Item;
use crate::game_engine::{entity_components, player};
use crate::game_engine::stat::StatC;
use crate::game_engine::world;
use crate::game_engine::world::World;

pub async fn basic_world() -> world::World {
    let mut sprites = crate::rendering_engine::abstractions::SpriteContainer::new();
    sprites.sprite_id_lookup.insert(CompactString::from("test_sprite"), 0);
    sprites.sprites.push(crate::rendering_engine::abstractions::Sprite{
        tex_x: 0.0, tex_y: 0.0, 
        tex_w: 1.0, tex_h: 1.0,
        texture_index: 0
    }
    );
    sprites.sprite_id_lookup.insert(CompactString::from("player_front"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("player_right"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("player_left"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("player_back"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("melee_attack"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("attack_highlight"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("sword"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("spear"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("slot_highlight"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("hslot"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("inventory"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("inventory_background"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("health_bar_back"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("health"), 0);
    sprites.sprite_id_lookup.insert(CompactString::from("level_editor_menu_background"), 0);
    let mut world = ok_or_panic!(world::World::new(player::Player::new(596.0, 400.0, 10.0, 10, 1.0, 0),sprites));
    world.inventory.add_item(Item {
        name: CompactString::from("test1"),
        attack_sprite: Some(CompactString::from("melee_attack")),
        item_type: crate::game_engine::item::ItemType::MeleeWeapon,
        width_to_length_ratio: None,
        lore: String::from("test"),
        sprite: CompactString::from("sword"),
        stats: create_stat_list!(
            damage => StatC {flat: 150.0, percent: 0.0},
            width => StatC {flat: 50.0, percent: 0.0},
            reach => StatC {flat: 65.0, percent: 0.0}
        ),
        time_til_usable: 0.0
    });
    world.inventory.add_item(Item {
        name: CompactString::from("test2"),
        attack_sprite: Some(CompactString::from("melee_attack")),
        item_type: crate::game_engine::item::ItemType::MeleeWeapon,
        width_to_length_ratio: None,
        lore: String::from("test"),
        sprite: CompactString::from("spear"),
        stats: create_stat_list!(
            damage => StatC {flat: 150.0, percent: 0.0},
            width => StatC {flat: 50.0, percent: 0.0},
            reach => StatC {flat: 65.0, percent: 0.0}
        ), 
        time_til_usable: 0.0
    });
    world

}
pub async fn basic_camera(world: &mut World) -> Camera {
    let camera = Camera::new(1152,720);
    world.inventory.init_ui();
    camera
}

pub async fn basic_entity() -> entity_archetype_json {
    entity_archetype_json {
        basic_tags: vec!["aggressive".into(), "hasCollision".into(), "attacker".into(), "damageable".into(), "respects_collision".into()],
        name: "Test".into(),
        collision_box: Some(entity_components::CollisionBox {
            w: 32.0,
            h: 32.0,
            x_offset: 0.0,
            y_offset: 0.0
        }),
        damage_box: Some(entity_components::CollisionBox {
            w: 32.0,
            h: 32.0,
            x_offset: 0.0,
            y_offset: 0.0
        }),
        health: Some(10),
        monster_type: CompactString::new("Undead"),
        movement_speed: Some(2.5),
        range: Some(47),
        aggro_range: Some(1000),
        attack_type: CompactString::new("Melee"),
        attack_pattern: Some("test".into()),
        loot_table: vec![],
        sprite: Some("test_sprite".into())
    }
}
