use compact_str::CompactString;
use itertools::izip;

use crate::error::PError;
use crate::{ptry, punwrap};
use std::cell::RefCell;
use super::camera::Camera;
use super::entity_attacks::EntityAttackBox;
use super::entity_components::{self, CollisionBox, EntityAttackComponent, PathfindingComponent, PositionComponent};
use super::json_parsing::entity_archetype_json;
use super::world::{Chunk, World};
use super::player::Player;
use super::pathfinding::{self, EntityDirectionOptions};

impl World {
    pub fn move_entity(&self, position_component: &mut PositionComponent, entity_id: &usize, movement: [f32; 2], chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, respects_collision: bool, has_collision: bool) -> Result<(), PError>{ 
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
    pub fn update_entities(&mut self, camera: &mut Camera) -> Result<(), PError> {
        // self.entity_attacks.borrow_mut().clear();
        self.pathfinding_frame += 1;
        self.pathfinding_frame %= 5;
        let player: Player = self.player.borrow().clone();
        let mut entities_to_update = Vec::new();
        let chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>> = &mut self.chunks.borrow_mut();
        for chunk in self.loaded_chunks.iter() {
            entities_to_update.extend(chunkref[*chunk].entities_ids.clone());
        }
        
        entities_to_update.sort();
        if entities_to_update.is_empty() {return Ok(());}

        // death checks
        let mut entities_to_update_index = 0;
        for (i, damageable_component) in 
            self.components.damageable_components.iter().enumerate().filter_map(
                |(i, damageable_component) | 
                if entities_to_update_index == entities_to_update.len() {None} 
                else if i == entities_to_update[entities_to_update_index] && damageable_component.is_some() {entities_to_update_index += 1; Some((i, damageable_component.as_ref().unwrap().borrow()))} 
                else {None}
            ){
            if damageable_component.health <= 0.0 {
                self.kill_entity(i);
            }
        }


        // pathfind towards player updates
        entities_to_update_index = 0;
        for (i, mut pathfinding_component, mut position_component, aggro_component, collision_component) in izip!(
            self.components.pathfinding_components.iter(),
            self.components.position_components.iter(),
            self.components.aggro_components.iter(),
            self.components.collision_components.iter()
        ).enumerate().filter_map(
            |(i, (pathfinding_component, position_component, aggro_component, collision_component))|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && pathfinding_component.is_some() && position_component.is_some() && aggro_component.is_some() {entities_to_update_index += 1; Some((i, pathfinding_component.as_ref().unwrap().borrow_mut(), position_component.as_ref().unwrap().borrow_mut(), aggro_component.as_ref().unwrap().borrow(), collision_component.as_ref().map(|x| x.borrow())))}
            else {None}
        ){
            let player_ref = self.player.borrow();
            let player_x = player_ref.x + player_ref.collision_box.x_offset;
            let player_y = player_ref.y + player_ref.collision_box.y_offset;
            if aggro_component.aggroed {
                ptry!(self.move_entity_towards_player(&i, &collision_component.as_ref().map(|x| x.collision_box).unwrap_or(CollisionBox::default()), &mut position_component, &mut pathfinding_component, chunkref,  player_x, player_y, collision_component.as_ref().map(|x| x.respects_collision).unwrap_or(false), collision_component.is_some()));
            }
        }

        // aggro component updates
        entities_to_update_index = 0;
        for (i, mut aggro_component, position_component) in izip!(
            self.components.aggro_components.iter(),
            self.components.position_components.iter()
        ).enumerate().filter_map(
            |(i, (aggro_component, position_component))|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && aggro_component.is_some() && position_component.is_some() {entities_to_update_index += 1; Some((i, aggro_component.as_ref().unwrap().borrow_mut(), position_component.as_ref().unwrap().borrow()))}
            else {None}
        ){
            let player_ref = self.player.borrow();
            let player_x = player_ref.x + player_ref.collision_box.x_offset;
            let player_y = player_ref.y + player_ref.collision_box.y_offset;
            let distance = f64::sqrt(
                (position_component.y as f64 - (player_y) as f64).powf(2.0) + (position_component.x as f64 - (player_x) as f64).powf(2.0),
            );
            if aggro_component.aggroed {
            } else if distance <= aggro_component.aggro_range as f64 && (aggro_component.aggro_through_walls || ptry!(self.is_line_of_sight(position_component.x, position_component.y, player_x, player_y))) {
                aggro_component.aggroed = true;
            }
        }

        // attack component updates
        entities_to_update_index = 0;

        for (i, position_component, mut attack_component) in izip!(
            self.components.position_components.iter(),
            self.components.attack_components.iter()
        ).enumerate().filter_map(
            |(i, (position_component, attack_component))|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && position_component.is_some() && attack_component.is_some() {entities_to_update_index += 1; Some((i, position_component.as_ref().unwrap().borrow(), attack_component.as_ref().unwrap().borrow_mut()))}
            else {None}
        ){
             
            let player_ref = self.player.borrow();
            let player_x = player_ref.x + player_ref.collision_box.x_offset;
            let player_y = player_ref.y + player_ref.collision_box.y_offset;
            let distance = f64::sqrt(
                (position_component.y as f64 - (player_y) as f64).powf(2.0) + (position_component.x as f64 - (player_x) as f64).powf(2.0),
            );
            if distance < attack_component.attack_range as f64 {
                let attack_pattern = punwrap!(self.entity_attack_pattern_lookup.get(&attack_component.entity_attack_pattern), Expected, "entity attack component on entity with id {} refers to non-existent entity attack pattern {}", i, attack_component.entity_attack_pattern);
                if attack_component.cur_attack_cooldown <= 0.0 {
                    let direction_to_player_unnormalized = [
                        player_x - position_component.x,
                        player_y - position_component.y
                    ];
                    let magnitude = f32::sqrt(direction_to_player_unnormalized[0].powf(2.0) + direction_to_player_unnormalized[1].powf(2.0));
                    let direction_to_player = [
                        direction_to_player_unnormalized[0] / magnitude,
                        direction_to_player_unnormalized[1] / magnitude
                    ];
                    let angle = f32::atan2(direction_to_player[1], direction_to_player[0]);
                    let descriptor = punwrap!(self.get_attack_descriptor_by_name(&attack_pattern.attacks[attack_component.cur_attack]), Invalid, "attack pattern {} refers to a non-existent attack {}", &attack_component.entity_attack_pattern, attack_pattern.attacks[attack_component.cur_attack]);
                    if ptry!(self.is_line_of_sight(position_component.x, position_component.y, player_x, player_y), "while updating entity with id {}", i) {
                        match descriptor.r#type {
                            AttackType::Magic => {
                                let max_dist = descriptor.reach as f32/2.0 + descriptor.max_start_dist_from_entity.unwrap_or(0) as f32;
                                let dist_to_player = f32::sqrt((player_x - position_component.x).powf(2.0) + (player_y - position_component.y).powf(2.0));
                                if dist_to_player < max_dist {
                                    self.entity_attacks.borrow_mut().push(EntityAttackBox {
                                        archetype: attack_pattern.attacks[attack_component.cur_attack].clone(),
                                        x: player_x,
                                        y: player_y,
                                        time_charged: 0.0,
                                        rotation: angle,
                                    });
                                } else{
                                    self.entity_attacks.borrow_mut().push(
                                        EntityAttackBox {
                                            archetype: attack_pattern.attacks[attack_component.cur_attack].clone(),
                                            x: position_component.x + direction_to_player[0] * (max_dist),
                                            y: position_component.y + direction_to_player[1] * (max_dist),
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
                                        x: position_component.x + angle.cos() * (descriptor.reach as f32/2.0),
                                        y: position_component.y + angle.sin() * (descriptor.reach as f32/2.0),
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
                    }
                }
            }
        }
        // dot updates
        entities_to_update_index = 0;
        for (i, position_component, mut damageable_component) in izip!(
            self.components.position_components.iter(),
            self.components.damageable_components.iter()
        ).enumerate().filter_map(
            |(i, (position_component, damageable_component))|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && position_component.is_some() && damageable_component.is_some() {entities_to_update_index += 1; Some((i, position_component.as_ref().unwrap().borrow(), damageable_component.as_ref().unwrap().borrow_mut()))}
            else {None}
        ){
            let mut poison_tick = 0.0;
            damageable_component.poisons.retain_mut(|poison| {
                poison.time_alive += 1.0;
                if poison.time_alive >= poison.lifetime {
                    return false;
                }
                if poison.time_alive % poison.time_per_tick < 1.0 {
                    poison_tick += poison.damage;
                }
                true
            });
            if poison_tick.abs() > 0.0 {
                self.damage_entity_dot(&position_component, &mut damageable_component, poison_tick, camera, [0.6, 0.0, 0.8, 1.0]);
            }
            let mut remove_fire = false;
            let fire_tick = if let Some(fire) = &mut damageable_component.fire {
                fire.time_alive += 1.0;
                if fire.time_alive >= fire.lifetime {
                    remove_fire = true;
                }
                if fire.time_alive % fire.time_per_tick < 1.0 {
                    fire.damage
                }else{0.0}
            }else{0.0};
                
            if fire_tick.abs() > 0.0 {
                self.damage_entity_dot(&position_component, &mut damageable_component, fire_tick, camera, [1.0, 0.4, 0.0, 1.0]);
            }
            if remove_fire {
                damageable_component.fire = None;
            }
        }
        // anim frame updates
        for mut anim_component in self.components.animation_components.iter().enumerate().filter_map(
            |(i, anim_component)|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && anim_component.is_some() {entities_to_update_index += 1; Some(anim_component.as_ref().unwrap().borrow_mut())}
            else {None}
        ){
            anim_component.animation_frame += 1;
            anim_component.animation_frame %= 120;
        }
        // attack cooldown updates
        entities_to_update_index = 0;

        for mut attack_component in self.components.attack_components.iter().enumerate().filter_map(
            |(i, attack_component)|
            if entities_to_update_index == entities_to_update.len() {None}
            else if i == entities_to_update[entities_to_update_index] && attack_component.is_some() {entities_to_update_index += 1; Some(attack_component.as_ref().unwrap().borrow_mut())}
            else {None}
        ){
            if attack_component.cur_attack_cooldown > 0.0 {
                attack_component.cur_attack_cooldown -= 1.0/60.0;
            }
        }


        Ok(())
    }
    pub fn is_line_of_sight(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Result<bool, PError>{
        let mut x = x1;
        let mut y = y1;
        let dx = x2 - x1;
        let dy = y2 - y1;
        let steps = (f32::max(dx.abs(), dy.abs())/32.0) as i32;
        let dx = dx / steps as f32;
        let dy = dy / steps as f32;
        for _ in 0..steps{
            x += dx;
            y += dy;
            if ptry!(self.check_collision(true, None, x.floor(), y.floor(), 32, 32, false)){
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub fn move_entity_towards_player(&self, entity_id: &usize,collision_box: &CollisionBox, position_component: &mut PositionComponent, pathfinding_component: &mut PathfindingComponent, chunkref: &mut std::cell::RefMut<'_, Vec<Chunk>>, player_x: f32, player_y: f32, respects_collision: bool, has_collision: bool) -> Result<(), PError>{
        let direction: [f32; 2] = [player_x - position_component.x, player_y - position_component.y];
        let entity_pathfinding_frame = punwrap!(self.pathfinding_frames.get(entity_id), Expected, "all entities that follow player should have a pathfinding frame, entity with id {} doesn't, was the entity properly created?", entity_id);
        if direction[0] == 0.0 && direction[1] == 0.0 {
            return Ok(());
        }
        let movement_speed = pathfinding_component.movement_speed;
        if self.pathfinding_frame != *entity_pathfinding_frame && respects_collision{
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
                let direction: EntityDirectionOptions= ptry!(pathfinding::pathfind_by_block(position_component, *collision_box, *entity_id, self));
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
                let direction: EntityDirectionOptions = ptry!(pathfinding::pathfind_high_granularity(position_component, *collision_box,*entity_id, self));
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
        assert_eq!(self.components.add_entity(), self.element_id - 1);
        self.components.position_components.get_mut(self.element_id - 1).unwrap().replace(RefCell::new(PositionComponent{x, y}));
        self.pathfinding_frames.insert(self.element_id - 1, new_entity_pathfinding_frame);
        self.element_id - 1
    }
    pub fn add_pathfinding_component(&mut self, entity_id: usize, new_entity_pathfinding: PathfindingComponent){
        self.components.pathfinding_components.insert(entity_id, Some(RefCell::new(new_entity_pathfinding)));
    }
    pub fn add_attack_component(&mut self, entity_id: usize, new_entity_attack: EntityAttackComponent){
        self.components.attack_components.insert(entity_id, Some(RefCell::new(new_entity_attack)));
    }
    pub fn add_damageable_component(&mut self, entity_id: usize, new_entity_damageable: entity_components::DamageableComponent){
        self.components.damageable_components.insert(entity_id, Some(RefCell::new(new_entity_damageable)));
    }   
    pub fn create_entity_with_archetype(&mut self, x: f32, y: f32, archetype: CompactString) -> Result<usize, PError>{
        let entity = self.add_entity(x, y);
        let archetype = self.entity_archetype_descriptor_lookup.get(&archetype).expect("Archetype not found");

        let mut has_collision = false;
        let mut respects_collision = false;
        let mut aggressive = false;
        let mut damageable = false;
        let mut attacker = false;
        
        for tag in archetype.basic_tags.iter() {
            let tag = tag.as_str();
            match tag {
                "respectsCollision" => {
                    respects_collision = true;
                },
                "hasCollision" => {
                    has_collision = true;
                },
                "aggressive" => {
                    aggressive = true;
                },
                "damageable" => {
                    damageable = true;
                },
                "attacker" => {
                    attacker = true;
                },
                _ => {}
            }
        }

        if attacker {
            self.components.attack_components.insert(entity, Some(RefCell::new(EntityAttackComponent{
                cur_attack: 0,
                cur_attack_cooldown: 0.0,
                entity_attack_pattern: punwrap!(archetype.attack_pattern.clone(), JSONValidationError, "entity archetype {} has attacker tag but no attack pattern", archetype.name),
                attack_range: punwrap!(archetype.range, JSONValidationError, "entity archetype {} has attacker tag but no range", archetype.name),
            })));
        }
        if has_collision {
            if let Some(collision_box) = archetype.collision_box {
                self.components.collision_components.insert(entity, Some(RefCell::new(super::components::CollisionComponent{
                    collision_box,
                    respects_collision,
                })));
            } else {
                self.components.collision_components.insert(entity, Some(RefCell::new(super::components::CollisionComponent{
                    collision_box: CollisionBox {
                        x_offset: 0.0,
                        y_offset: 0.0,
                        w: 32.0,
                        h: 32.0,
                    },
                    respects_collision,
                })));
            }
        }
        if let Some(sprite) = &archetype.sprite {
            self.components.sprite_components.insert(entity, Some(RefCell::new(super::components::SpriteComponent {
                sprite: punwrap!(self.sprites.get_sprite_id(sprite), JSONValidationError, "entity archetype {} refers to sprite {} but that sprite doesn't exist", archetype.name, sprite)
            })));
        }
        if aggressive {
            self.components.aggro_components.insert(entity, Some(RefCell::new(entity_components::AggroComponent{
                aggroed: false,
                aggro_through_walls: respects_collision,
                aggro_range: punwrap!(archetype.aggro_range, JSONValidationError, "entity archetype {} has aggressive tag but no aggro range", archetype.name),
            })));
            self.components.pathfinding_components.insert(entity, Some(RefCell::new(entity_components::PathfindingComponent {
                cur_direction: EntityDirectionOptions::None,
                movement_speed: punwrap!(archetype.movement_speed, JSONValidationError, "entity archetype {} has aggressive tag but no movement speed, give it a movement speed of 0 if you don't want it to move", archetype.name)
        })));
        }
        if damageable {
            self.components.damageable_components.insert(entity, Some(RefCell::new(entity_components::DamageableComponent{
                health: punwrap!(archetype.health, JSONValidationError, "entity archetype {} has damageable tag but no max health", archetype.name) as f32,
                max_health: punwrap!(archetype.health, JSONValidationError, "entity archetype {} has damageable tag but no max_health", archetype.name),
                damage_box: archetype.damage_box.unwrap_or(CollisionBox {
                    x_offset: 0.0,
                    y_offset: 0.0,
                    w: 32.0,
                    h: 32.0,
                }),
                poisons: vec![],
                fire: None
            })));
        }

        if !archetype.loot_table.is_empty(){
            self.components.loot_components.insert(entity, Some(RefCell::new(super::components::LootComponent {
                loot_tables: archetype.loot_table.clone()
            }))); 
        }
        Ok(entity)
    }
    pub fn add_entity_archetype(&mut self, name: CompactString, archetype: entity_archetype_json){
        self.entity_archetype_descriptor_lookup.insert(name, archetype);
    }
    pub fn add_aggro_component(&mut self, entity_id: usize, new_entity_aggro: entity_components::AggroComponent){
        self.components.aggro_components.insert(entity_id, Some(RefCell::new(new_entity_aggro)));
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

#[derive(Clone, Debug, PartialEq)]
pub struct EntityAttackPattern {
    pub attacks: Vec<CompactString>,
    pub attack_cooldowns: Vec<f32>,  
}
impl Default for EntityAttackPattern{
    fn default() -> Self{
        Self{
            attacks: Vec::new(),
            attack_cooldowns: Vec::new(),
        }
    }
}
impl EntityAttackPattern{
    pub fn new(attacks: Vec<CompactString>, attack_cooldowns: Vec<f32>) -> Self{
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
