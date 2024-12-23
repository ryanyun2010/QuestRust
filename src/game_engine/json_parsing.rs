use serde::Deserialize;
use std::hash::Hash;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use crate::game_engine::entities::EntityTags;
use crate::game_engine::entities::EntityAttack;
use crate::rendering_engine::abstractions::Sprite;
use super::entities::EntityAttackPattern;

#[derive(Debug, Deserialize)]
struct entity_archetype_json {
    name: String,
    basic_tags: Vec<String>,
    monster_type: String,
    movement_speed: f32,
    range: usize,
    aggro_range: usize,
    attack_type: String,
    attack_pattern: String
}
#[derive(Debug, Deserialize)]
struct entity_attack_pattern_json {
    name: String,
    attacks: Vec<String>,
    cooldowns: Vec<f32>
}
#[derive(Debug, Deserialize)]
struct entity_attack_json{
    name: String,
    damage: f32,
}
#[derive(Debug, Deserialize)]
pub struct sprite_json {
    name: String,
    path: String,
}

pub struct JSON_parser {
    pub entity_archetypes_json: HashMap<String, entity_archetype_json>,
    pub entity_attack_patterns_json: HashMap<String, entity_attack_pattern_json>,
    pub entity_attacks_json: HashMap<String, entity_attack_json>,
    pub sprites_json: HashMap<String, sprite_json>,
}

impl JSON_parser {
    pub fn new() -> Self {
        Self {
            entity_archetypes_json: HashMap::new(),
            entity_attack_patterns_json: HashMap::new(),
            entity_attacks_json: HashMap::new(),
            sprites_json: HashMap::new(),
        }
    }
    pub fn parse_entity_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<entity_archetype_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for archetype in data {
            self.entity_archetypes_json.insert(archetype.name.clone(), archetype);
        }
    }
    pub fn parse_entity_attack_patterns(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<entity_attack_pattern_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for pattern in data {
            self.entity_attack_patterns_json.insert(pattern.name.clone(), pattern);
        }
    }
    pub fn parse_entity_attacks(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<entity_attack_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for attack in data {
            self.entity_attacks_json.insert(attack.name.clone(), attack);
        }
    }
    pub fn parse_sprites(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<sprite_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for sprite in data {
            self.sprites_json.insert(sprite.name.clone(), sprite);
        }
    }
    
    pub fn convert(&mut self) -> ParsedData {
        // Convert the JSON data into the game's data structures
        // Convert Entity Attacks First
        let mut data = ParsedData::new();
        for (name, entity_attack) in &self.entity_attacks_json {
            data.entity_attacks.insert(name.clone(), EntityAttack::new(entity_attack.damage));
        }
        for (name, entity_attack_pattern) in &self.entity_attack_patterns_json {
            let mut attacks = Vec::new();
            for attack in &entity_attack_pattern.attacks {
                attacks.push(data.entity_attacks.get(attack).unwrap().clone());
            }
            data.entity_attack_patterns.insert(name.clone(), EntityAttackPattern::new(attacks, entity_attack_pattern.cooldowns.clone()));
        }

        for (name, entity_archetype) in &self.entity_archetypes_json {
            let mut tags = Vec::new();
            for basic_tag in &entity_archetype.basic_tags {
                match basic_tag.as_str() {
                    "aggressive" => {
                        tags.push(EntityTags::Aggressive);
                    }
                    "respectsCollision" => {
                        tags.push(EntityTags::RespectsCollision);
                    }
                    "hasCollision" => {
                        tags.push(EntityTags::HasCollision);
                    }
                    "followsPlayer" => {
                        tags.push(EntityTags::FollowsPlayer);
                    }
                    _ => {
                        panic!("When parsing entity archetypes, basic tag: {} in archetype: {} was not recognized", basic_tag, entity_archetype.name);
                    }
                }
            }
            match entity_archetype.monster_type.as_str() {
                "Undead" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Undead));
                },
                "Uruk" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Uruk));
                },
                "Parasite" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Parasite));
                },
                "Beast" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Beast));
                },
                "Demon" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Demon));
                },
                "Dragon" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Dragon));
                },
                "Item" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Item));
                },
                "Ambient" => {
                    tags.push(EntityTags::MonsterType(crate::game_engine::entities::MonsterType::Ambient));
                },
                _ => {
                    panic!("When parsing entity archetypes, monster type: {} in archetype: {} was not recognized", entity_archetype.monster_type, entity_archetype.name);
                }
            }
            tags.push(EntityTags::MovementSpeed(entity_archetype.movement_speed));
            tags.push(EntityTags::Range(entity_archetype.range));
            tags.push(EntityTags::AggroRange(entity_archetype.aggro_range));
            match entity_archetype.attack_type.as_str() {
                "Melee" => {
                    tags.push(EntityTags::AttackType(crate::game_engine::entities::AttackType::Melee));
                },
                "Ranged" => {
                    tags.push(EntityTags::AttackType(crate::game_engine::entities::AttackType::Ranged));
                },
                "Magic" => {
                    tags.push(EntityTags::AttackType(crate::game_engine::entities::AttackType::Magic));
                },
                _ => {
                    panic!("When parsing entity archetypes, attack type: {} in archetype: {} was not recognized", entity_archetype.attack_type, entity_archetype.name);
                }
            }
            tags.push(EntityTags::Attacks(data.entity_attack_patterns.get(&entity_archetype.attack_pattern).expect(&format!("When parsing entity archetypes, attack pattern: {} in archetype: {} was not found", entity_archetype.attack_pattern, entity_archetype.name)).clone()));
            data.entity_archetypes.insert(entity_archetype.name.clone(), tags);
            
        }
        let mut sprites_to_load = Vec::new();
        let mut i = 0;
        for (_, sprite) in &self.sprites_json {
            sprites_to_load.push(sprite.path.clone());
            data.texture_ids.insert(sprite.name.clone(), i);
            i += 1;
        }
        data.sprites_to_load_json = sprites_to_load;

        data
    }
    pub fn parse_and_convert_game_data(&mut self, entity_archetypes_path: &str, entity_attack_patterns_path: &str, entity_attacks_path: &str, sprites_path: &str) -> ParsedData{
        self.parse_entity_archetypes(entity_archetypes_path);
        self.parse_entity_attack_patterns(entity_attack_patterns_path);
        self.parse_entity_attacks(entity_attacks_path);
        self.parse_sprites(sprites_path);
        self.convert()
    }
    
}


#[derive(Debug, Clone)]
pub struct ParsedData{
    pub entity_archetypes: HashMap<String, Vec<EntityTags>>,
    pub entity_attack_patterns: HashMap<String, EntityAttackPattern>,
    pub entity_attacks: HashMap<String, EntityAttack>,
    pub texture_ids: HashMap<String, i32>,
    pub sprites_to_load_json: Vec<String>
}

impl ParsedData{
    pub fn new() -> Self{
        Self{
            entity_archetypes: HashMap::new(),
            entity_attack_patterns: HashMap::new(),
            entity_attacks: HashMap::new(),
            texture_ids: HashMap::new(),
            sprites_to_load_json: Vec::new()
        }
    }
    pub fn get_archetype(&self, name: &str) -> Option<&Vec<EntityTags>> {
        self.entity_archetypes.get(name)
    }
    pub fn get_texture_id(&self, name: &str) -> i32 {
        self.texture_ids.get(name).expect(&format!("Texture with name: {} was not found", name)).clone()
    }
}