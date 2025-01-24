use core::panic;

use crate::game_engine::terrain::TerrainTags;
use crate::game_engine::world::World;
use crate::json_parsing::ParsedData;
use crate::game_engine::player::Player;

pub fn generate_world_from_json_parsed_data(data: &ParsedData) -> World {

    
    let starting_level_descriptor = data.starting_level_descriptor.clone();
    let player_descriptor = starting_level_descriptor.player;
    let mut world = World::new(Player::new(player_descriptor.x, player_descriptor.y, player_descriptor.health, player_descriptor.max_health, player_descriptor.movement_speed, data.sprites.get_sprite_id("player_front").expect("Couldn't find player_front sprite")), data.sprites.clone());
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
    
    for entity_descriptor in starting_level_descriptor.entities.iter(){
        let entity = world.create_entity_with_archetype(entity_descriptor.x, entity_descriptor.y, entity_descriptor.archetype.clone());
        world.set_sprite(entity, world.sprites.get_sprite_id(&entity_descriptor.sprite).expect(format!("Could not find sprite: {}", entity_descriptor.sprite).as_str()));
    }
    for terrain_json in starting_level_descriptor.terrain.iter(){
        let start_x = terrain_json.x;
        let start_y = terrain_json.y;
        let width = terrain_json.width;
        let height = terrain_json.height;
        let descriptor = data.get_terrain_archetype(&terrain_json.terrain_archetype).expect(format!("Could not find terrain archetype: {}", terrain_json.terrain_archetype).as_str());
        let tags = descriptor.basic_tags.clone();
        let archetype = world.add_terrain_archetype(match_terrain_tags(&descriptor.basic_tags));
        match descriptor.r#type.as_str() {
            "basic" => {
                for x in start_x..start_x + width{
                    for y in start_y..start_y + height{
                        let terrain = world.add_terrain(x * 32, y * 32);
                        world.set_sprite(terrain, world.sprites.get_sprite_id(&descriptor.sprites[0]).expect(format!("Could not find sprite: {}", descriptor.sprites[0]).as_str()));
                        world.set_terrain_archetype(terrain, archetype);
                    }
                }
            },
            "randomness" => {
                let random_chances = descriptor.random_chances.clone().expect("Randomness terrain must have random_chances");
                let mut random_chances_adjusted = Vec::new();
                let mut sum_so_far = 0.0;
                for chance in random_chances{
                    random_chances_adjusted.push(chance + sum_so_far);
                    sum_so_far += chance;
                }
                for x in start_x..start_x + width{
                    for y in start_y..start_y + height{
                        let terrain = world.add_terrain(x * 32, y * 32);
                        let random_number = rand::random::<f32>();
                        for (index, chance) in random_chances_adjusted.iter().enumerate(){
                            if random_number < *chance{
                                world.set_sprite(terrain, world.sprites.get_sprite_id(&descriptor.sprites[index]).expect(format!("Could not find sprite: {}", descriptor.sprites[index]).as_str()));
                                world.set_terrain_archetype(terrain, archetype);
                                break;
                            }
                        };
                    }
                }
            },
            _ => {
                panic!("Unknown terrain type: {}", descriptor.r#type);
            }
        }
    }
    world
}


pub fn match_terrain_tags (tags: &Vec<String>) -> Vec<TerrainTags> {
    let mut tags_ = Vec::new();
    for tag in tags{
        match tag.as_str(){
            "blocksMovement" => {
                tags_.push(TerrainTags::BlocksMovement);
            },
            _ => {
                panic!("Unknown terrain tag: {}", tag);
            }
        }
    }
    tags_
}