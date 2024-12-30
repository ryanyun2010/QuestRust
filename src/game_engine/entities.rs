use crate::loot::Loot;
use crate::game_engine::item::Item;
use std::collections::HashMap;
use super::world::{Chunk, EntityDirectionOptions, World};
use super::player::Player;
use super::pathfinding;
use super::json_parsing::ParsedData;

#[derive(Copy, Clone, Debug)]
pub struct Entity{
    pub element_id: usize,
    pub x: f32,
    pub y: f32,
    pub aggroed_to_player: bool,
    // I should change this to components one day 🤷‍♂️
    pub cur_attack: usize,
    pub cur_attack_cooldown: f32,
    pub cur_pathfinding_direction: EntityDirectionOptions,
    // Oh wait I need to make EntityComponents soon 💀
    // Back to Items for now.
    // pub components: Vec<EntityComponents>
}
// TODO: ENTITY CHUNKING HAS A CRAZY AMOUNT OF BUGS HERE

impl Entity{
    pub fn new(element_id: usize, x: f32, y:f32) -> Self{
        Self{
            element_id: element_id,
            x: x,
            y: y,
            aggroed_to_player: false,
            cur_pathfinding_direction: EntityDirectionOptions::None,
            cur_attack: 0,
            cur_attack_cooldown: 0.15,
        }
    }
}

impl World {
    pub fn move_entity(&self, entity: &mut Entity, entity_id: &usize, movement: [f32; 2], chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, entityref: HashMap<usize, Entity> , respects_collision: bool, has_collision: bool){ 
        if respects_collision && self.check_collision(false, Some(*entity_id), (entity.x + movement[0]).floor() as usize, (entity.y + movement[1]).floor() as usize, 32,32, true,Some(entityref)){
            return;
        }
        let prev_chunk = self.get_chunk_from_xy(entity.x as usize, entity.y as usize).unwrap();
        
        if has_collision {
            let mut collision_cache_ref = self.collision_cache.borrow_mut();
            let prev_collision_tiles = World::get_terrain_tiles(entity.x as usize, entity.y as usize, 32, 32);
            let new_collision_tiles = World::get_terrain_tiles((entity.x + movement[0]) as usize, (entity.y + movement[1]) as usize, 32, 32);
            
            for tile in prev_collision_tiles.iter(){
                if new_collision_tiles.contains(tile){
                    continue;
                }else{
                    let tile_potentially = collision_cache_ref.get_mut(tile);
                    if tile_potentially.is_none(){
                        continue;
                    }else{
                        tile_potentially.unwrap().retain(|&x| x != *entity_id);
                    }
                }
            }
        }
        entity.x += movement[0];
        entity.y += movement[1];

        let new_chunk_potentially = self.get_chunk_from_xy(entity.x as usize, entity.y as usize);
        let new_chunk: usize;
        if new_chunk_potentially.is_none(){
            new_chunk = self.new_chunk(World::coord_to_chunk_coord(entity.x as usize), World::coord_to_chunk_coord(entity.y as usize), Some(chunkref));
        }else{
            new_chunk = new_chunk_potentially.unwrap();
        }

        if new_chunk != prev_chunk {
            println!("Moving entity from chunk: {} to chunk: {}", prev_chunk, new_chunk);
            println!("prev chunk: {:?}, new chunk: {:?}", chunkref[prev_chunk].entities_ids, chunkref[new_chunk].entities_ids);
            chunkref[prev_chunk].entities_ids.retain(|x| *x != *entity_id);
            chunkref[new_chunk].entities_ids.push(*entity_id);
            self.entity_lookup.borrow_mut().insert(new_chunk, *entity_id);
        } 
    }
    pub fn update_entities(&mut self) {
        self.pathfinding_frame += 1;
        self.pathfinding_frame = self.pathfinding_frame % 5;
        let player: Player = self.player.borrow().clone();
        for chunk in self.loaded_chunks.iter() {
            let chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>> = &mut self.chunks.borrow_mut();
            for entity_id in chunkref[*chunk].clone().entities_ids.iter() {
                self.update_entity(entity_id, player.x.floor(), player.y.floor(), chunkref);
            }
        }
    }
    pub fn update_entity(&self, entity_id: &usize, player_x: f32, player_y: f32, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>) {
        let entity_tags_potentially: Option<&Vec<EntityTags>> = self.get_entity_tags(*entity_id);
        if entity_tags_potentially.is_none() {
            return;
        }
        let entity_tags: &Vec<EntityTags> = entity_tags_potentially.unwrap();
        let entity_hash = self.entities.borrow().clone();
        let mut entity_mut_hash: std::cell::RefMut<'_, HashMap<usize, Entity>> = self.entities.borrow_mut();
        let entity: &mut Entity = entity_mut_hash.get_mut(entity_id).unwrap();
        let mut distance: f64 = f64::MAX;
        let mut follows_player: bool = false;
        let mut aggroed_to_player: bool = false;
        let mut aggro_range: usize = 0;
        let mut attack_range: usize = 0;
        let mut movement_speed: f32 = 1.0;
        let mut aggressive: bool = false;
        let mut attacks: Option<EntityAttackPattern>= None; 
        let mut can_attack_player: bool = false;
        let mut respects_collision: bool = false;
        let mut has_collision: bool = false;
        for tag in entity_tags.iter() {
            // println!("{:?}", entity_tags[tag_id]);
            match tag.clone() {
                EntityTags::FollowsPlayer => {
                    follows_player = true;
                },
                EntityTags::AggroRange(range) => {
                    aggro_range = range;
                },
                EntityTags::Range(range) => {
                    attack_range = range;
                },
                EntityTags::MovementSpeed(speed) => {
                    movement_speed = speed;
                },
                EntityTags::Aggressive => {
                    aggressive = true;
                },
                EntityTags::Attacks(att) => {
                    attacks = Some(att);
                },
                EntityTags::RespectsCollision => {
                    respects_collision = true;
                },
                EntityTags::HasCollision => {
                    has_collision = true;
                },
                _ => ()
            }
        }
        
        if follows_player {
            distance = f64::sqrt(
                (entity.y as f64 - (player_y) as f64).powf(2.0) + (entity.x as f64 - (player_x) as f64).powf(2.0),
            );
            if aggressive && distance <= (attack_range as f64) {
                can_attack_player = true;
            }else{
                if distance < (aggro_range as f64){
                    aggroed_to_player = true;
                }
            }
        }
        if can_attack_player && aggressive {
            let attack_pattern: EntityAttackPattern = attacks.unwrap();
            if entity.cur_attack_cooldown <= 0.0 {
                self.player.borrow_mut().health -= attack_pattern.attacks[entity.cur_attack].attack();
                entity.cur_attack += 1;
                if entity.cur_attack >= attack_pattern.attacks.len(){
                    entity.cur_attack = 0;
                }
                entity.cur_attack_cooldown = attack_pattern.attack_cooldowns[entity.cur_attack];                
            }else{
                entity.cur_attack_cooldown -= 1.0/60.0;
            }
        }
        if aggroed_to_player {
            self.move_entity_towards_player(entity_id, entity,chunkref, entity_hash.clone(),  player_x, player_y, respects_collision, has_collision, movement_speed);
        }
    }

