use crate::loot::Loot;
use crate::game_engine::item::Item;
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use super::entity_components::{self, EntityAttackComponent, EntityComponentHolder, PathfindingComponent, PositionComponent};
use super::world::{Chunk, EntityDirectionOptions, World};
use super::player::Player;
use super::pathfinding;
use super::json_parsing::ParsedData;


macro_rules! get_component {
    ($self:expr, $entity_id:expr, $component:ident) => {
        $self.entity_components_lookup
            .get($entity_id)
            .expect("Entity Not Found")
            .$component
            .borrow()
    };
}

impl World {
    pub fn move_entity(&self, position_component: &mut PositionComponent, entity_id: &usize, movement: [f32; 2], chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, respects_collision: bool, has_collision: bool){ 
        if respects_collision && self.check_collision(false, Some(*entity_id), (position_component.x + movement[0]).floor() as usize, (position_component.y + movement[1]).floor() as usize, 32,32, true){
            return;
        }
        let prev_chunk = self.get_chunk_from_xy(position_component.x as usize, position_component.y as usize).unwrap();
        
        if has_collision {
            let mut collision_cache_ref = self.collision_cache.borrow_mut();
            let prev_collision_tiles = World::get_terrain_tiles(position_component.x as usize, position_component.y as usize, 32, 32);
            let new_collision_tiles = World::get_terrain_tiles((position_component.x + movement[0]) as usize, (position_component.y + movement[1]) as usize, 32, 32);
            
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
        position_component.x += movement[0];
        position_component.y += movement[1];

        let new_chunk_potentially = self.get_chunk_from_xy(position_component.x as usize, position_component.y as usize);
        let new_chunk: usize;
        if new_chunk_potentially.is_none(){
            new_chunk = self.new_chunk(World::coord_to_chunk_coord(position_component.x as usize), World::coord_to_chunk_coord(position_component.y as usize), Some(chunkref));
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
        let entity_components = self.entity_components_lookup.get(entity_id).unwrap();

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
            let position_component = get_component!(self, entity_id, position).expect("Entities with tag: FollowsPlayer must have a PositionComponent");
            distance = f64::sqrt(
                (position_component.y as f64 - (player_y) as f64).powf(2.0) + (position_component.x as f64 - (player_x) as f64).powf(2.0),
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
            let attack_pattern: EntityAttackPattern = attacks.expect("Aggressive entities must have an attack pattern");

            let mut attack_component_ref = entity_components.attack.borrow_mut();
            let attack_component = attack_component_ref.as_mut().expect("Aggressive entities must have an attack component");

            if attack_component.cur_attack_cooldown <= 0.0 {
                self.player.borrow_mut().health -= attack_pattern.attacks[attack_component.cur_attack].attack();
                attack_component.cur_attack += 1;
                if attack_component.cur_attack >= attack_pattern.attacks.len(){
                    attack_component.cur_attack = 0;
                }
                attack_component.cur_attack_cooldown = attack_pattern.attack_cooldowns[attack_component.cur_attack];                
            }else{
                attack_component.cur_attack_cooldown -= 1.0/60.0;
            }
        }
        if aggroed_to_player {
            let mut pathfinding_component_ref = entity_components.pathfinding.borrow_mut();
            let pathfinding_component = pathfinding_component_ref.as_mut().expect("Entities with tag: FollowsPlayer must have a PathfindingComponent");
            let mut position_component_ref = entity_components.position.borrow_mut();
            let position_component = position_component_ref.as_mut().expect("Entities with tag: FollowsPlayer must have a PositionComponent");
            self.move_entity_towards_player(entity_id, position_component, pathfinding_component, chunkref,  player_x, player_y, respects_collision, has_collision, movement_speed);
        }
    }
    pub fn move_entity_towards_player(&self, entity_id: &usize, position_component: &mut PositionComponent, pathfinding_component: &mut PathfindingComponent, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, player_x: f32, player_y: f32, respects_collision: bool, has_collision: bool, movement_speed: f32){
        let direction: [f32; 2] = [player_x - position_component.x, player_y - position_component.y];
        let entity_pathfinding_frame = self.pathfinding_frames.get(entity_id).unwrap();
        if direction[0] == 0.0 && direction[1] == 0.0 {
            return;
        }
        if self.pathfinding_frame != *entity_pathfinding_frame {
            let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
            if magnitude > 128.0{
                match pathfinding_component.cur_direction {
                    EntityDirectionOptions::Down => {
                        self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision);
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
            let collision_component = get_component!(self, entity_id, collision_box).expect("Entities with tag: FollowsPlayer must have a CollisionBox");
            if magnitude > 128.0{
                let direction: EntityDirectionOptions = pathfinding::pathfind_by_block(position_component.clone(), collision_component, *entity_id, self);
                match direction {
                    EntityDirectionOptions::Down => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Down;
                        self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Up;
                        self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Left;
                        self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Right;
                        self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::None => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::None;
                    },
                }
            }else if magnitude > 60.0{
                let direction: EntityDirectionOptions = pathfinding::pathfind_high_granularity(position_component.clone(), collision_component,*entity_id, self);
                match direction {
                    EntityDirectionOptions::Down => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Down;
                        self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Up => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Up;
                        self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Left => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Left;
                        self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::Right => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Right;
                        self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision);
                    },
                    EntityDirectionOptions::None => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::None;
                    },
                }
            }else {
                let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
                self.move_entity(position_component, entity_id,  movement, chunkref,  respects_collision, has_collision);
            }
        }else{
            let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
            self.move_entity(position_component, entity_id,  movement, chunkref,  respects_collision, has_collision);
        }
    }
    pub fn add_entity(&mut self, x: f32, y: f32) -> usize{
        let new_entity_position = entity_components::PositionComponent {
            x: x,
            y: y,
        };
        let new_entity_pathfinding_frame = self.next_pathfinding_frame_for_entity;
        self.next_pathfinding_frame_for_entity += 1;
        self.next_pathfinding_frame_for_entity = self.next_pathfinding_frame_for_entity % 5;
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_xy(new_entity_position.x.floor() as usize, new_entity_position.y.floor() as usize);
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_entity_position.x.floor() as usize), World::coord_to_chunk_coord(new_entity_position.y.floor() as usize), None);
        } else{
            chunk_id = chunk_id_potentially.unwrap();
        }
        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].entities_ids.push(self.element_id - 1);
        let ECH = entity_components::EntityComponentHolder::new();
        *ECH.position.borrow_mut() = Some(new_entity_position);
        *ECH.collision_box.borrow_mut() = Some(entity_components::CollisionBox{
            x_offset: 0.0, y_offset: 0.0, w: 32.0, h: 32.0
        });
        *ECH.pathfinding.borrow_mut() = Some(entity_components::PathfindingComponent::default());
        *ECH.health.borrow_mut() = Some(entity_components::HealthComponent::new(100));
        *ECH.attack.borrow_mut() = Some(entity_components::EntityAttackComponent::default());
        self.entity_components_lookup.borrow_mut().insert(self.element_id - 1, ECH);
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