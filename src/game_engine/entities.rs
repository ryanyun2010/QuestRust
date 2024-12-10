use crate::loot::Loot;
use crate::game_engine::item::Item;
use std::{cell::RefCell, collections::HashMap};
use super::world::{Chunk, Player, World};
#[derive(Copy, Clone, Debug)]
pub struct Entity{
    pub element_id: usize,
    pub x: f32,
    pub y: f32,
    pub aggroed_to_player: bool,
    // I should change this to components one day 🤷‍♂️
    pub cur_attack: usize,
    pub cur_attack_cooldown: f32,
    // Oh wait I need to make EntityComponents soon 💀
    // Back to Items for now.
    // pub components: Vec<EntityComponents>
}

impl Entity{
    pub fn new(element_id: usize, x: f32, y:f32) -> Self{
        Self{
            element_id: element_id,
            x: x,
            y: y,
            aggroed_to_player: false,
            cur_attack: 0,
            cur_attack_cooldown: 0.15,
        }
    }
}

impl World {
    pub fn move_entity(&self, entity: &mut Entity, entity_id: &usize, movement: [f32; 2], chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>){
        let prev_chunk = self.get_chunk_from_xy(entity.x as usize, entity.y as usize).unwrap();
        entity.x += movement[0];
        entity.y += movement[1];
        let new_chunk_potentially = self.get_chunk_from_xy(entity.x as usize, entity.y as usize);
        let new_chunk: usize;
        if new_chunk_potentially.is_none(){
            new_chunk = self.new_chunk(World::coord_to_chunk_coord(entity.x as usize), World::coord_to_chunk_coord(entity.y as usize));
        }else{
            new_chunk = new_chunk_potentially.unwrap();
        }

        if new_chunk != prev_chunk {
            chunkref[prev_chunk].entities_ids.retain(|x| *x != *entity_id);
            chunkref[new_chunk].entities_ids.push(*entity_id);
            self.entity_lookup.borrow_mut().insert(new_chunk, *entity_id);
        } 
        // entity.move_(movement);
    }
    pub fn update_entities(&mut self) {
        let player: Player = self.player.borrow().clone();
        for chunk in self.loaded_chunks.iter() {
            let mut chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>> = &mut self.chunks.borrow_mut();
            for entity_id in chunkref[*chunk].clone().entities_ids.iter() {
                self.update_entity(entity_id, player.x, player.y, chunkref);
            }
        }
    }
    pub fn update_entity(&self, entity_id: &usize, player_x: f32, player_y: f32, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>) {
        let entity_tags: &Vec<EntityTags> = self.get_entity_tags(*entity_id).unwrap();
        let mut entity_mut_hash: std::cell::RefMut<'_, HashMap<usize, Entity>> = self.entities.borrow_mut();
        let mut entity: &mut Entity = entity_mut_hash.get_mut(entity_id).unwrap();
        let mut distance: f64 = f64::MAX;
        let mut follows_player: bool = false;
        let mut aggroed_to_player: bool = false;
        let mut aggro_range: usize = 0;
        let mut attack_range: usize = 0;
        let mut movement_speed: f32 = 1.0;
        let mut aggressive: bool = false;
        let mut attacks: Option<EntityAttackPattern>= None; 
        let mut can_attack_player: bool = false;
        for tag_id in 0..entity_tags.len() {
            // println!("{:?}", entity_tags[tag_id]);
            match entity_tags[tag_id].clone() {
                EntityTags::FollowsPlayer => {
                    follows_player = true;
                },
                EntityTags::AggroRange(range) => {
                    aggro_range = range as usize;
                },
                EntityTags::Range(range) => {
                    attack_range = range as usize;
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
                // println!("Attacking player {:?}", self.player.borrow().health);
            }else{
                entity.cur_attack_cooldown -= 1.0/60.0;
            }

            
        }
        if aggroed_to_player {
            let direction: [f32; 2] = [player_x - entity.x, player_y - entity.y];
            if (direction[0].abs() + direction[1].abs()) > 0.0 {
                let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
                let movement: [f32; 2] = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
                self.move_entity(entity, entity_id,  movement, chunkref);

            }
        }
    }
    pub fn add_entity(&mut self, x: f32, y: f32) -> usize{
        let new_entity: Entity = Entity::new(self.element_id,x,y);
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_xy(World::coord_to_chunk_coord(new_entity.x.floor() as usize), World::coord_to_chunk_coord(new_entity.y.floor() as usize));
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_entity.x.floor() as usize), World::coord_to_chunk_coord(new_entity.y.floor() as usize));
        } else{
            chunk_id = chunk_id_potentially.unwrap();
        }
        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].entities_ids.push(self.element_id - 1);
        self.entities.borrow_mut().insert(self.element_id - 1, new_entity);
        self.entity_lookup.borrow_mut().insert(self.element_id - 1, chunk_id);
        // self.entity_tags_lookup.insert(self.element_id - 1, tags);
        self.element_id - 1
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
    Range,
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
    Structure
}
#[derive(Clone, Debug)]
pub enum EntityTags {
    Aggressive,
    MonsterType(MonsterType),
    FollowsPlayer,
    Range(usize),
    AggroRange(usize),
    AttackType(AttackType),
    Attacks(EntityAttackPattern),
    MovementSpeed(f32),
    Item(Item),
    Drops(Loot),
    BaseHealth(usize),
}

impl EntityTags{
    // pub fn new(aggressive: bool, monster_type: MonsterType, follows_player: bool, range: usize, aggro_range: usize, attack_type: AttackType, attacks: EntityAttackPattern, movement_speed: usize, is_item: bool, drops: Option<Loot>,  contained_item: Option<Item>, max_health: usize) -> Self{
    //     Self{
    //         aggressive: aggressive,
    //         monster_type: monster_type,
    //         follows_player: follows_player,
    //         range: range,
    //         aggro_range: aggro_range,
    //         attack_type: attack_type,
    //         attacks: attacks,
    //         movement_speed: movement_speed,
    //         is_item: is_item,
    //         drops: drops,
    //         contained_item: contained_item,
    //         max_health: max_health
    //     }
    // }

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
    damage: usize
}

impl EntityAttack{
    pub fn new(damage: usize) -> Self{
        Self{
            damage: damage
        }
    }

    pub fn attack(&self) -> i32{
        self.damage as i32
    }
}