#![cfg(test)]

use crate::{game_engine::{entities::AttackType, json_parsing::{self, PathBundle}, pathfinding::EntityDirectionOptions}, ok_or_panic};
pub const TEST_PATH_BUNDLE: PathBundle = PathBundle{
    entity_archetypes_path: "src/tests/test_game_data/entity_archetypes.json",
    entity_attack_patterns_path: "src/tests/test_game_data/entity_attack_patterns.json",
    entity_attacks_path: "src/tests/test_game_data/entity_attacks.json",
    sprites_path: "src/tests/test_game_data/sprites.json",
    starting_level_path: "src/tests/test_game_data/starting_level.json",
    terrain_archetypes_path: "src/tests/test_game_data/terrain_archetypes.json",
    item_archetypes_path: "src/tests/test_game_data/items.json",
    loot_table_path: "src/tests/test_game_data/loot_tables.json",
};


#[tokio::test]
async fn json_parsing_test(){
    let mut parser = json_parsing::JSON_parser::new();
    let parsed_data = parser.parse_and_convert_game_data(TEST_PATH_BUNDLE);
    assert!(parsed_data.starting_level_descriptor.player.x == 596.0, "Player x should be 596.0");
    assert!(parsed_data.starting_level_descriptor.player.y == 400.0, "Player y should be 400.0");
    assert!(parsed_data.starting_level_descriptor.terrain.len() == 1, "There should be one terrain block");
    assert!(parsed_data.starting_level_descriptor.entities.len() == 1, "There should be one entity");
    assert!(parsed_data.entity_archetypes.len() == 1, "There should be one entity archetype");
    assert!(parsed_data.entity_attack_patterns.len() == 1, "There should be one entity attack pattern");
    assert!(parsed_data.entity_attacks.len() == 1, "There should be one entity attack");
    let archetype = parsed_data.entity_archetypes.get("ghost").expect("There should be a ghost archetype");
    let mut found_aggressive = false;
    let mut found_follows_player = false;
    let mut found_movement_speed = false;
    let mut found_range = false;
    let mut found_aggro_range = false;
    let mut found_attack = false;
    let mut found_attack_type = false;
    let mut found_monster_type = false;
    let mut found_health = false;
    for tag in archetype.iter(){
        match tag{
            crate::game_engine::entities::EntityTags::BaseHealth(health) => {   
                assert!(!found_health, "BaseHealth tag should only be found once");
                found_health = true;
                assert!(*health == 100, "BaseHealth should be 100");
            }
            crate::game_engine::entities::EntityTags::Aggressive => {
                assert!(!found_aggressive, "Aggressive tag should only be found once");
                found_aggressive = true;
            },
            crate::game_engine::entities::EntityTags::FollowsPlayer => {
                assert!(!found_follows_player, "FollowsPlayer tag should only be found once");
                found_follows_player = true;
            },
            crate::game_engine::entities::EntityTags::MovementSpeed(speed) => {
                assert!(!found_movement_speed, "MovementSpeed tag should only be found once");
                found_movement_speed = true;
                assert!(*speed == 2.0, "Movement speed should be 2.0");
            },
            crate::game_engine::entities::EntityTags::Range(range) => {
                assert!(!found_range, "Range tag should only be found once");
                found_range = true;
                assert!(*range == 47, "Range should be 47");
            },
            crate::game_engine::entities::EntityTags::AggroRange(aggro_range) => {
                assert!(!found_aggro_range, "AggroRange tag should only be found once");
                found_aggro_range = true;
                assert!(*aggro_range == 1000, "AggroRange should be 1000");
            },
            crate::game_engine::entities::EntityTags::Attacks(entity_attack_pattern) => {
                assert!(!found_attack, "Attack Pattern tag should only be found once");
                found_attack = true;
                let attack_pattern = entity_attack_pattern.clone();
                assert!(attack_pattern.attacks.len() == 1, "There should be one attack in the attack pattern");
                assert!(attack_pattern.attack_cooldowns.len() == 1, "There should be one attack cooldown in the attack pattern");
                assert!(attack_pattern.attack_cooldowns[0] == 0.1, "Attack cooldown should be 0.1");
            },
            crate::game_engine::entities::EntityTags::AttackType(attack_type) => {
                assert!(!found_attack_type, "AttackType tag should only be found once");
                found_attack_type = true;
                assert!(*attack_type == AttackType::Melee, "Attack type should be melee");
            },
            crate::game_engine::entities::EntityTags::MonsterType(monster_type) => {
                assert!(!found_monster_type, "MonsterType tag should only be found once");
                found_monster_type = true;
                assert!(*monster_type == crate::game_engine::entities::MonsterType::Undead, "Monster type should be ghost");
            },
            _ => {}
        }
    }
    assert!(found_aggressive, "Aggressive tag should be found");
    assert!(found_follows_player, "FollowsPlayer tag should be found");
    assert!(found_movement_speed, "MovementSpeed tag should be found");
    assert!(found_range, "Range tag should be found");
    assert!(found_aggro_range, "AggroRange tag should be found");
    assert!(found_attack, "Attack tag should be found");
    assert!(found_attack_type, "AttackType tag should be found");
    assert!(found_monster_type, "MonsterType tag should be found");
    let terrain_archetype = parsed_data.terrain_archetypes.get("basic_outside").expect("There should be a basic terrain archetype");
    assert!(terrain_archetype.name == "basic_outside", "Terrain archetype name should be basic_outside");
    assert!(terrain_archetype.r#type == "basic", "Terrain archetype type should be basic");
    assert!(terrain_archetype.basic_tags.len() == 1, "There should be one basic tags");
    assert!(terrain_archetype.basic_tags[0] == "blocksMovement", "The one basic tag should be blocksMovement");
    assert!(terrain_archetype.sprites.len() == 1, "There should be one sprite");
    assert!(terrain_archetype.sprites[0] == "outside", "The one sprite should be outside");

    let entity = parsed_data.starting_level_descriptor.entities.first().expect("There should be an entity");
    assert!(entity.x == 900.0, "Entity x should be 900.0");
    assert!(entity.y == 405.0, "Entity y should be 405.0");
    assert!(entity.archetype == "ghost", "Entity archetype should be ghost");
    assert!(entity.sprite == "ghost", "Entity sprite should be ghost");
    let terrain = parsed_data.starting_level_descriptor.terrain.first().expect("There should be a terrain");
    assert!(terrain.x == 0, "Terrain x should be 0");
    assert!(terrain.y == 0, "Terrain y should be 0");
    assert!(terrain.width == 1, "Terrain width should be 1");
    assert!(terrain.height == 1, "Terrain height should be 1");
    assert!(terrain.terrain_archetype == "basic_outside", "Terrain archetype should be basic_outside");
}

#[tokio::test]
async fn world_generation_test(){
    let mut parser = json_parsing::JSON_parser::new();
    let parsed_data = parser.parse_and_convert_game_data(TEST_PATH_BUNDLE);
    let world = ok_or_panic!(crate::game_engine::starting_level_generator::generate_world_from_json_parsed_data(&parsed_data));
    assert!(world.player.borrow().x == 596.0, "Player x should be 596.0");
    assert!(world.player.borrow().y == 400.0, "Player y should be 400.0");
    assert!(world.player.borrow().health == 100.0, "Player health should be 100.0");
    assert!(world.player.borrow().max_health == 100, "Player max health should be 100");
    assert!(world.player.borrow().movement_speed == 3.5, "Player movement speed should be 3.5");
    let player_sprite_id_expected = world.sprites.get_sprite_id("player_front").expect("There should be a player sprite");
    assert!(world.player.borrow().sprite_id == player_sprite_id_expected, "The player should have the player_front sprite");
    assert!(world.terrain.len() == 1, "There should be one terrain block");

    let chunk_id = world.get_chunk_from_chunk_xy(0, 0).expect("There should be a chunk at 0,0");
    let chunks_ref = world.chunks.borrow();
    let chunk = chunks_ref.get(chunk_id).expect("There should be a chunk at 0,0");
    assert!(chunk.entities_ids.len() == 1, "There should be one entity in the chunk");
    assert!(chunk.terrain_ids.len() == 1, "There should be one terrain in the chunk");
    let entity_id = chunk.entities_ids[0];
    let enitty_position = *world.entity_position_components.get(&entity_id).expect("There should be an entity position component").borrow();
    assert!(enitty_position.x == 900.0, "Entity x should be 900.0");
    assert!(enitty_position.y == 405.0, "Entity y should be 405.0");
    let entity_collision_box = world.get_entity_collision_box(entity_id).expect("There should be an entity collision box component");
    assert!(entity_collision_box.w == 32.0, "Entity collision box width should be 32.0");
    assert!(entity_collision_box.h == 32.0, "Entity collision box height should be 32.0");
    assert!(entity_collision_box.x_offset == 0.0, "Entity collision box x offset should be 0.0");
    assert!(entity_collision_box.y_offset == 0.0, "Entity collision box y offset should be 0.0");
    let entity_pathfinding = *world.entity_pathfinding_components.get(&entity_id).expect("There should be an entity pathfinding component").borrow();
    assert!(entity_pathfinding.cur_direction ==EntityDirectionOptions::None,  "Entity direction should be none priot to the update of the world");
    let entity_attack = *world.entity_attack_components.get(&entity_id).expect("There should be an entity attack component").borrow();
    assert!(entity_attack.cur_attack == 0, "The entity should start at attack 0");
    assert!(entity_attack.cur_attack_cooldown == 0.0, "The entity should start with a cooldown of 0.0");
    let entity_archetype = world.get_entity_archetype(&entity_id).expect("There should be an entity archetype").clone();
    assert!(entity_archetype == "ghost", "The entity archetype should be ghost");
    let entity_tags = world.get_entity_tags(entity_id).expect("There should be entity tags").clone();
    let mut found_aggressive = false;
    let mut found_follows_player = false;
    let mut found_movement_speed = false;
    let mut found_range = false;
    let mut found_aggro_range = false;
    let mut found_attack = false;
    let mut found_attack_type = false;
    let mut found_monster_type = false;
    for tag in entity_tags.iter(){
        match tag{
            crate::game_engine::entities::EntityTags::Aggressive => {
                assert!(!found_aggressive, "Aggressive tag should only be found once");
                found_aggressive = true;
            },
            crate::game_engine::entities::EntityTags::FollowsPlayer => {
                assert!(!found_follows_player, "FollowsPlayer tag should only be found once");
                found_follows_player = true;
            },
            crate::game_engine::entities::EntityTags::MovementSpeed(speed) => {
                assert!(!found_movement_speed, "MovementSpeed tag should only be found once");
                found_movement_speed = true;
                assert!(*speed == 2.0, "Movement speed should be 2.0");
            },
            crate::game_engine::entities::EntityTags::Range(range) => {
                assert!(!found_range, "Range tag should only be found once");
                found_range = true;
                assert!(*range == 47, "Range should be 47");
            },
            crate::game_engine::entities::EntityTags::AggroRange(aggro_range) => {
                assert!(!found_aggro_range, "AggroRange tag should only be found once");
                found_aggro_range = true;
                assert!(*aggro_range == 1000, "AggroRange should be 1000");
            },
            crate::game_engine::entities::EntityTags::Attacks(entity_attack_pattern) => {
                assert!(!found_attack, "Attack Pattern tag should only be found once");
                found_attack = true;
                let attack_pattern = entity_attack_pattern.clone();
                assert!(attack_pattern.attacks.len() == 1, "There should be one attack in the attack pattern");
                assert!(attack_pattern.attack_cooldowns.len() == 1, "There should be one attack cooldown in the attack pattern");
                assert!(attack_pattern.attack_cooldowns[0] == 0.1, "Attack cooldown should be 0.1");
                let attack = world.get_attack_descriptor_by_name(&attack_pattern.attacks[0]).expect("There should be an attack descriptor");
                assert!(attack.damage == 150.0, "Attack damage should be 150.0");
                assert!(attack.reach == 50, "Attack reach should be 50");
                assert!(attack.width == 50, "Attack width should be 50");
                assert!(attack.time_to_charge == 5, "Attack time to charge should be 5");
                assert!(attack.sprite == "attack_highlight", "Attack sprite should be attack_highlight");


            },
            crate::game_engine::entities::EntityTags::AttackType(attack_type) => {
                assert!(!found_attack_type, "AttackType tag should only be found once");
                found_attack_type = true;
                assert!(*attack_type == AttackType::Melee, "Attack type should be melee");
            },
            crate::game_engine::entities::EntityTags::MonsterType(monster_type) => {
                assert!(!found_monster_type, "MonsterType tag should only be found once");
                found_monster_type = true;
                assert!(*monster_type == crate::game_engine::entities::MonsterType::Undead, "Monster type should be ghost");
            },
            _ => {}
        }
    }
    assert!(found_aggressive, "Aggressive tag should be found");
    assert!(found_follows_player, "FollowsPlayer tag should be found");
    assert!(found_movement_speed, "MovementSpeed tag should be found");
    assert!(found_range, "Range tag should be found");
    assert!(found_aggro_range, "AggroRange tag should be found");
    assert!(found_attack, "Attack tag should be found");
    assert!(found_attack_type, "AttackType tag should be found");
    assert!(found_monster_type, "MonsterType tag should be found"); 
    let ghost_sprite_id_expected = world.sprites.get_sprite_id("ghost").expect("There should be a ghost sprite");
    let ghost_sprite_id = world.sprite_lookup.get(&entity_id).expect("There should be a sprite id for the entity");
    assert!(*ghost_sprite_id == ghost_sprite_id_expected, "The entity should have the ghost sprite");
    let terrain_id = chunk.terrain_ids[0];
    let terrain = world.terrain.get(&terrain_id).expect("There should be a terrain");
    assert!(terrain.x == 0, "Terrain x should be 0");
    assert!(terrain.y == 0, "Terrain y should be 0");
    let terrain_sprite_id_expected = world.sprites.get_sprite_id("outside").expect("There should be an outside sprite");
    let terrain_sprite_id = world.sprite_lookup.get(&terrain_id).expect("There should be a sprite id for the terrain");
    assert!(*terrain_sprite_id == terrain_sprite_id_expected, "The terrain should have the outside sprite");
    let terrain_tags = world.get_terrain_tags(terrain_id).expect("There should be terrain tags").clone();
    assert!(terrain_tags.len() == 1, "There should be one terrain tag");
    assert!(terrain_tags[0] == crate::game_engine::terrain::TerrainTags::BlocksMovement, "The one terrain tag should be BlocksMovement");

}
