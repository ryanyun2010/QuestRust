use compact_str::CompactString;

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
    
    for entity_descriptor in starting_level_descriptor.entities.iter(){
        let entity = world.create_entity_with_archetype(entity_descriptor.x, entity_descriptor.y, entity_descriptor.archetype.clone());
        world.set_sprite(entity, punwrap!(world.sprites.get_sprite_id(&entity_descriptor.sprite), Invalid, "could not find sprite {} while generating world from json data", &entity_descriptor.sprite));
    }
    for terrain_json in starting_level_descriptor.terrain.iter(){
        let start_x = terrain_json.x;
        let start_y = terrain_json.y;
        let width = terrain_json.width;
        let height = terrain_json.height;
        let descriptor = punwrap!(data.get_terrain_archetype(&terrain_json.terrain_archetype), Invalid, "could not find terrain archetype {} while generating world from json data", &terrain_json.terrain_archetype);
        let tags = descriptor.basic_tags.clone();
        let archetype = world.add_terrain_archetype(ptry!(match_terrain_tags(&descriptor.basic_tags)));
        match descriptor.r#type.as_str() {
            "basic" => {
                for x in start_x..start_x + width{
                    for y in start_y..start_y + height{
                        let terrain = world.add_terrain(x * 32, y * 32);
                        world.set_sprite(terrain, punwrap!(world.sprites.get_sprite_id(&descriptor.sprites[0]), Invalid, "Could not find sprite: {} while generating world from json data", descriptor.sprites[0]));
                        world.set_terrain_archetype(terrain, archetype);
                    }
                }
            },
            "randomness" => {
                let random_chances = punwrap!(descriptor.random_chances.clone(), Invalid, "Terrain with type 'randomness' must have a random_chances vec");
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
                                world.set_sprite(terrain, punwrap!(world.sprites.get_sprite_id(&descriptor.sprites[index]), Invalid, "Could not find sprite: {} while generating world from json data", descriptor.sprites[index]));
                                world.set_terrain_archetype(terrain, archetype);
                                break;
                            }
                        };
                    }
                }
            },
            _ => {
                return Err(perror!(Invalid, "Found unknown terrain type: {} while generating world from json data", descriptor.r#type));
            }
        }
    }
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
