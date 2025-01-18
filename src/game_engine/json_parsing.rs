use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter, Write};
use std::fs::File;
use std::collections::HashMap;
use crate::game_engine::entities::{EntityTags, EntityAttackPattern};
use crate::rendering_engine::abstractions::SpriteContainer;

use super::entities::AttackType;
use super::entity_attacks::EntityAttackDescriptor;
use super::entity_components::CollisionBox;
use super::item::{ItemArchetype, ItemType};
use super::stat::GearStatList;


pub struct PathBundle{
    pub entity_archetypes_path: &'static str,
    pub entity_attack_patterns_path: &'static str,
    pub entity_attacks_path: &'static str,
    pub terrain_archetypes_path: &'static str,
    pub sprites_path: &'static str,
    pub starting_level_path: &'static str,
    pub item_archetypes_path: &'static str
}

pub const PATH_BUNDLE: PathBundle = PathBundle{
    entity_archetypes_path: "src/game_data/entity_archetypes.json",
    entity_attack_patterns_path: "src/game_data/entity_attack_patterns.json",
    entity_attacks_path: "src/game_data/entity_attacks.json",
    terrain_archetypes_path: "src/game_data/terrain_archetypes.json",
    sprites_path: "src/game_data/sprites.json",
    starting_level_path: "src/game_data/starting_level.json",
    item_archetypes_path: "src/game_data/items.json"
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_attack_descriptor_json {
    pub name: String,
    pub r#type: String,
    pub damage: f32,
    pub reach: usize,
    pub max_start_dist_from_entity: Option<usize>,
    pub width: usize,
    pub time_to_charge: usize,
    pub sprite: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_archetype_json {
    pub name: String,
    pub basic_tags: Vec<String>,
    pub collision_box: Option<CollisionBox>,
    pub damage_box: Option<CollisionBox>,
    pub health: usize,
    pub monster_type: String,
    pub movement_speed: f32,
    pub range: usize,
    pub aggro_range: usize,
    pub attack_type: String,
    pub attack_pattern: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_attack_pattern_json {
    name: String,
    attacks: Vec<String>,
    cooldowns: Vec<f32>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprites_json_descriptor {
    pub basic_sprites: Vec<sprite_json>,
    pub spritesheets: Vec<sprite_sheet_json>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprite_sheet_json{
    pub name: String,
    pub path: String,
    pub width: usize,
    pub height: usize,
    pub sprite_width: usize,
    pub sprite_height: usize,
    pub sprite_padding: usize,
    pub sprites: Vec<sprite_sheet_sprite_json>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprite_sheet_sprite_json {
    pub name: String,
    pub x: usize,
    pub y: usize
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprite_json {
    pub name: String,
    pub path: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct player_json{
    pub x: f32,
    pub y: f32,
    pub sprite: String,
    pub health: f32,
    pub max_health: i32,
    pub movement_speed: f32
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct entity_json{
    pub x: f32,
    pub y: f32,
    pub archetype: String,
    pub sprite: String,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct terrain_json{
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub terrain_archetype: String
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct terrain_archetype_json{
    pub name: String,
    pub r#type: String,
    pub random_chances: Option<Vec<f32>>,
    pub sprites: Vec<String>,
    pub basic_tags: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct starting_level_json{
    pub player: player_json,
    pub entities: Vec<entity_json>,
    pub terrain: Vec<terrain_json>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct item_json {

}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct player_projectile_descriptor_json{
    pub name: String,
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub AOE: f32,
    pub size: f32,
    pub sprite: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct melee_attack_descriptor_json{
    pub name: String,
    pub damage: f32,
    pub width: f32,
    pub reach: f32,
    pub lifetime: f32,
    pub sprite: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct player_attacks_descriptor_json {
    pub ranged_projectiles: Vec<player_projectile_descriptor_json>,
    pub melee_attacks: Vec<melee_attack_descriptor_json>
}
impl player_attacks_descriptor_json {
    pub fn new() -> Self {
        Self {
            ranged_projectiles: Vec::new(),
            melee_attacks: Vec::new()
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct item_archetype_json {
    pub name: String,
    pub sprite: String,
    pub attack_sprite: String,
    pub item_type: ItemType,
    pub lore: String,
    pub stats: GearStatList
}



#[derive(Clone)]
pub struct JSON_parser {
    pub entity_archetypes_json: HashMap<String, entity_archetype_json>,
    pub entity_attack_patterns_json: HashMap<String, entity_attack_pattern_json>,
    pub entity_attacks_json: HashMap<String, entity_attack_descriptor_json>,
    pub terrain_archetypes_json: HashMap<String, terrain_archetype_json>,
    pub sprites_json: sprites_json_descriptor,
    pub starting_level_json: starting_level_json,
    pub item_archetype_json: Vec<item_archetype_json>
}

#[macro_export]
macro_rules! from_JSON_entity_tag_parsing_basic {
    ($output:ident, $tag_list:expr) => {
        from_JSON_entity_tag_parsing_under! [$output, $tag_list;
            "aggressive", Aggressive,
            "respectsCollision", RespectsCollision,
            "followsPlayer", FollowsPlayer
        ]
    }
}
#[macro_export]
macro_rules! from_JSON_entity_tag_parsing_under {
    ($output:ident, $tag_list:expr; $($string_list:expr, $id_list:ident),*) => {
        for current_tag in $tag_list {
            match current_tag.as_str() {
                $(
                    $string_list => {
                        $output.push(EntityTags::$id_list);
                    }
                )*
                _ => {
                    panic!("When parsing entity archetypes, tag: {} was not recognized", current_tag);
                }
            }
        }
    };
}

impl JSON_parser {
    pub fn new() -> Self {
        Self {
            entity_archetypes_json: HashMap::new(),
            entity_attack_patterns_json: HashMap::new(),
            terrain_archetypes_json: HashMap::new(),
            entity_attacks_json: HashMap::new(),
            sprites_json: sprites_json_descriptor {
                basic_sprites: Vec::new(),
                spritesheets: Vec::new()
            },
            item_archetype_json: Vec::new(),
            starting_level_json: starting_level_json {
                player: player_json {
                    x: 0.0,
                    y: 0.0,
                    sprite: String::from(""),
                    health: 0.0,
                    max_health: 0,
                    movement_speed: 0.0
                },
                entities: Vec::new(),
                terrain: Vec::new()
            },
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
    pub fn parse_terrain_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<terrain_archetype_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for archetype in data {
            self.terrain_archetypes_json.insert(archetype.name.clone(), archetype);
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
        let data: Vec<entity_attack_descriptor_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for attack in data {
            self.entity_attacks_json.insert(attack.name.clone(), attack);
        }
    }
    pub fn parse_sprites(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: sprites_json_descriptor = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        self.sprites_json = data;
    }
    pub fn parse_starting_level(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: starting_level_json = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        self.starting_level_json = data;
    }

    pub fn parse_item_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let data: Vec<item_archetype_json> = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        self.item_archetype_json = data;
    }
    
    pub fn convert(&self) -> ParsedData {
        // Convert the JSON data into the game's data structures
        // Convert Entity Attacks First
        let mut data = ParsedData::new();
        for (name, entity_attack) in &self.entity_attacks_json {
            let t = match entity_attack.r#type.as_str() {
                "melee" => AttackType::Melee,
                "ranged" => AttackType::Ranged,
                "magic" => AttackType::Magic,
                _ => {
                    panic!("When parsing entity attacks, type: {} in attack: {} was not recognized", entity_attack.r#type, name);
                }
            };
            data.entity_attacks.insert(name.clone(), EntityAttackDescriptor {
                r#type: t,
                damage: entity_attack.damage,
                reach: entity_attack.reach,
                max_start_dist_from_entity: entity_attack.max_start_dist_from_entity,
                width: entity_attack.width,
                time_to_charge: entity_attack.time_to_charge,
                sprite: entity_attack.sprite.clone()
            });
        }
        for (name, entity_attack_pattern) in &self.entity_attack_patterns_json {
            data.entity_attack_patterns.insert(name.clone(), EntityAttackPattern::new(entity_attack_pattern.attacks.clone(), entity_attack_pattern.cooldowns.clone()));
        }

        for (.., entity_archetype) in &self.entity_archetypes_json {
            let tags = self.convert_archetype(&entity_archetype, &data);
            data.entity_archetypes.insert(entity_archetype.name.clone(), tags);
            
        }
        (data.sprites_to_load_json, data.sprites) = SpriteContainer::create_from_json(&self.sprites_json);
        data.starting_level_descriptor = self.starting_level_json.clone();

        for (.., terrain_archetype) in &self.terrain_archetypes_json {
            data.terrain_archetypes.insert(terrain_archetype.name.clone(), terrain_archetype.clone());
        }

        for (item_archetype) in &self.item_archetype_json {
            data.item_archetypes.insert(item_archetype.name.clone(), ItemArchetype{
                name: item_archetype.name.clone(),
                stats: item_archetype.stats.clone(),
                lore: item_archetype.lore.clone(),
                item_type: item_archetype.item_type.clone(),
                sprite: item_archetype.sprite.clone(),
                attack_sprite: item_archetype.attack_sprite.clone()
        });
        }

        data
    }
    pub fn convert_archetype(&self, entity_archetype: &entity_archetype_json, data: &ParsedData) -> Vec<EntityTags> {
        let mut tags = Vec::new();
        from_JSON_entity_tag_parsing_basic!(tags, &entity_archetype.basic_tags);
        if entity_archetype.collision_box.is_some() {
            tags.push(EntityTags::HasCollision(entity_archetype.collision_box.clone().unwrap()));
        }
        if entity_archetype.damage_box.is_some() { 
            tags.push(EntityTags::Damageable(entity_archetype.damage_box.clone().unwrap()));
        }
        tags.push(EntityTags::BaseHealth(entity_archetype.health));
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
        return tags;
    }
    pub fn parse_and_convert_game_data(&mut self, paths: PathBundle) -> ParsedData{
        self.parse_entity_archetypes(paths.entity_archetypes_path);
        self.parse_entity_attack_patterns(paths.entity_attack_patterns_path);
        self.parse_entity_attacks(paths.entity_attacks_path);
        self.parse_terrain_archetypes(paths.terrain_archetypes_path);
        self.parse_sprites(paths.sprites_path);
        self.parse_starting_level(paths.starting_level_path);
        self.parse_item_archetypes(paths.item_archetypes_path);
        self.convert()
    }

    pub fn write(&self, paths: PathBundle) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(paths.starting_level_path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "{}", serde_json::to_string(&self.starting_level_json)?)?;
        Ok(())
    }
    pub fn get_entity_archetype_json(&self, name: &str) -> Option<&entity_archetype_json> {
        self.entity_archetypes_json.get(name)
    }
    pub fn get_terrain_archetype_json(&self, name: &str) -> Option<&terrain_archetype_json> {
        self.terrain_archetypes_json.get(name)
    }
    
}


#[derive(Debug, Clone)]
pub struct ParsedData{
    pub entity_archetypes: HashMap<String, Vec<EntityTags>>,
    pub entity_attack_patterns: HashMap<String, EntityAttackPattern>,
    pub entity_attacks: HashMap<String, EntityAttackDescriptor>,
    pub terrain_archetypes: HashMap<String, terrain_archetype_json>,
    pub sprites_to_load_json: Vec<String>,
    pub sprites: SpriteContainer,
    pub starting_level_descriptor: starting_level_json,
    pub item_archetypes: HashMap<String, ItemArchetype>,
}

impl ParsedData{
    pub fn new() -> Self{
        Self{
            entity_archetypes: HashMap::new(),
            entity_attack_patterns: HashMap::new(),
            entity_attacks: HashMap::new(),
            terrain_archetypes: HashMap::new(),
            sprites_to_load_json: Vec::new(),
            sprites: SpriteContainer::new(),
            item_archetypes: HashMap::new(),
            starting_level_descriptor: starting_level_json {
                player: player_json {
                    x: 0.0,
                    y: 0.0,
                    sprite: String::from(""),
                    health: 0.0,
                    max_health: 0,
                    movement_speed: 0.0
                },
                entities: Vec::new(),
                terrain: Vec::new()
            }
        }
    }
    pub fn get_entity_archetype(&self, name: &str) -> Option<&Vec<EntityTags>> {
        self.entity_archetypes.get(name)
    }
    pub fn get_terrain_archetype(&self, name: &str) -> Option<&terrain_archetype_json> {
        self.terrain_archetypes.get(name)
    }
}