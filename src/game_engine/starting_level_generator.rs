use compact_str::CompactString;

use rand::prelude::*;
use crate::game_engine::terrain::TerrainTags;
use crate::game_engine::world::World;
use crate::json_parsing::ParsedData;
use crate::game_engine::player::Player;
use crate::{perror, ptry, punwrap};
use crate::error::PError;

pub fn generate_world_from_json_parsed_data(data: &ParsedData) -> Result<World, PError> {


    let starting_level_descriptor = data.starting_level_descriptor.clone();
    let player_descriptor = starting_level_descriptor.player;
    let mut world = ptry!(World::new(Player::new(player_descriptor.x, player_descriptor.y, player_descriptor.health, player_descriptor.max_health, player_descriptor.movement_speed, data.sprites.get_sprite_id("player_front").expect("Couldn't find player_front sprite")), data.sprites.clone()), "while generating world from json data");
    world.item_archetype_lookup = data.item_archetypes.clone();
    world.loot_table_lookup = data.loot_table_lookup.clone();

    for archetype in data.entity_archetypes.iter(){
        world.add_entity_archetype(archetype.0.clone(), archetype.1.clone());
    }
    for attack in data.entity_attacks.iter(){
        world.entity_attack_descriptor_lookup.insert(
            attack.0.clone(),
            attack.1.clone()
        );
    }

    world.entity_attack_pattern_lookup = data.entity_attack_patterns.clone();
    for entity_descriptor in starting_level_descriptor.entities.iter(){
        ptry!(world.create_entity_with_archetype(entity_descriptor.x, entity_descriptor.y, entity_descriptor.archetype.clone()));
    }
    world.terrain_archetype_jsons = data.terrain_archetypes.clone();

    for archetype in data.terrain_archetypes.iter(){
        world.add_terrain_archetype(archetype.0.clone(), ptry!(match_terrain_tags(&archetype.1.basic_tags), "while generating world from json data"));
    }
    for terrain_json in starting_level_descriptor.terrain.iter(){
        ptry!(world.generate_terrain_from_descriptor(terrain_json, 0, 0));
    }
    world.room_descriptors = data.rooms.clone();
    world.spawn_archetype_descriptors = data.spawn_archetypes.clone();

    Ok(world)
}




pub fn match_terrain_tags (tags: &Vec<CompactString>) -> Result<Vec<TerrainTags>, PError> {
    let mut tags_ = Vec::new();
    for tag in tags{
        match tag.as_str(){
            "blocksMovement" => {
                tags_.push(TerrainTags::BlocksMovement);
            },
            _ => {
                return Err(perror!(Invalid, "Found unknown terrain tag: {}", tag));
            }
        }
    }
    Ok(tags_)
}


pub fn generate_room(world: &mut World, room: CompactString, x: usize, y: usize) -> Result<(), PError> {
    let room_descriptor = punwrap!(world.room_descriptors.get(&room), NotFound, "Could not find room {}", room).clone();
    let spawn_archetype = punwrap!(world.spawn_archetype_descriptors.get(&room_descriptor.spawn_archetype), Invalid, "Room {} refers to spawn archetype {} but there is no spawn archetype with name {}", room, room_descriptor.spawn_archetype, room_descriptor.spawn_archetype).clone();

    for terrain in room_descriptor.terrain.iter(){
        ptry!(world.generate_terrain_from_descriptor(terrain, x as i32, y as i32));
    }

    let mut cur_points = 0;
    let mut rng = rand::thread_rng();

    while cur_points < spawn_archetype.total_points_to_spawn && !room_descriptor.spawnable.is_empty() {
        let mut options = Vec::new();
        for option in spawn_archetype.basic.iter() {
            if option.points + option.points <= spawn_archetype.total_points_to_spawn {
                options.push(option);
            }
        }
        if options.is_empty() {
            break;
        }
        let choice = if options.len() == 1 {
            options[0]
        } else {
            options[rng.gen_range(0..options.len() - 1)]
        };
        let position = if room_descriptor.spawnable.len() == 1 {
            room_descriptor.spawnable[0]
        } else {
            room_descriptor.spawnable[rng.gen_range(0..room_descriptor.spawnable.len() - 1)]
        };
        let real_position = [position[0] as f32 * 32.0 + x as f32 * 32.0, position[1] as f32 * 32.0 + y as f32 * 32.0];
        world.create_entity_with_archetype(real_position[0], real_position[1], choice.archetype.clone());
        cur_points += choice.points;
    }
    
    Ok(())
}
