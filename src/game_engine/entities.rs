use crate::error::PError;
use crate::{ptry, punwrap};
use std::cell::{RefCell, RefMut};
use super::entity_attacks::EntityAttackBox;
use super::entity_components::{self, CollisionBox, EntityAttackComponent, PathfindingComponent, PositionComponent};
use super::world::{Chunk, World};
use super::player::Player;
use super::pathfinding::{self, EntityDirectionOptions};

impl World {
    pub fn move_entity(&self, mut position_component: RefMut<PositionComponent>, entity_id: &usize, movement: [f32; 2], chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, respects_collision: bool, has_collision: bool) -> Result<(), PError>{ 
        if respects_collision && ptry!(self.check_collision(false, Some(*entity_id), (position_component.x + movement[0]).floor(), (position_component.y + movement[1]).floor(), 32,32, true)){
            return Ok(());
        }
        let prev_chunk = punwrap!(self.get_chunk_from_xy(position_component.x as usize, position_component.y as usize), "entity with id {} doesn't have a current chunk?", entity_id);
        
        if has_collision {
            let mut collision_cache_ref = self.collision_cache.borrow_mut();
            let prev_collision_tiles = World::get_terrain_tiles(position_component.x as usize, position_component.y as usize, 32, 32);
            let new_collision_tiles = World::get_terrain_tiles((position_component.x + movement[0]) as usize, (position_component.y + movement[1]) as usize, 32, 32);
            
            for tile in prev_collision_tiles.iter(){
                if new_collision_tiles.contains(tile){
                    continue;
                }else{
                    let tile_potentially = collision_cache_ref.get_mut(tile);
                    if let Some(tile) = tile_potentially{
                        tile.retain(|&x| x != *entity_id);
                    }
                }
            }
        }
        position_component.x += movement[0];
        position_component.y += movement[1];

        let new_chunk_potentially = self.get_chunk_from_xy(position_component.x as usize, position_component.y as usize);
        let new_chunk = if let Some(new_chunk) = new_chunk_potentially{
            new_chunk
        }else{
            self.new_chunk(World::coord_to_chunk_coord(position_component.x as usize), World::coord_to_chunk_coord(position_component.y as usize), Some(chunkref))
        };

        if new_chunk != prev_chunk {
            chunkref[prev_chunk].entities_ids.retain(|x| *x != *entity_id);
            chunkref[new_chunk].entities_ids.push(*entity_id);
        } 
        Ok(())
    }
    pub fn update_entities(&mut self) -> Result<(), PError> {
        // self.entity_attacks.borrow_mut().clear();
        self.pathfinding_frame += 1;
        self.pathfinding_frame %= 5;
        let player: Player = self.player.borrow().clone();
        for chunk in self.loaded_chunks.iter() {
            let chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>> = &mut self.chunks.borrow_mut();
            for entity_id in chunkref[*chunk].clone().entities_ids.iter() {
                ptry!(self.update_entity(entity_id, player.x.floor(), player.y.floor(), chunkref), "While updating entity {}", entity_id);
            }
        }
        Ok(())
    }
    pub fn update_entity(&self, entity_id: &usize, player_x: f32, player_y: f32, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>) -> Result<(), PError>{
        let entity_tags = punwrap!(self.get_entity_tags(*entity_id), NotFound, "Entity with id {} doesn't have any tags", entity_id);
        let mut distance: f64 = f64::MAX;
        let mut follows_player: bool = false;
        let mut aggro_range = 0;
        let mut attack_range = 0;
        let mut movement_speed = 1.0;
        let mut aggressive = false;
        let mut attacks = None; 
        let mut can_attack_player = false;
        let mut respects_collision = false;
        let mut collision = None;

        for tag in entity_tags.iter() {
            match tag {
                EntityTags::FollowsPlayer => {
                    follows_player = true;
                },
                EntityTags::AggroRange(range) => {
                    aggro_range = *range;
                },
                EntityTags::Range(range) => {
                    attack_range = *range;
                },
                EntityTags::MovementSpeed(speed) => {
                    movement_speed = *speed;
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
                EntityTags::HasCollision(collision_box) => {
                    collision = Some(collision_box);

                },
                _ => ()
            }
        }
        
        let health_component = self.entity_health_components.get(entity_id);
        if let Some(health_component) = health_component {
            let health_component = health_component.borrow();
            if health_component.health <= 0.0 {
                self.kill_entity(*entity_id);
                return Ok(());
            }
        }
        
        if follows_player {
            let position_component = punwrap!(self.entity_position_components.get(entity_id), Expected, "all entities with follow player tag should have entity position component").borrow();
            let px = player_x + self.player.borrow().collision_box.x_offset;
            let py = player_y + self.player.borrow().collision_box.y_offset;
            distance = f64::sqrt(
                (position_component.y as f64 - (py) as f64).powf(2.0) + (position_component.x as f64 - (px) as f64).powf(2.0),
            );
          
        }
        
        if aggressive {
            let mut aggro = punwrap!(self.entity_aggro_components.get(entity_id), Expected, "all entities with the aggressive tag should have an aggro component").borrow_mut();
            let position_component = punwrap!(self.entity_position_components.get(entity_id), "all entities with the follow player tag should have a position component").borrow();
            let px = player_x + self.player.borrow().collision_box.x_offset;
            let py = player_y + self.player.borrow().collision_box.y_offset;
            let distance = f64::sqrt(
                (position_component.y as f64 - (py) as f64).powf(2.0) + (position_component.x as f64 - (px) as f64).powf(2.0),
                );
            if aggro.aggroed {
                if distance <= (attack_range as f64) {
                    can_attack_player = true;
                }
            } else if distance <= (aggro_range as f64) {
                aggro.aggroed = true;
                if distance <= (attack_range as f64) {
                    can_attack_player = true;
                } 
            }
        }
        if can_attack_player && aggressive {
            let attack_pattern: &EntityAttackPattern = punwrap!(attacks, Expected, "all entities with the aggressive tag should have an attack pattern");
            let mut attack_component = punwrap!(self.entity_attack_components.get(entity_id), Expected, "all aggressive entities should have an attack component").borrow_mut();

            if attack_component.cur_attack_cooldown <= 0.0 {
                // self.player.borrow_mut().health -= attack_pattern.attacks[attack_component.cur_attack].attack();
                
                let position = punwrap!(self.entity_position_components.get(entity_id), Expected, "all entities that can attack should have a position component").borrow();
                let px = player_x + self.player.borrow().collision_box.x_offset;
                let py = player_y + self.player.borrow().collision_box.y_offset;
                let direction_to_player_unnormalized = [
                    px - position.x,
                    py - position.y
                ];
                let magnitude = f32::sqrt(direction_to_player_unnormalized[0].powf(2.0) + direction_to_player_unnormalized[1].powf(2.0));
                let direction_to_player = [
                    direction_to_player_unnormalized[0] / magnitude as f32,
                    direction_to_player_unnormalized[1] / magnitude as f32
                ];
                let angle = f32::atan2(direction_to_player[1], direction_to_player[0]);
                let descriptor = punwrap!(self.get_attack_descriptor_by_name(&attack_pattern.attacks[attack_component.cur_attack]), Invalid, "attack descriptor refers to a non-existent attack {}", attack_pattern.attacks[attack_component.cur_attack]); 
                match descriptor.r#type {
                    AttackType::Magic => {
                        let max_dist = descriptor.reach as f32/2.0 + descriptor.max_start_dist_from_entity.unwrap_or(0) as f32;
                        let dist_to_player = f32::sqrt((px - position.x).powf(2.0) + (py - position.y).powf(2.0));
                        if dist_to_player < max_dist {
                            self.entity_attacks.borrow_mut().push(EntityAttackBox {
                                archetype: attack_pattern.attacks[attack_component.cur_attack].clone(),
                                x: px,
                                y: py,
                                time_charged: 0.0,
                                rotation: angle,
                            });
                        } else{
                            self.entity_attacks.borrow_mut().push(
                                EntityAttackBox {
                                    archetype: attack_pattern.attacks[attack_component.cur_attack].clone(),
                                    x: position.x + direction_to_player[0] * (max_dist),
                                    y: position.y + direction_to_player[1] * (max_dist),
                                    time_charged: 0.0,
                                    rotation: angle,
                                }
                            )
                        }
                    }
                    AttackType::Melee => {
                        self.entity_attacks.borrow_mut().push(
                            EntityAttackBox {
                                archetype: attack_pattern.attacks[attack_component.cur_attack].clone(),
                                x: position.x + angle.cos() * (descriptor.reach as f32/2.0),
                                y: position.y + angle.sin() * (descriptor.reach as f32/2.0),
                                time_charged: 0.0,
                                rotation: angle,
                            }
                        )
                    }
                    _ => todo!()
                }
                attack_component.cur_attack += 1;
                if attack_component.cur_attack >= attack_pattern.attacks.len(){
                    attack_component.cur_attack = 0;
                }

                attack_component.cur_attack_cooldown = attack_pattern.attack_cooldowns[attack_component.cur_attack];                
            }else{
                attack_component.cur_attack_cooldown -= 1.0/60.0;
            }
        }
        let mut aggroed_to_entity = false;
        if aggressive {
            let aggro_potentially = self.entity_aggro_components.get(entity_id);
            if let Some(aggro) = aggro_potentially {
                aggroed_to_entity = aggro.borrow().aggroed;
            }
        }
        if aggroed_to_entity{
            let pathfinding_component = punwrap!(self.entity_pathfinding_components.get(entity_id), Expected, "all entities that follow player should have a pathfinding component").borrow_mut();
            let position_component = punwrap!(self.entity_position_components.get(entity_id), Expected, "all entities that follow player should have a position component").borrow_mut();
            ptry!(self.move_entity_towards_player(entity_id, collision.unwrap_or(&CollisionBox {
                    w: 0.0,
                    h: 0.0,
                    x_offset: 0.0,
                    y_offset: 0.0
            }),position_component, pathfinding_component, chunkref,  player_x, player_y, respects_collision, collision.is_some(), movement_speed));
        }
        Ok(())
    }
    pub fn move_entity_towards_player(&self, entity_id: &usize,collision_box: &CollisionBox, position_component: RefMut<PositionComponent>, mut pathfinding_component: RefMut<PathfindingComponent>, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, player_x: f32, player_y: f32, respects_collision: bool, has_collision: bool, movement_speed: f32) -> Result<(), PError>{
        let direction: [f32; 2] = [player_x - position_component.x, player_y - position_component.y];
        let entity_pathfinding_frame = punwrap!(self.pathfinding_frames.get(entity_id), Expected, "all entities that follow player should have a pathfinding frame, entity with id {} doesn't, was the entity properly created?", entity_id);
        if direction[0] == 0.0 && direction[1] == 0.0 {
            return Ok(());
        }
        if self.pathfinding_frame != *entity_pathfinding_frame {
            let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
            if magnitude > 128.0{
                match pathfinding_component.cur_direction {
                    EntityDirectionOptions::Down => {
                        ptry!(self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Up => {
                        ptry!(self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Left => {
                        ptry!(self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Right => {
                        ptry!(self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::None => {
                    },
                }
                return Ok(());
            }
        }
        let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        if respects_collision {
            if magnitude > 128.0{
                let direction: EntityDirectionOptions= ptry!(pathfinding::pathfind_by_block(*position_component, *collision_box, *entity_id, self));
                match direction {
                    EntityDirectionOptions::Down => {
                        ptry!(self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision));
                        pathfinding_component.cur_direction = EntityDirectionOptions::Down;
                    },
                    EntityDirectionOptions::Up => {
                        ptry!(self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision));
                        pathfinding_component.cur_direction = EntityDirectionOptions::Up;
                    },
                    EntityDirectionOptions::Left => {
                        ptry!(self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision));
                        pathfinding_component.cur_direction = EntityDirectionOptions::Left;
                    },
                    EntityDirectionOptions::Right => {
                        ptry!(self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision));
                        pathfinding_component.cur_direction = EntityDirectionOptions::Right;
                    },
                    EntityDirectionOptions::None => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::None;
                    },
                };
            } else if magnitude > 60.0{
                let direction: EntityDirectionOptions = ptry!(pathfinding::pathfind_high_granularity(*position_component, *collision_box,*entity_id, self));
                match direction {
                    EntityDirectionOptions::Down => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Down;
                        ptry!(self.move_entity(position_component, entity_id, [0.0, movement_speed], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Up => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Up;
                        ptry!(self.move_entity(position_component, entity_id, [0.0, -movement_speed], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Left => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Left;
                        ptry!(self.move_entity(position_component, entity_id, [-movement_speed, 0.0], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::Right => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::Right;
                        ptry!(self.move_entity(position_component, entity_id, [movement_speed, 0.0], chunkref, respects_collision, has_collision));
                    },
                    EntityDirectionOptions::None => {
                        pathfinding_component.cur_direction = EntityDirectionOptions::None;
                    },
                }
            }else {
                let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
                ptry!(self.move_entity(position_component, entity_id,  movement, chunkref,  respects_collision, has_collision));
            }
        }else{
            let movement = [direction[0] / magnitude * movement_speed, direction[1] / magnitude * movement_speed];
            ptry!(self.move_entity(position_component, entity_id,  movement, chunkref,  respects_collision, has_collision));
        }
        Ok(())
    }
    pub fn add_entity(&mut self, x: f32, y: f32) -> usize{
        let new_entity_pathfinding_frame = self.next_pathfinding_frame_for_entity;
        self.next_pathfinding_frame_for_entity += 1;
        self.next_pathfinding_frame_for_entity %= 5;
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_xy(x.floor() as usize, y.floor() as usize);
        let chunk_id = if let Some(cid) = chunk_id_potentially {
            cid
        } else{
            self.new_chunk(World::coord_to_chunk_coord(x.floor() as usize), World::coord_to_chunk_coord(y.floor() as usize), None)
        };
        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].entities_ids.push(self.element_id - 1);
        self.entity_position_components.insert(self.element_id - 1, RefCell::new(PositionComponent{x, y}));
        self.pathfinding_frames.insert(self.element_id - 1, new_entity_pathfinding_frame);
        self.element_id - 1
    }
    pub fn add_pathfinding_component(&mut self, entity_id: usize, new_entity_pathfinding: PathfindingComponent){
        self.entity_pathfinding_components.insert(entity_id, RefCell::new(new_entity_pathfinding));
    }
    pub fn add_attack_component(&mut self, entity_id: usize, new_entity_attack: EntityAttackComponent){
        self.entity_attack_components.insert(entity_id, RefCell::new(new_entity_attack));
    }
    pub fn add_health_component(&mut self, entity_id: usize, new_entity_health: entity_components::HealthComponent){
        self.entity_health_components.insert(entity_id, RefCell::new(new_entity_health));
    }   
    pub fn create_entity_with_archetype(&mut self, x: f32, y: f32, archetype: String) -> usize{
        let entity = self.add_entity(x, y);
        self.set_entity_archetype(entity, archetype.clone());
        let archetype = self.entity_archetype_tags_lookup.get(&archetype).expect("Archetype not found");
        let mut needs_attack_component = false;
        let mut needs_pathfinding_component = false;
        let mut needs_health_component = false;
        let mut needs_aggro_component = false;
        let mut health = 0;
        for tag in archetype.iter(){
            match tag.clone(){
                EntityTags::Attacks(_) => {
                    needs_attack_component = true;
                },
                EntityTags::FollowsPlayer => {
                    needs_pathfinding_component = true;
                },
                EntityTags::BaseHealth(h) => {
                    needs_health_component = true;
                    health = h;
                },
                EntityTags::Aggressive => {
                    needs_aggro_component = true;
                }
                _ => {}
            }
        }
        if needs_attack_component{
            self.add_attack_component(entity, entity_components::EntityAttackComponent::default());
        }
        if needs_pathfinding_component{
            self.add_pathfinding_component(entity, entity_components::PathfindingComponent::default());
        }
        if needs_health_component{
            self.add_health_component(entity, entity_components::HealthComponent{health: health as f32, max_health: health});
        }
        if needs_aggro_component{
            self.add_aggro_component(entity, entity_components::AggroComponent::default());
        }
        entity
    }
    pub fn add_entity_archetype(&mut self, name: String, archetype: Vec<EntityTags>){
        self.entity_archetype_tags_lookup.insert(name, archetype);
    }
    pub fn add_aggro_component(&mut self, entity_id: usize, new_entity_aggro: entity_components::AggroComponent){
        self.entity_aggro_components.insert(entity_id, RefCell::new(new_entity_aggro));
    }
    pub fn get_entity_archetype(&self, element_id: &usize) -> Option<&String>{
        self.entity_archetype_lookup.get(element_id)
    }
    pub fn get_entity_tags(&self, element_id: usize) -> Option<&Vec<EntityTags>>{
        let entity_archetype_id = self.get_entity_archetype(&element_id)?;
        self.entity_archetype_tags_lookup.get(entity_archetype_id)
    }
    pub fn set_entity_archetype(&mut self, element_id: usize, archetype_id: String){
        self.entity_archetype_lookup.insert(element_id, archetype_id);
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AttackType {
    Melee,
    Ranged,
    Magic
}


#[derive(Copy, Clone, Debug, PartialEq)]
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
    HasCollision(CollisionBox),
    AggroRange(usize),
    AttackType(AttackType),
    Attacks(EntityAttackPattern),
    MovementSpeed(f32),
    Drops(Vec<usize>), // loot table ids
    BaseHealth(usize),
    Damageable(CollisionBox)
}

#[derive(Clone, Debug)]
pub struct EntityAttackPattern {
    pub attacks: Vec<String>,
    pub attack_cooldowns: Vec<f32>,  
}
impl EntityAttackPattern{
    pub fn new(attacks: Vec<String>, attack_cooldowns: Vec<f32>) -> Self{
        Self{
            attacks,
            attack_cooldowns,
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