use compact_str::CompactString;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};
use std::fs::File;
use crate::error::PError;
use crate::game_engine::entities::EntityAttackPattern;
use crate::perror;
use crate::rendering_engine::abstractions::SpriteContainer;

use super::entities::AttackType;
use super::entity_attacks::EntityAttackDescriptor;
use super::entity_components::CollisionBox;
use super::item::{ItemArchetype, ItemType};
use super::loot::{LootTable, LootTableEntry};
use super::stat::GearStatList;


pub struct PathBundle{
    pub entity_archetypes_path: &'static str,
    pub entity_attack_patterns_path: &'static str,
    pub entity_attacks_path: &'static str,
    pub terrain_archetypes_path: &'static str,
    pub sprites_path: &'static str,
    pub starting_level_path: &'static str,
    pub item_archetypes_path: &'static str,
    pub loot_table_path: &'static str,
    pub rooms_path: &'static str,
    pub spawn_archetypes_path: &'static str
}

pub const PATH_BUNDLE: PathBundle = PathBundle{
    entity_archetypes_path: "src/game_data/entity_archetypes.json",
    entity_attack_patterns_path: "src/game_data/entity_attack_patterns.json",
    entity_attacks_path: "src/game_data/entity_attacks.json",
    terrain_archetypes_path: "src/game_data/terrain_archetypes.json",
    sprites_path: "src/game_data/sprites.json",
    starting_level_path: "src/game_data/starting_level.json",
    item_archetypes_path: "src/game_data/items.json",
    loot_table_path: "src/game_data/loot_tables.json",
    rooms_path: "src/game_data/rooms.json",
    spawn_archetypes_path: "src/game_data/spawn_archetypes.json"
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_attack_descriptor_json {
    pub name: CompactString,
    pub r#type: CompactString,
    pub damage: f32,
    pub reach: usize,
    pub max_start_dist_from_entity: Option<usize>,
    pub width: usize,
    pub time_to_charge: usize,
    pub sprite: CompactString,
    pub poison: Option<PoisonDescriptor>,
    pub fire: Option<FireDescriptor>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PoisonDescriptor {
    pub damage: f32,
    pub lifetime: f32,
    pub time_between_ticks: f32
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FireDescriptor {
    pub damage: f32,
    pub lifetime: f32,
    pub time_between_ticks: f32
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_archetype_json {
    pub name: CompactString,
    pub basic_tags: Vec<CompactString>,
    pub collision_box: Option<CollisionBox>,
    pub damage_box: Option<CollisionBox>,
    pub health: Option<usize>,
    pub monster_type: CompactString,
    pub movement_speed: Option<f32>,
    pub range: Option<usize>,
    pub aggro_range: Option<usize>,
    pub attack_type: CompactString,
    pub attack_pattern: Option<CompactString>,
    pub loot_table: Vec<CompactString>,
    pub sprite: Option<CompactString>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_attack_pattern_json {
    name: CompactString,
    attacks: Vec<CompactString>,
    cooldowns: Vec<f32>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprites_json_descriptor {
    pub basic_sprites: Vec<sprite_json>,
    pub spritesheets: Vec<sprite_sheet_json>
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprite_sheet_json{
    pub name: CompactString,
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
    pub name: CompactString,
    pub x: usize,
    pub y: usize
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct sprite_json {
    pub name: CompactString,
    pub path: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct player_json{
    pub x: f32,
    pub y: f32,
    pub sprite: CompactString,
    pub health: f32,
    pub max_health: i32,
    pub movement_speed: f32
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct entity_json{
    pub x: f32,
    pub y: f32,
    pub archetype: CompactString
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct terrain_json{
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub terrain_archetype: CompactString
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct terrain_archetype_json{
    pub name: CompactString,
    pub r#type: CompactString,
    pub random_chances: Option<Vec<f32>>,
    pub sprites: Vec<CompactString>,
    pub basic_tags: Vec<CompactString>
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
    pub name: CompactString,
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub AOE: f32,
    pub size: f32,
    pub sprite: CompactString
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct melee_attack_descriptor_json{
    pub name: CompactString,
    pub damage: f32,
    pub width: f32,
    pub reach: f32,
    pub lifetime: f32,
    pub sprite: CompactString
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct player_attacks_descriptor_json {
    pub ranged_projectiles: Vec<player_projectile_descriptor_json>,
    pub melee_attacks: Vec<melee_attack_descriptor_json>
}
impl Default for player_attacks_descriptor_json {
    fn default() -> Self {
        Self {
            ranged_projectiles: Vec::new(),
            melee_attacks: Vec::new()
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct item_archetype_json {
    pub name: CompactString,
    pub sprite: CompactString,
    pub attack_sprite: Option<CompactString>,
    pub width_to_length_ratio: Option<f32>,
    pub item_type: ItemType,
    pub lore: String,
    pub stats: GearStatList
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct item_loot_table_json {
    pub name: CompactString,
    pub loot: Vec<loot_table_entry_json> 
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct loot_table_entry_json {
    pub item: Option<CompactString>,
    pub weight: usize
}

#[derive(Clone)]
pub struct JSON_parser {
    pub entity_archetypes_json: FxHashMap<CompactString, entity_archetype_json>,
    pub entity_attack_patterns_json: FxHashMap<CompactString, entity_attack_pattern_json>,
    pub entity_attacks_json: FxHashMap<CompactString, entity_attack_descriptor_json>,
    pub terrain_archetypes_json: FxHashMap<CompactString, terrain_archetype_json>,
    pub sprites_json: sprites_json_descriptor,
    pub starting_level_json: starting_level_json,
    pub item_archetype_json: Vec<item_archetype_json>,
    pub loot_table_json: Vec<item_loot_table_json>,
    pub rooms_json: FxHashMap<CompactString, room_descriptor_json>,
    pub spawn_archetypes_json: FxHashMap<CompactString, spawn_archetype_json>
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct room_descriptor_json {
    pub name: CompactString,
    pub terrain: Vec<terrain_json>,
    pub width: usize,
    pub height: usize,
    pub spawnable: Vec<[usize; 2]>,
    pub spawn_archetype: CompactString,
    pub entrance: [usize; 2],
    pub exit: [usize; 2]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct spawn_archetype_json {
    pub name: CompactString,
    pub basic: Vec<entity_spawn_json>,
    pub total_points_to_spawn: usize,
    pub special: Vec<special_spawn_json>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct entity_spawn_json {
    pub archetype: CompactString,
    pub points: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct special_spawn_json {
    pub x: usize,
    pub y: usize,
    pub archetype: CompactString,
}


impl Default for JSON_parser {
    fn default() -> Self {
        Self::new()
    }
}

impl JSON_parser {
    pub fn new() -> Self {
        Self {
            entity_archetypes_json: FxHashMap::default(),
            entity_attack_patterns_json: FxHashMap::default(),
            terrain_archetypes_json: FxHashMap::default(),
            entity_attacks_json: FxHashMap::default(),

            sprites_json: sprites_json_descriptor {
                basic_sprites: Vec::new(),
                spritesheets: Vec::new()
            },
            item_archetype_json: Vec::new(),
            starting_level_json: starting_level_json {
                player: player_json {
                    x: 0.0,
                    y: 0.0,
                    sprite: CompactString::from(""),
                    health: 0.0,
                    max_health: 0,
                    movement_speed: 0.0
                },
                entities: Vec::new(),
                terrain: Vec::new()
            },
            loot_table_json: Vec::new(),
            rooms_json: FxHashMap::default(),
            spawn_archetypes_json: FxHashMap::default()
        }
    }

    pub fn parse_entity_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open entity archetypes file.");
        let reader = BufReader::new(file);
        let data: Vec<entity_archetype_json> = serde_json::from_reader(reader).expect("Entity Archetypes JSON was not well-formatted");
        for archetype in data {
            self.entity_archetypes_json.insert(archetype.name.clone(), archetype);
        }
    }
    pub fn parse_terrain_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open terrain archetypes file."); 
        let reader = BufReader::new(file);
        let data: Vec<terrain_archetype_json> = serde_json::from_reader(reader).expect("Terrain Archetypes JSON was not well-formatted");
        for archetype in data {
            self.terrain_archetypes_json.insert(archetype.name.clone(), archetype);
        }
    }
    pub fn parse_entity_attack_patterns(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open entity attack patterns file."); 
        let reader = BufReader::new(file);
        let data: Vec<entity_attack_pattern_json> = serde_json::from_reader(reader).expect("Entity Attack Patterns JSON was not well-formatted");
        for pattern in data {
            self.entity_attack_patterns_json.insert(pattern.name.clone(), pattern);
        }
    }
    pub fn parse_entity_attacks(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open entity atttacks file."); 
        let reader = BufReader::new(file);
        let data: Vec<entity_attack_descriptor_json> = serde_json::from_reader(reader).expect("Entity Attacks JSON was not well-formatted");
        for attack in data {
            self.entity_attacks_json.insert(attack.name.clone(), attack);
        }
    }
    pub fn parse_sprites(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open sprites file."); 
        let reader = BufReader::new(file);
        let data: sprites_json_descriptor = serde_json::from_reader(reader).expect("Sprites JSON was not well-formatted");
        self.sprites_json = data;
    }
    pub fn parse_starting_level(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open starting level file."); 
        let reader = BufReader::new(file);
        let data: starting_level_json = serde_json::from_reader(reader).expect("Starting Level JSON was not well-formatted");
        self.starting_level_json = data;
    }

    pub fn parse_item_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open item archetypes file."); 
        let reader = BufReader::new(file);
        let data: Vec<item_archetype_json> = serde_json::from_reader(reader).expect("Item Archetypes JSON was not well-formatted");
        self.item_archetype_json = data;
    }

    pub fn parse_loot_tables(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open the loot tables file."); 
        let reader = BufReader::new(file);
        let data: Vec<item_loot_table_json> = serde_json::from_reader(reader).expect("Loot Tables JSON was not well-formatted");
        self.loot_table_json = data;
    }

    pub fn parse_rooms(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open the rooms file.");
        let reader = BufReader::new(file);
        let data: Vec<room_descriptor_json> = serde_json::from_reader(reader).expect("Rooms JSON was not well-formatted");
        for room in data {
            self.rooms_json.insert(room.name.clone(), room);
        }
    }
    
    pub fn parse_spawn_archetypes(&mut self, path: &str) {
        let file = File::open(path).expect("\nCould not open the spawn archetypes file.");
        let reader = BufReader::new(file);
        let data: Vec<spawn_archetype_json> = serde_json::from_reader(reader).expect("Spawn Archetypes JSON was not well-formatted");
        for archetype in data {
            self.spawn_archetypes_json.insert(archetype.name.clone(), archetype);
        }
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
                sprite: entity_attack.sprite.clone(),
                fire: entity_attack.fire.clone(),
                poison: entity_attack.poison.clone()
            });
        }
        for (name, entity_attack_pattern) in &self.entity_attack_patterns_json {
            data.entity_attack_patterns.insert(name.clone(), EntityAttackPattern::new(entity_attack_pattern.attacks.clone(), entity_attack_pattern.cooldowns.clone()));
        }


        let mut ltid_lookup: HashMap<CompactString, usize>  = HashMap::new();
        let mut tables= FxHashMap::default();
        for loot_table in self.loot_table_json.iter() {
            let mut entries = Vec::new();
            for entry in loot_table.loot.iter() {
                entries.push(
                    LootTableEntry{
                        item: entry.item.clone(), 
                        weight: entry.weight
                    });
            }
            ltid_lookup.insert(loot_table.name.clone(), tables.len());
            tables.insert(loot_table.name.clone(), LootTable::new(entries));
        }

        data.loot_table_lookup = tables;

        (data.sprites_to_load_json, data.sprites) = SpriteContainer::create_from_json(&self.sprites_json);
        data.starting_level_descriptor = self.starting_level_json.clone();

        for (.., entity_archetype) in &self.entity_archetypes_json {
            crate::ok_or_panic!(JSON_parser::validate_entity_archetype(entity_archetype));
            data.entity_archetypes.insert(entity_archetype.name.clone(), entity_archetype.clone());
        }
        for (.., terrain_archetype) in &self.terrain_archetypes_json {
            data.terrain_archetypes.insert(terrain_archetype.name.clone(), terrain_archetype.clone());
        }

        for item_archetype in &self.item_archetype_json {
            data.item_archetypes.insert(item_archetype.name.clone(), ItemArchetype{
                name: item_archetype.name.clone(),
                stats: item_archetype.stats.clone(),
                lore: item_archetype.lore.clone(),
                item_type: item_archetype.item_type.clone(),
                width_to_length_ratio: item_archetype.width_to_length_ratio,
                sprite: item_archetype.sprite.clone(),
                attack_sprite: item_archetype.attack_sprite.clone()
        });
        }
        data.rooms = self.rooms_json.clone();
        data.spawn_archetypes = self.spawn_archetypes_json.clone();

        data
    }

    pub fn validate_entity_archetype(archetype: &entity_archetype_json) -> Result<(), PError>{
        let name = &archetype.name;
        let mut has_collision = false;
        let mut respects_collision = false;

        for tag in &archetype.basic_tags {
            match tag.as_str() {
                "attacker" => {
                    if archetype.attack_pattern.is_none() {
                        return Err(perror!(JSONValidationError, "Entity archetype: {} has the attacker tag but no attack pattern", name));
                    }
                    if archetype.range.is_none() {
                        return Err(perror!(JSONValidationError, "Entity archetype: {} has the attacker tag but no range", name));
                    }
                },
                "damageable" => {
                    if archetype.health.is_none() {
                        return Err(perror!(JSONValidationError, "Entity archetype: {} has the damageable tag but no health", name));
                    }
                },
                "aggressive" => {
                    if archetype.aggro_range.is_none() {
                        return Err(perror!(JSONValidationError, "Entity archetype: {} has the aggressive tag but no aggro range", name));
                    }
                    if archetype.movement_speed.is_none() {
                        return Err(perror!(JSONValidationError, "Entity archetype: {} has the aggressive tag but no movement speed", name));
                    }
                },
                "respectsCollision" => {
                    respects_collision = true;
                },
                "hasCollision" => {
                    has_collision = true;
                },
                "animated" => {},
                _ => {
                    return Err(perror!(JSONValidationError, "Entity archetype: {} has an unrecognized tag: {}", name, tag));
                }
            }
        }

        if has_collision && !respects_collision {
            return Err(perror!(JSONValidationError, "Entity archetype {} has collision but doesn't respect collision, this causes very uncertain behavior and is not recommended", name));
        }


        if archetype.attack_pattern.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "attacker").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has an attack pattern but no attacker tag", name))?;
        }
        if archetype.health.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "damageable").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has health but no damageable tag", name))?;
        }
        if archetype.aggro_range.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "aggressive").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has an aggro range but no aggressive tag", name))?;
        }
        if archetype.collision_box.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "hasCollision").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has a collision box but no hasCollision tag", name))?;
        }
        if archetype.damage_box.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "damageable").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has a damage box but no damageable tag", name))?;
        }

        if archetype.movement_speed.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "aggressive").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has a movement speed but no aggressive tag", name))?;
        }
        if archetype.range.is_some() {
            archetype.basic_tags.iter().find(|tag| tag.as_str() == "attacker").ok_or_else(|| perror!(JSONValidationError, "Entity archetype: {} has a range but no attacker tag", name))?;
        }
        Ok(())
    }
    pub fn parse_and_convert_game_data(&mut self, paths: PathBundle) -> ParsedData{
        self.parse_entity_archetypes(paths.entity_archetypes_path);
        self.parse_entity_attack_patterns(paths.entity_attack_patterns_path);
        self.parse_entity_attacks(paths.entity_attacks_path);
        self.parse_terrain_archetypes(paths.terrain_archetypes_path);
        self.parse_sprites(paths.sprites_path);
        self.parse_starting_level(paths.starting_level_path);
        self.parse_item_archetypes(paths.item_archetypes_path);
        self.parse_loot_tables(paths.loot_table_path);
        self.parse_rooms(paths.rooms_path);
        self.parse_spawn_archetypes(paths.spawn_archetypes_path);
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
    pub entity_archetypes: FxHashMap<CompactString, entity_archetype_json>,
    pub entity_attack_patterns: FxHashMap<CompactString, EntityAttackPattern>,
    pub entity_attacks: FxHashMap<CompactString, EntityAttackDescriptor>,
    pub terrain_archetypes: FxHashMap<CompactString, terrain_archetype_json>,
    pub rooms: FxHashMap<CompactString, room_descriptor_json>,
    pub spawn_archetypes: FxHashMap<CompactString, spawn_archetype_json>,
    pub sprites_to_load_json: Vec<String>,
    pub sprites: SpriteContainer,
    pub starting_level_descriptor: starting_level_json,
    pub item_archetypes: FxHashMap<CompactString, ItemArchetype>,
    pub loot_table_lookup: FxHashMap<CompactString, LootTable>,
}

impl Default for ParsedData {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsedData{
    pub fn new() -> Self{
        Self{
            entity_archetypes: FxHashMap::default(),
            entity_attack_patterns: FxHashMap::default(),
            entity_attacks: FxHashMap::default(),
            terrain_archetypes: FxHashMap::default(),
            sprites_to_load_json: Vec::new(),
            sprites: SpriteContainer::new(),
            item_archetypes: FxHashMap::default(),
            starting_level_descriptor: starting_level_json {
                player: player_json {
                    x: 0.0,
                    y: 0.0,
                    sprite: CompactString::from(""),
                    health: 0.0,
                    max_health: 0,
                    movement_speed: 0.0
                },
                entities: Vec::new(),
                terrain: Vec::new()
            },
            loot_table_lookup: FxHashMap::default(),
            rooms: FxHashMap::default(),
            spawn_archetypes: FxHashMap::default()
        }
    }
    pub fn get_terrain_archetype(&self, name: &str) -> Option<&terrain_archetype_json> {
        self.terrain_archetypes.get(name)
    }
}