    pub fn move_entity_towards_player(&self, entity_id: &usize, entity: &mut Entity, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, entity_hash: HashMap<usize, Entity>, player_x: f32, player_y: f32, respects_collision: bool, has_collision: bool, movement_speed: f32){
        let direction: [f32; 2] = [player_x - entity.x, player_y - entity.y];
        let entity_pathfinding_frame = self.pathfinding_frames.get(entity_id).unwrap();
        if direction[0] == 0.0 && direction[1] == 0.0 {
            return;
        }
        if self.pathfinding_frame != *entity_pathfinding_frame {
            let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
            if magnitude > 128.0{
                match entity.cur_pathfinding_direction {
                    EntityDirectionOptions::Down => {
                        self.move_entity(entity, entity_id, [0.0, movement_speed], chunkref, entity_hash.clone(), respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        self.move_entity(entity, entity_id, [0.0, -movement_speed], chunkref, entity_hash.clone(), respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        self.move_entity(entity, entity_id, [-movement_speed, 0.0], chunkref, entity_hash.clone(), respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        self.move_entity(entity, entity_id, [movement_speed, 0.0], chunkref, entity_hash.clone(), respects_collision, has_collision);
                    },
                    EntityDirectionOptions::None => {
                        ()
                    },
                }
                return;
            }
        }
        let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        if respects_collision {
            if magnitude > 128.0{
                let direction: EntityDirectionOptions = pathfinding::pathfind_by_block(*entity_id, self, entity, entity_hash.clone());
                match direction {
                    EntityDirectionOptions::Down => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Down;
                        self.move_entity(entity, entity_id, [0.0, movement_speed], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Up;
                        self.move_entity(entity, entity_id, [0.0, -movement_speed], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Left;
                        self.move_entity(entity, entity_id, [-movement_speed, 0.0], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Right;
                        self.move_entity(entity, entity_id, [movement_speed, 0.0], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::None => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::None;
                    },
                }
            }else if magnitude > 60.0{
                let direction: EntityDirectionOptions = pathfinding::pathfind_high_granularity(*entity_id, self, entity, entity_hash.clone());
                match direction {
                    EntityDirectionOptions::Down => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Down;
                        self.move_entity(entity, entity_id, [0.0, movement_speed], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Up;
                        self.move_entity(entity, entity_id, [0.0, -movement_speed], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Left;
                        self.move_entity(entity, entity_id, [-movement_speed, 0.0], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::Right;
                        self.move_entity(entity, entity_id, [movement_speed, 0.0], chunkref, entity_hash, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::None => {
                        entity.cur_pathfinding_direction = EntityDirectionOptions::None;
                    },
                }
            }else {
                let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
                self.move_entity(entity, entity_id,  movement, chunkref, entity_hash,  respects_collision, has_collision);
            }
        }else{
            let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
            self.move_entity(entity, entity_id,  movement, chunkref, entity_hash,  respects_collision, has_collision);
        }
    }
    pub fn add_entity(&mut self, x: f32, y: f32) -> usize{
        let new_entity: Entity = Entity::new(self.element_id,x,y);
        let new_entity_pathfinding_frame = self.next_pathfinding_frame_for_entity;
        self.next_pathfinding_frame_for_entity += 1;
        self.next_pathfinding_frame_for_entity = self.next_pathfinding_frame_for_entity % 5;
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_xy(new_entity.x.floor() as usize, new_entity.y.floor() as usize);
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_entity.x.floor() as usize), World::coord_to_chunk_coord(new_entity.y.floor() as usize), None);
        } else{
            chunk_id = chunk_id_potentially.unwrap();
        }
        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].entities_ids.push(self.element_id - 1);
        self.entities.borrow_mut().insert(self.element_id - 1, new_entity);
        self.pathfinding_frames.insert(self.element_id - 1, new_entity_pathfinding_frame);
        self.entity_lookup.borrow_mut().insert(self.element_id - 1, chunk_id);
        self.element_id - 1
    }
    
    pub fn create_entity_from_json_archetype(&mut self, x: f32, y: f32, archetype: &str, parser: &ParsedData) -> usize{
        let archetype = parser.get_entity_archetype(archetype).expect(&format!("Archetype {} not found", archetype));
        let entity = self.add_entity(x, y);
        self.add_entity_tags(entity, archetype.clone());
        entity
    }
    pub fn get_entity_tags(&self, element_id: usize) -> Option<&Vec<EntityTags>>{
        self.entity_tags_lookup.get(&element_id)
    }
    pub fn get_entity(&self, element_id: usize) -> Option<Entity>{
        let k: &usize = &element_id;
        let borrow: std::cell::Ref<'_, HashMap<usize, Entity>> = self.entities.borrow();
        borrow.get(k).cloned()
    }

}

#[derive(Copy, Clone, Debug)]
pub enum AttackType {
    Melee,
    Ranged,
    Magic
}


#[derive(Copy, Clone, Debug)]
pub enum MonsterType {
    Undead,
    Uruk,
    Parasite,
    Beast,
    Demon,
    Dragon,
    Item,
    Ambient,
}
#[derive(Clone, Debug)]
pub enum EntityTags {
    Aggressive,
    MonsterType(MonsterType),
    FollowsPlayer,
    Range(usize),
    RespectsCollision,
    HasCollision,
    AggroRange(usize),
    AttackType(AttackType),
    Attacks(EntityAttackPattern),
    MovementSpeed(f32),
    Item(Item),
    Drops(Loot),
    BaseHealth(usize),


}

#[derive(Clone, Debug)]
pub struct EntityAttackPattern {
    pub attacks: Vec<EntityAttack>,
    pub attack_cooldowns: Vec<f32>,  
}
impl EntityAttackPattern{
    pub fn new(attacks: Vec<EntityAttack>, attack_cooldowns: Vec<f32>) -> Self{
        Self{
            attacks: attacks,
            attack_cooldowns: attack_cooldowns,
        }
    }
    // pub fn in_range_to_attack(&mut self) -> Option<EntityAttack>{
    //     if self.cur_attack_cooldown <= 0.0{
    //         self.cur_attack += 1;
    //         if self.cur_attack >= self.attacks.len(){
    //             self.cur_attack = 0;
    //         }
    //         self.cur_attack_cooldown = self.attack_cooldowns[self.cur_attack];
    //         return Some(self.attacks[self.cur_attack]);
    //     }
    //     self.cur_attack_cooldown -= 1.0/60.0;
    //     None
    // }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityAttack{
    damage: f32
}

impl EntityAttack{
    pub fn new(damage: f32) -> Self{
        Self{
            damage: damage
        }
    }

    pub fn attack(&self) -> f32{
        self.damage as f32
    }
}