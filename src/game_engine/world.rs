use core::f32;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::{RefCell, RefMut};
use std::f32::consts::PI;

use crate::error::PError;
use crate::{error_prolif_allow, ptry, punwrap};
use crate::rendering_engine::abstractions::SpriteContainer;
use crate::entities::EntityTags;
use crate::game_engine::player::Player;
use crate::game_engine::terrain::{Terrain, TerrainTags};

use super::camera::Camera;
use super::entities::AttackType;
use super::entity_attacks::{EntityAttackBox, EntityAttackDescriptor};
use super::entity_components::{self, AggroComponent, CollisionBox, HealthComponent, PositionComponent};
use super::game::MousePosition;
use super::inventory::Inventory;
use super::item::{Item, ItemArchetype, ItemType};
use super::items_on_floor::ItemOnFloor;
use super::loot::LootTable;
use super::player_attacks::PlayerAttack;
use super::stat::StatList;
use super::utils::{self, Rectangle};

#[derive(Debug, Clone)]
pub struct DamageTextDescriptor {
    pub world_text_id: usize, 
    pub lifespan: f32
}

#[derive(Debug, Clone)]
pub struct Chunk{  
    pub chunk_id: usize,
    x: usize,
    y: usize,
    pub terrain_ids: Vec<usize>,
    pub entities_ids: Vec<usize>,
    
}

pub struct World{
    pub chunks: RefCell<Vec<Chunk>>,
    pub player: RefCell<Player>,
    pub element_id: usize,
    pub chunk_lookup: RefCell<FxHashMap<[usize; 2],usize>>, // corresponds chunk x,y to id

    pub inventory: Inventory,
    pub item_archetype_lookup: FxHashMap<String, ItemArchetype>,

    pub collision_cache: RefCell<FxHashMap<[usize; 2], Vec<usize>>>,
    pub damage_cache: RefCell<FxHashMap<[usize; 2], Vec<usize>>>, 
   
    pub pathfinding_frames: FxHashMap<usize, usize>, // entity id to frame of pathfinding
    pub next_pathfinding_frame_for_entity: usize,
    pub pathfinding_frame: usize,
    
    pub level_editor: bool,

    pub loaded_chunks: Vec<usize>, // DANGEROUS: chunk ids that are currently loaded, this is created as a SIDE EFFECT of the camera, and should not be edited in the world
    
    pub terrain: FxHashMap<usize, Terrain>, // corresponds element id to Terrain element
    pub terrain_archetype_tags_lookup: Vec<Vec<TerrainTags>>,
    pub terrain_archetype_lookup: FxHashMap<usize, usize>,

    pub entity_archetype_tags_lookup: FxHashMap<String,Vec<EntityTags>>, // corresponds entity_archetype name to the entity's tags
    pub entity_archetype_lookup: FxHashMap<usize,String>, // corresponds element_ids to entity_archetype

    pub entity_position_components: FxHashMap<usize, RefCell<entity_components::PositionComponent>>,
    pub entity_attack_components: FxHashMap<usize, RefCell<entity_components::EntityAttackComponent>>,
    pub entity_health_components: FxHashMap<usize, RefCell<entity_components::HealthComponent>>,
    pub entity_pathfinding_components: FxHashMap<usize, RefCell<entity_components::PathfindingComponent>>,
    pub entity_aggro_components: FxHashMap<usize, RefCell<entity_components::AggroComponent>>,

    pub sprites: SpriteContainer,
    pub sprite_lookup: FxHashMap<usize, usize>, // corresponds element id to sprite id

    pub player_attacks: RefCell<Vec<PlayerAttack>>,
    pub entities_to_be_killed_at_end_of_frame: RefCell<Vec<usize>>,

    pub entity_attacks: RefCell<Vec<EntityAttackBox>>,
    pub entity_attack_descriptor_lookup: FxHashMap<String, EntityAttackDescriptor>,

    pub damage_text: RefCell<Vec<DamageTextDescriptor>>,

    pub items_on_floor: RefCell<Vec<ItemOnFloor>>,

    pub loot_table_lookup: Vec<LootTable>, // loot table id to loot table object
}

impl World{ 
    pub fn new(player: Player, sprite_container: SpriteContainer) -> Self{
        let iof = vec![ItemOnFloor {
            x: 700.0,
            y: 500.0,
            item: Item {
                name: String::from("test1"),
                attack_sprite: Some(String::from("melee_attack")),
                item_type: ItemType::MeleeWeapon,
                width_to_length_ratio: None,
                lore: String::from("test"),
                sprite: String::from("sword"),
                stats: crate::create_stat_list!(
                    damage => 150.0,
                    width => 50.0,
                    reach => 65.0,
                    cooldown => 10.0,
                ),
                time_til_usable: 10.0,
            }
        }];

        Self{
            chunks: RefCell::new(Vec::new()),
            player: RefCell::new(player),
            element_id: 0, 
            sprites: sprite_container,
            sprite_lookup: FxHashMap::default(),
            chunk_lookup: RefCell::new(FxHashMap::default()),
            entity_archetype_lookup: FxHashMap::default(),
            entity_archetype_tags_lookup: FxHashMap::default(),
            terrain_archetype_tags_lookup: Vec::new(),
            terrain_archetype_lookup: FxHashMap::default(),
            terrain: FxHashMap::default(),
            inventory: Inventory::default(),
            item_archetype_lookup: FxHashMap::default(),
            loaded_chunks: Vec::new(),
            collision_cache: RefCell::new(FxHashMap::default()),
            damage_cache: RefCell::new(FxHashMap::default()),
            pathfinding_frames: FxHashMap::default(),
            next_pathfinding_frame_for_entity: 0,
            pathfinding_frame: 0,
            level_editor: false,
            entity_attack_components: FxHashMap::default(),
            entity_health_components: FxHashMap::default(),
            entity_position_components: FxHashMap::default(),
            entity_pathfinding_components: FxHashMap::default(),
            entity_aggro_components: FxHashMap::default(),
            player_attacks: RefCell::new(Vec::new()),
            entities_to_be_killed_at_end_of_frame: RefCell::new(Vec::new()),
            entity_attacks: RefCell::new(Vec::new()),
            entity_attack_descriptor_lookup: FxHashMap::default(),
            damage_text: RefCell::new(Vec::new()),
            items_on_floor: RefCell::new(iof),
            loot_table_lookup: Vec::new(),
        }
    }
    pub fn new_chunk(&self, chunk_x: usize, chunk_y: usize, chunkref: Option<&mut std::cell::RefMut<'_, Vec<Chunk>>>) -> usize{
        if chunkref.is_none(){
            let new_chunk_id = self.chunks.borrow().len(); 
            self.chunks.borrow_mut().push(
                Chunk{
                    chunk_id: new_chunk_id,
                    x: chunk_x,
                    y: chunk_y,
                    terrain_ids: Vec::new(),
                    entities_ids: Vec::new(),
                });
            self.chunk_lookup.borrow_mut().insert([chunk_x, chunk_y], new_chunk_id);
            new_chunk_id
        }else{
            let cr = chunkref.unwrap();
            let new_chunk_id = cr.len(); 
            cr.push(
                Chunk{
                    chunk_id: new_chunk_id,
                    x: chunk_x,
                    y: chunk_y,
                    terrain_ids: Vec::new(),
                    entities_ids: Vec::new(),
                });
            self.chunk_lookup.borrow_mut().insert([chunk_x, chunk_y], new_chunk_id);
            new_chunk_id
        }
    }
    pub fn remove_terrain(&mut self, element_id: usize) -> Result<(), PError>{
        let terrain = punwrap!(self.terrain.get(&element_id), NotFound, "Tried to remove terrain with id {}, but it wasn't found", element_id);
        let chunk_id = punwrap!(self.get_chunk_from_xy(terrain.x, terrain.y), Invalid, "Tried to remove terrain with id {}, but it wasn't in a chunk?", element_id);
        let chunk = &mut self.chunks.borrow_mut()[chunk_id];
        let index = punwrap!(chunk.terrain_ids.iter().position(|&x| x == element_id), Invalid, "Tried to remove terrain with id {}, but it wasn't in the chunk expected, chunk with id {}", element_id, chunk_id);
        chunk.terrain_ids.remove(index);
        self.terrain.remove(&element_id);
        self.terrain_archetype_lookup.remove(&element_id);
        Ok(())
    }
    pub fn set_loaded_chunks(&mut self, chunk_ids: Vec<usize>){
        self.loaded_chunks = chunk_ids;
    }
    pub fn get_terrain(&self, element_id: usize) -> Option<&Terrain>{
        self.terrain.get(&element_id)
    }
    pub fn coord_to_chunk_coord(coord: usize) -> usize{
        (coord as f32 / 1024.0).floor() as usize
    }
    pub fn get_chunk_from_xy(&self, x: usize, y: usize) -> Option<usize>{
        let chunk_x: usize = World::coord_to_chunk_coord(x);
        let chunk_y: usize = World::coord_to_chunk_coord(y);
        self.chunk_lookup.borrow().get(&[chunk_x, chunk_y]).copied()
    }
    pub fn get_chunk_from_chunk_xy(&self, x: usize, y: usize) -> Option<usize>{
        self.chunk_lookup.borrow().get(&[x, y]).copied()
    }
    pub fn add_terrain(&mut self, x: usize, y: usize) -> usize{
        
        let new_terrain: Terrain = Terrain{ element_id: self.element_id, x, y };
        
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_chunk_xy(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y));
        
        let chunk_id: usize = if let Some(chunk_id) = chunk_id_potentially{
            chunk_id
        }else {
            self.new_chunk(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y), None)
        };

        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].terrain_ids.push(self.element_id - 1);
        self.terrain.insert(self.element_id - 1, new_terrain);
        self.element_id - 1
    }
    pub fn add_terrain_archetype(&mut self, tags: Vec<TerrainTags>) -> usize{
        self.terrain_archetype_tags_lookup.push(tags);
        self.terrain_archetype_tags_lookup.len() - 1
    }
    pub fn set_terrain_archetype(&mut self, id: usize, archetype_id: usize){
        self.terrain_archetype_lookup.insert(id, archetype_id);
    }
    pub fn get_terrain_tags(&self, id: usize) -> Option<&Vec<TerrainTags>>{
        let potential_archetype = self.terrain_archetype_lookup.get(&id)?;
        self.terrain_archetype_tags_lookup.get(*potential_archetype)
    }
    pub fn get_terrain_archetype(&self, id: usize) -> Option<&usize> {
        self.terrain_archetype_lookup.get(&id)
    }
    pub fn get_archetype_tags(&self, archetype: usize) -> Option<&Vec<TerrainTags>>{
        self.terrain_archetype_tags_lookup.get(archetype)
    }
    pub fn get_terrain_tiles(x: usize, y: usize, w: usize, h: usize) -> Vec<[usize; 2]>{
        let mut tiles: Vec<[usize; 2]> = Vec::new();
        let left_x = (x as f32 / 32.0).floor() as usize;
        let right_x = ((x as f32 + w as f32) / 32.0).floor() as usize;
        let top_y = (y as f32 / 32.0).floor() as usize;
        let bot_y = ((y as f32 + h as f32)/ 32.0).floor() as usize;
        for x in left_x..=right_x{
            for y in top_y..=bot_y{
                tiles.push([x,y]);
            }
        }
        tiles
    }
    pub fn get_collision_tiles_rotated_rect(x: usize, y:usize, w:usize, h:usize, rotation: f32) -> Vec<[usize; 2]> {
        let corners = super::utils::get_rotated_corners(&Rectangle { x: x as f32, y: y as f32, width: w as f32, height: h as f32, rotation });
        let mut left_most_x = None;
        let mut right_most_x = None;
        let mut top_most_y = None;
        let mut bot_most_y = None;
        for corner in corners {
            if corner.0 < left_most_x.unwrap_or(f32::MAX){
                left_most_x = Some(corner.0);
            }
            if corner.0 > right_most_x.unwrap_or(f32::MIN) {
                right_most_x = Some(corner.0);
            }
            if corner.1 > bot_most_y.unwrap_or(f32::MIN) {
                bot_most_y = Some(corner.1);
            }
            if corner.1 < top_most_y.unwrap_or(f32::MAX) || top_most_y.is_none() {
                top_most_y = Some(corner.1);
            }
        }
        World::get_terrain_tiles(left_most_x.unwrap().floor() as usize, top_most_y.unwrap().floor() as usize, (right_most_x.unwrap() - left_most_x.unwrap()).ceil() as usize, (bot_most_y.unwrap() - top_most_y.unwrap()).ceil() as usize)
    }
    pub fn generate_collision_cache_and_damage_cache(&mut self) -> Result<(), PError>{
        let mut collision_cache_ref = self.collision_cache.borrow_mut();
        let mut damage_cache_ref = self.damage_cache.borrow_mut();
        collision_cache_ref.clear();
        damage_cache_ref.clear();
        for chunk_id in self.loaded_chunks.iter(){ 
            let chunk = &self.chunks.borrow()[*chunk_id];
            for terrain_id in chunk.terrain_ids.iter(){
                let terrain = punwrap!(self.terrain.get(terrain_id), Invalid, "chunk with id {} refers to terrain with id {}, but there is no terrain with id {}", chunk_id, terrain_id, terrain_id);
                let terrain_tags_potentially = self.get_terrain_tags(*terrain_id);
                if terrain_tags_potentially.is_none(){
                    continue;
                }
                let terrain_tags = terrain_tags_potentially.unwrap();
                for tag in terrain_tags.iter(){
                    match tag{
                        TerrainTags::BlocksMovement => {
                        let tiles_blocked: Vec<[usize; 2]> = World::get_terrain_tiles(terrain.x, terrain.y, 32, 32);
                        for tile in tiles_blocked.iter(){
                            let collision_cache_entry = collision_cache_ref.get_mut(&[tile[0],tile[1]]);
                            if let Some(entry) = collision_cache_entry {
                                entry.push(*terrain_id);
                            }else{
                                collision_cache_ref.insert([tile[0],tile[1]], vec![*terrain_id]);
                            }
                        }
                        }
                        _ => ()
                    }
                }
            }

            for entity_id in chunk.entities_ids.iter(){
                let position_component = punwrap!(self.entity_position_components.get(entity_id), Expected, "all entities should have a position component").borrow();
                
                let entity_tags_potentially = self.get_entity_tags(*entity_id);
                if entity_tags_potentially.is_none(){
                    continue;
                }
                let entity_tags = entity_tags_potentially.unwrap();
                for tag in entity_tags.iter(){
                    match tag{
                        EntityTags::HasCollision(cbox) => {
                            let tiles_blocked: Vec<[usize; 2]> = World::get_terrain_tiles((position_component.x + cbox.x_offset) as usize, (position_component.y + cbox.y_offset) as usize, cbox.w as usize, cbox.h as usize);
                            for tile in tiles_blocked.iter(){
                                let collision_cache_entry = collision_cache_ref.get_mut(&[tile[0],tile[1]]);
                                if let Some(entry) = collision_cache_entry {
                                    entry.push(*entity_id);
                                }else{
                                    collision_cache_ref.insert([tile[0],tile[1]], vec![*entity_id]);
                                }
                            }
                        },
                        EntityTags::Damageable(dbox) => {
                            let tiles_blocked: Vec<[usize; 2]> = World::get_terrain_tiles((position_component.x + dbox.x_offset) as usize, (position_component.y + dbox.y_offset) as usize, dbox.w as usize, dbox.h as usize);
                            for tile in tiles_blocked.iter(){
                                let damage_cache_entry = damage_cache_ref.get_mut(&[tile[0],tile[1]]);
                                if let Some(entry) = damage_cache_entry {
                                    entry.push(*entity_id);
                                }else{
                                    damage_cache_ref.insert([tile[0],tile[1]], vec![*entity_id]);
                                }
                            }
                        }
                        _ => ()
                    }
                }
            }
        }  
        Ok(())
    }
    pub fn check_collision(&self, player: bool, id_to_ignore: Option<usize>, x: f32, y: f32, w: usize, h: usize, entity: bool) -> Result<bool, PError>{
        if !player {
            let player = self.player.borrow();
            let pw = player.collision_box.w;
            let ph = player.collision_box.h;
            let px = player.x + player.collision_box.x_offset;
            let py = player.y + player.collision_box.y_offset;
            if px.floor() - 1.0 < (x + w as f32) && px.floor() + pw + 1.0 > x && py.floor() - 1.0 < (y + h as f32) && py.floor() + ph + 1.0 > y{
                return Ok(true);
            }
        }
        let tiles_to_check = World::get_terrain_tiles(x.floor() as usize, y.floor() as usize, w, h);
        let mut ids_to_check: Vec<usize> = Vec::new();
        for tile in tiles_to_check.iter(){
            if self.collision_cache.borrow().get(&[tile[0],tile[1]]).is_none(){
                continue;
            }else{
                ids_to_check.extend(self.collision_cache.borrow().get(&[tile[0],tile[1]]).unwrap());
            }
        }
        let idti: usize = id_to_ignore.unwrap_or(usize::MAX);
        for id in ids_to_check{
            if id == idti{
                continue;
            }
            let terrain_potentially = self.terrain.get(&id);
            
            if terrain_potentially.is_none(){
                if entity{
                    let entity_collision_box = punwrap!(self.get_entity_collision_box(id), Invalid, "all entities in the collision cache should have a collision box, but entity with id {} does not have one", id);
                    let entity_position = punwrap!(self.entity_position_components.get(&id), Expected, "all entities should have a position component").borrow();
                    let ex = entity_position.x + entity_collision_box.x_offset;
                    let ey = entity_position.y + entity_collision_box.y_offset;
                    let ew = entity_collision_box.w;
                    let eh = entity_collision_box.h;
                    if ex < (x + w as f32) && ex + ew > x && ey < (y + h as f32) && ey + eh > y{
                        return Ok(true);
                    }
                }
            }else{
                let terrain = terrain_potentially.unwrap();
                if (terrain.x as f32) < (x + w as f32) && terrain.x as f32 + 32.0 > x && (terrain.y as f32) < (y + h as f32) && (terrain.y as f32 + 32.0) > y{
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    pub fn check_collision_non_damageable(&self, player: bool, id_to_ignore: Option<usize>, x: usize, y: usize, w: usize, h: usize, entity: bool) -> bool{
        if !player {
            let player = self.player.borrow();
            let pw = player.collision_box.w;
            let ph = player.collision_box.h;
            let px = player.x + player.collision_box.x_offset;
            let py = player.y + player.collision_box.y_offset;
            if px.floor() - 1.0 < (x + w) as f32 && px.floor() + pw + 1.0 > x as f32 && py.floor() - 1.0 < (y + h) as f32 && py.floor() + ph + 1.0 > y as f32{
                return true;
            }
        }
        let tiles_to_check = World::get_terrain_tiles(x, y, w, h);
        let mut ids_to_check: Vec<usize> = Vec::new();
        for tile in tiles_to_check.iter(){
            if self.collision_cache.borrow().get(&[tile[0],tile[1]]).is_none(){
                continue;
            }else{
                ids_to_check.extend(self.collision_cache.borrow().get(&[tile[0],tile[1]]).unwrap());
            }
        }
        let idti: usize = id_to_ignore.unwrap_or(usize::MAX);
        for id in ids_to_check{
            if id == idti{
                continue;
            }
            let terrain_potentially = self.terrain.get(&id);
            
            if terrain_potentially.is_none(){
                if entity{
                    let entity_collision_box = self.get_entity_collision_box(id).unwrap();
                    let entity_position = self.entity_position_components.get(&id).unwrap().borrow();
                    let entity_tags = self.get_entity_tags(id).unwrap();
                    let mut damageable = false;
                    for tag in entity_tags.iter(){
                        match tag{
                            EntityTags::Damageable(_) => {
                            damageable = true;
                            }
                            _ => ()
                        }
                    }
                    if damageable {
                        continue;
                    }
                    let ex = entity_position.x + entity_collision_box.x_offset;
                    let ey = entity_position.y + entity_collision_box.y_offset;
                    let ew = entity_collision_box.w;
                    let eh = entity_collision_box.h;
                    if ex < (x + w) as f32 && ex + ew > x as f32 && ey < (y + h) as f32 && ey + eh > y as f32{
                        return true;
                    }
                }
                
            }else{
                let terrain = terrain_potentially.unwrap();
                if terrain.x < x + w && terrain.x + 32 > x && terrain.y < y + h && terrain.y + 32 > y{
                    return true;
                }
            }
        }
        false
    }
    
    pub fn get_entity_damage_box(&self, id: usize) -> Option<&CollisionBox> {
        let entity_tags = self.get_entity_tags(id)?;
        for tag in entity_tags.iter(){
            match tag{
                EntityTags::Damageable(dbox) => {
                return Some(dbox);
            }
                _ => ()
        }
        }
        None
    }
    pub fn get_entity_collision_box(&self, id: usize) -> Option<&CollisionBox>{
        let entity_tags = self.get_entity_tags(id)?;
        for tag in entity_tags.iter(){
            match tag{
                EntityTags::HasCollision(cbox) => {
                return Some(cbox);
                }
                _ => ()
            }
        }
        None
    }
    pub fn get_attacked_rotated_rect(&self, player: bool, id_to_ignore: Option<usize>, x: usize, y: usize, w: usize, h: usize, rotation: f32, entity: bool) -> Vec<usize>{
        if !player {
            unimplemented!("non-player get attacks not implemented");
        }
        let tiles_to_check = World::get_collision_tiles_rotated_rect(x, y, w, h, rotation);
        let mut ids_to_check = FxHashSet::default();
        for tile in tiles_to_check.iter(){
            if self.damage_cache.borrow().get(&[tile[0],tile[1]]).is_none(){
                continue;
            }else{
                ids_to_check.extend(self.damage_cache.borrow().get(&[tile[0],tile[1]]).unwrap());
            }
        }
        let idti: usize = id_to_ignore.unwrap_or(usize::MAX);
        let mut colliding = Vec::new();
        for id in ids_to_check{
            if id == idti{
                continue;
            }
            let terrain_potentially = self.terrain.get(&id);
            
            if terrain_potentially.is_none(){
                if entity{
                    let entity_damage_box = self.get_entity_damage_box(id).expect("All entities in damage cache should have damage boxes");
                    let entity_position = self.entity_position_components.get(&id).expect("All entities in damage cache should have position components").borrow();
                    let ex = entity_position.x + entity_damage_box.x_offset;
                    let ey = entity_position.y + entity_damage_box.y_offset;
                    let ew = entity_damage_box.w;
                    let eh = entity_damage_box.h;
                    if super::utils::check_collision(&Rectangle {
                        x: x as f32, y: y as f32, width: w as f32, height: h as f32, rotation },
                        &Rectangle {
                            x: ex, y: ey, width: ew, height: eh, rotation: 0.0
                        }
                    ){
                        colliding.push(id);
                    }
                }
                
            }else{
                let terrain = terrain_potentially.unwrap();
                if super::utils::check_collision(&Rectangle {
                    x: x as f32, y: y as f32, width: w as f32, height: h as f32, rotation },
                    &Rectangle {
                        x: terrain.x as f32, y: terrain.y as f32, width: 32.0, height: 32.0, rotation: 0.0
                    }
                ){
                    colliding.push(id);
                }
            }
        }
        colliding
    }
    pub fn get_attacked(&self, player: bool, id_to_ignore: Option<usize>, x: usize, y: usize, w: usize, h: usize, entity: bool) -> Vec<usize>{
        if !player {
            unimplemented!("non-player get_attack not implemented");
        }
        let tiles_to_check = World::get_terrain_tiles(x, y, w, h);
        let mut ids_to_check = FxHashSet::default();
        for tile in tiles_to_check.iter(){
            if self.damage_cache.borrow().get(&[tile[0],tile[1]]).is_none(){
                continue;
            }else{
                ids_to_check.extend(self.damage_cache.borrow().get(&[tile[0],tile[1]]).unwrap());
            }
        }
        let idti: usize = id_to_ignore.unwrap_or(usize::MAX);
        let mut colliding = Vec::new();
        for id in ids_to_check{
            if id == idti{
                continue;
            }
            let terrain_potentially = self.terrain.get(&id);
            
            if terrain_potentially.is_none(){
                if entity{
                    let entity_damage_box = self.get_entity_damage_box(id).expect("All entities in damage cache should have damage boxes");
                    let entity_position = self.entity_position_components.get(&id).expect("All entities in damage cache should have position components").borrow();
                    let ex = entity_position.x + entity_damage_box.x_offset;
                    let ey = entity_position.y + entity_damage_box.y_offset;
                    let ew = entity_damage_box.w;
                    let eh = entity_damage_box.h;
                    if ex < (x + w) as f32 && ex + ew > x as f32 && ey < (y + h) as f32 && ey + eh > y as f32{
                        colliding.push(id);
                    }
                }
                
            }else{
                let terrain = terrain_potentially.unwrap();
                if terrain.x < x + w && terrain.x + 32 > x && terrain.y < y + h && terrain.y + 32 > y{
                    colliding.push(id);
                }
            }
        }
        colliding
    }
    pub fn check_collision_with_player(&self, x: f32, y: f32, w: f32, h: f32, rotation: f32) -> bool{
        let player = self.player.borrow();
        utils::check_collision(&Rectangle {
            x, y, width: w, height: h, rotation },
            &Rectangle {
                x: player.x + player.collision_box.x_offset,
                y: player.y + player.collision_box.y_offset,
                width: player.collision_box.w,
                height: player.collision_box.h,
                rotation: 0.0
            }
        )
    }
    pub fn attempt_move_player(&self, player: &mut Player, movement: [f32; 2]) -> Result<(), PError>{
        
        if ptry!(self.check_collision(true, None,player.x.floor() + movement[0] + player.collision_box.x_offset, player.y.floor() + movement[1] + player.collision_box.y_offset, player.collision_box.w.floor() as usize, player.collision_box.h.floor() as usize, true)){
            return Ok(());
        }
        player.x += movement[0];
        player.y += movement[1];
        Ok(())
    }
    pub fn can_move_player(&self, player: &mut Player, movement: [f32; 2]) -> Result<bool, PError>{
        if ptry!(self.check_collision(true, None,player.x.floor() + movement[0] + player.collision_box.x_offset, player.y.floor() + movement[1] + player.collision_box.y_offset, player.collision_box.w.floor() as usize, player.collision_box.h.floor() as usize, true)){
            return Ok(false);
        }
        Ok(true)
    }
    pub fn set_sprite(&mut self, element_id: usize, sprite_id: usize){
        self.sprite_lookup.insert(element_id, sprite_id);
    }
    pub fn get_sprite(&self, element_id: usize) -> Option<usize>{
        self.sprite_lookup.get(&element_id).copied()
    }
    pub fn process_player_input(&mut self, keys: &FxHashMap<String,bool>) -> Result<(), PError>{
        let mut direction: [f32; 2] = [0.0,0.0];
        let mut player: std::cell::RefMut<'_, Player> = self.player.borrow_mut();
        if *keys.get("w").unwrap_or(&false) || *keys.get("arrowup").unwrap_or(&false){
            direction[1] -= 1.0;
        }
        if *keys.get("a").unwrap_or(&false) || *keys.get("arrowleft").unwrap_or(&false){
            direction[0] -= 1.0;
        }
        if *keys.get("s").unwrap_or(&false) || *keys.get("arrowdown").unwrap_or(&false){
            direction[1] += 1.0;
        }
        if *keys.get("d").unwrap_or(&false) || *keys.get("arrowright").unwrap_or(&false){
            direction[0] += 1.0;
        }

        if direction[0] == 0.0 && direction[1] < 0.0{
            player.sprite_id = self.sprites.get_sprite_id("player_back").expect("Could not find sprite id for player_back");
        } else if direction[0] == 0.0 && direction[1] > 0.0 {
            player.sprite_id = self.sprites.get_sprite_id("player_front").expect("Could not find sprite id for player_front");
        } else if direction[0] > 0.0 {
            player.sprite_id = self.sprites.get_sprite_id("player_right").expect("Could not find sprite id for player_right");
        } else if direction[0] < 0.0{
            player.sprite_id = self.sprites.get_sprite_id("player_left").expect("Could not find sprite id for player_left");
        }
        let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        
        if magnitude > 0.0{
            let movement = [(direction[0] / magnitude * player.movement_speed), (direction[1] / magnitude * player.movement_speed)];
            let player_movement_speed = player.movement_speed;
            
            if !ptry!(self.can_move_player(&mut player, [movement[0], 0.0])){
                ptry!(self.attempt_move_player(&mut player, [0.0, (direction[1] * player_movement_speed)]));
            }else if !ptry!(self.can_move_player(&mut player, [0.0, movement[1]])){
                ptry!(self.attempt_move_player(&mut player, [(direction[0] * player_movement_speed), 0.0]));
            }else{
                ptry!(self.attempt_move_player(&mut player, movement));
            }
        }

        if player.y.floor() < player.movement_speed {
            player.y = player.movement_speed;
        }
        if player.x.floor() < player.movement_speed {
            player.x = player.movement_speed;
        }
        Ok(())
    }
   
    pub fn add_player_attack(&self, stats: &StatList, attack_item: &Item, x: f32, y: f32, angle: f32) -> Result<(), PError>{    
        match attack_item.item_type {
            ItemType::MeleeWeapon => {
                self.player_attacks.borrow_mut().push(
                    PlayerAttack::new(stats.clone(), AttackType::Melee, punwrap!(attack_item.attack_sprite.clone(), Expected, "all melee weapons should have an attack sprite"), attack_item.width_to_length_ratio.unwrap_or(1.0), x, y, angle)
                );
            }
            ItemType::RangedWeapon => {
                self.player_attacks.borrow_mut().push(
                    PlayerAttack::new(stats.clone(), AttackType::Ranged, punwrap!(attack_item.attack_sprite.clone(), Expected, "all ranged weapons should have an attack sprite"),attack_item.width_to_length_ratio.unwrap_or(1.0), x, y, angle)
                );
            }
            _ => {}
        }
        Ok(())
    }
    pub fn damage_player(&self, damage: f32) -> Result<(), PError> {
        let defense = ptry!(self.inventory.get_combined_stats()).defense.unwrap_or(0.0);
        self.player.borrow_mut().health -= damage * 100.0/(defense + 100.0);
        Ok(())
    }

    pub fn update_entity_attacks(&self) -> Result<(), PError>{
        let mut attacks = self.entity_attacks.borrow_mut();
       let mut attacks_to_be_deleted = Vec::new();
        for (i, attack) in attacks.iter_mut().enumerate(){
            attack.time_charged += 1.0;
            let descriptor = punwrap!(self.get_attack_descriptor(attack), Expected, "Couldn't find attack descriptor for entity attack: {:?}", attack);
            if attack.time_charged.floor() as usize >= descriptor.time_to_charge {
                if self.check_collision_with_player(attack.x, attack.y, descriptor.reach as f32, descriptor.width as f32, attack.rotation * 180.0/PI){
                    ptry!(self.damage_player(descriptor.damage));
                }
                attacks_to_be_deleted.push(i);
            }
        }
        for (offset, index) in attacks_to_be_deleted.iter().enumerate(){
            attacks.remove(*index - offset);
        }
        Ok(())
    }
    pub fn update_player_attacks(&self, camera: &mut Camera){
        let mut attacks = self.player_attacks.borrow_mut();
        let mut attacks_to_be_deleted = Vec::new();
        let mut i = 0;
        for attack in attacks.iter_mut(){
            match attack.attack_type{
                AttackType::Melee => {
                    attack.time_alive += 1.0;
                    if attack.time_alive > 3.0{
                        attacks_to_be_deleted.push(i);
                        i += 1;
                        continue;
                    }
                    if attack.dealt_damage {
                        continue;
                    }
                    if attack.time_alive < 2.0 {   
                        let height = attack.stats.reach.unwrap_or(0.0);
                        let width = attack.stats.width.unwrap_or(0.0);
                        let collisions = self.get_attacked_rotated_rect(true, None, attack.x as usize, attack.y as usize, height.floor() as usize, width.floor() as usize,attack.angle, true);
                        for collision in collisions.iter(){
                            if self.entity_health_components.contains_key(collision){
                                let health_component = self.entity_health_components.get(collision).unwrap().borrow_mut();
                                let entity_position = self.entity_position_components.get(collision).unwrap().borrow();
                                let aggro_potentially = self.entity_aggro_components.get(collision);
                                let mut aggro = None;
                                if aggro_potentially.is_some(){
                                    aggro = Some(aggro_potentially.unwrap().borrow_mut());
                                }
                                attack.dealt_damage = true;
                                self.damage_entity( &entity_position, Some(health_component), aggro,  attack.stats.damage.unwrap_or(0.0), camera);
                            }
                        }
                    }
                }
                AttackType::Ranged => {
                    let angle = attack.angle * PI/180.0;
                    attack.x += angle.cos() * attack.stats.speed.unwrap_or(0.0);
                    attack.y += angle.sin() * attack.stats.speed.unwrap_or(0.0);
                    attack.time_alive += 1.0;
                    if attack.time_alive > attack.stats.lifetime.unwrap_or(f32::MAX){
                        attacks_to_be_deleted.push(i);
                        i += 1;
                        continue;
                    }
                    if attack.dealt_damage{
                        continue;
                    }
                    let length = attack.stats.size.unwrap_or(0.0).floor() as usize;
                    let width = (attack.width_to_length_ratio * length as f32) as usize;
                    let collisions = self.get_attacked_rotated_rect(true, None, (attack.x - length as f32/2.0) as usize, (attack.y - width as f32 /2.0) as usize, length, width,attack.angle, true);
                    let mut hit = false;
                    for collision in collisions.iter(){
                        if self.entity_health_components.contains_key(collision){
                            hit = true;
                            attacks_to_be_deleted.push(i);
                            attack.dealt_damage = true;
                            break;
                        }
                    }
                    if hit {
                        for collision in collisions.iter(){
                            if self.entity_health_components.contains_key(collision){
                                let health_component = self.entity_health_components.get(collision).unwrap().borrow_mut();
                                let entity_position = self.entity_position_components.get(collision).unwrap().borrow();
                                let aggro_potentially = self.entity_aggro_components.get(collision);
                                let mut aggro = None;
                                if aggro_potentially.is_some(){
                                    aggro = Some(aggro_potentially.unwrap().borrow_mut());
                                }
                                self.damage_entity(&entity_position, Some(health_component), aggro, attack.stats.damage.unwrap_or(0.0), camera);
                            }
                        }
                        
                    }else {
                        let length = attack.stats.size.unwrap_or(0.0).floor();
                        let width = attack.width_to_length_ratio * length;
                        let c = self.check_collision_non_damageable(true, None, (attack.x - length/2.0) as usize, (attack.y-width/2.0) as usize, length as usize, width as usize, true);
                        if c{
                            attacks_to_be_deleted.push(i);
                        }
                    }
                }
                AttackType::Magic => {
                    todo!()
                }
               
            }
            i += 1;
        }
        for (offset, index) in attacks_to_be_deleted.iter().enumerate(){
            attacks.remove(*index - offset);
        }
    }

    pub fn damage_entity(&self, entity_position_component: &PositionComponent, entity_health_component: Option<RefMut<HealthComponent>>, entity_aggro_component: Option<RefMut<AggroComponent>>, damage: f32, camera: &mut Camera){
        if entity_health_component.is_some() {
            let mut ehc = entity_health_component.unwrap();
            ehc.health -= damage;
            if ehc.health >= ehc.max_health as f32 {
                ehc.health = ehc.max_health as f32;
            }
        }
        let text_1 = camera.add_world_text(((damage * 10.0).round() / 10.0).to_string(), super::camera::Font::B, entity_position_component.x + 11.0, entity_position_component.y + 7.0, 150.0, 50.0, 50.0, [0.0, 0.0, 0.0, 1.0], wgpu_text::glyph_brush::HorizontalAlign::Center);
        let text_2 = camera.add_world_text(((damage * 10.0).round() / 10.0).to_string(), super::camera::Font::B, entity_position_component.x + 9.0, entity_position_component.y + 5.0, 150.0, 50.0, 50.0, [1.0, 1.0, 1.0, 1.0], wgpu_text::glyph_brush::HorizontalAlign::Center);
        if entity_aggro_component.is_some() {
            let mut aggro = entity_aggro_component.unwrap();
            if !aggro.aggroed{
                aggro.aggroed = true;
            }
        }
        self.damage_text.borrow_mut().push(DamageTextDescriptor{world_text_id: text_1, lifespan: 0.0});
        self.damage_text.borrow_mut().push(DamageTextDescriptor{world_text_id: text_2, lifespan: 0.0});
    }
    
    pub fn remove_entity(&mut self, entity_id: usize) -> Result<(), PError>{
        let entity_position = *punwrap!(self.entity_position_components.get(&entity_id), Expected, "all entities that are being killed should have a position component").borrow();
        let chunk_id = punwrap!(self.get_chunk_from_xy(entity_position.x as usize, entity_position.y as usize), Invalid, "there is no chunk at the position of the entity?");
        let chunk = &mut self.chunks.borrow_mut()[chunk_id];
        let index = chunk.entities_ids.iter().position(|&x| x == entity_id).unwrap();
        chunk.entities_ids.remove(index);
        self.entity_attack_components.remove(&entity_id);
        self.entity_position_components.remove(&entity_id);
        self.entity_pathfinding_components.remove(&entity_id);
        self.entity_health_components.remove(&entity_id);
        self.entity_archetype_lookup.remove(&entity_id);
        self.entity_aggro_components.remove(&entity_id);
        Ok(())
    }
    pub fn kill_entity(&self, entity_id: usize){
        self.entities_to_be_killed_at_end_of_frame.borrow_mut().push(entity_id);
    }
    pub fn kill_entities_to_be_killed(&mut self) -> Result<(), PError>{
        let entities = self.entities_to_be_killed_at_end_of_frame.borrow().clone();
        for entity in entities{
            if let Some(tags) = self.get_entity_tags(entity) {
                if let Some(entity_position) = self.entity_position_components.get(&entity) {
                    let entity_position = entity_position.borrow();
                    for tag in tags.iter(){
                        if let EntityTags::Drops(ref tables) = tag {
                            for table in tables.iter() {
                                let table = punwrap!(self.loot_table_lookup.get(*table), "entity with id {} has a loot table with id {} which doesn't exist", entity, table);
                                let items = table.roll();
                                for item in items.iter() {
                                    let it = ptry!(self.create_item_with_archetype(item.clone()), "while attempting to drop item {} from entity with id {}", item, entity);
                                    self.items_on_floor.borrow_mut().push(ItemOnFloor{
                                        item: it,
                                        x: entity_position.x,
                                        y: entity_position.y,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            ptry!(self.remove_entity(entity));
        }
        self.entities_to_be_killed_at_end_of_frame.borrow_mut().clear();
        Ok(())
    }
    pub fn get_attack_descriptor(&self, attack: &EntityAttackBox) -> Option<&EntityAttackDescriptor>{
        self.entity_attack_descriptor_lookup.get(&attack.archetype)
    }
    pub fn get_attack_descriptor_by_name(&self, archetype_name: &String) -> Option<&EntityAttackDescriptor>{
        self.entity_attack_descriptor_lookup.get(archetype_name)
    }
    pub fn on_key_down(&mut self, key: &str){
        if key.chars().all(char::is_numeric) {
            let num = key.parse::<usize>().unwrap();
            if num < 6 && num > 0 {
                self.inventory.set_hotbar_slot(num - 1);
            }
        }
    }
    pub fn on_mouse_click(&mut self, mouse_position: MousePosition, mouse_left: bool, mouse_right: bool, camera_width: f32, camera_height: f32) -> Result<(), PError>{
        if mouse_left{
            let stats = ptry!(self.inventory.get_combined_stats());
            let pitem = self.inventory.get_cur_held_item();
            let mut attacked = false;
            if let Some(item) = pitem {
                if item.time_til_usable <= 0.0 && (item.item_type == ItemType::MeleeWeapon || item.item_type == ItemType::MagicWeapon){
                    let mouse_direction_unnormalized = [(mouse_position.x_world - self.player.borrow().x - 16.0), (mouse_position.y_world - self.player.borrow().y - 22.0)];
                    let magnitude = f32::sqrt(mouse_direction_unnormalized[0].powf(2.0) + mouse_direction_unnormalized[1].powf(2.0));
                    let mouse_direction_normalized = [
                        mouse_direction_unnormalized[0] / magnitude,
                        mouse_direction_unnormalized[1] / magnitude
                    ];
                    let shots = stats.shots.unwrap_or(1.0).floor() as usize;
                    if shots > 1 && (item.item_type == ItemType::RangedWeapon || item.item_type == ItemType::MagicWeapon) {
                        let mut spread = f32::min(PI/8.0, PI/shots as f32);
                        spread /= stats.focus.unwrap_or(1.0);
                        let angle = mouse_direction_normalized[1].atan2(mouse_direction_normalized[0]) - (shots as f32 - 1.0) * spread/2.0;
                        for i in 0..shots {
                            let ang_adjusted = angle + spread * i as f32;
                            ptry!(self.add_player_attack(
                                    &stats,
                                    item, 
                                    self.player.borrow().x + 16.0 + ang_adjusted.cos() * 25.0,
                                    self.player.borrow().y + 22.0 + ang_adjusted.sin() * 25.0,
                                    ang_adjusted * 180.0/PI));
                        }
                        attacked = true;
                    } else {
                        let angle = mouse_direction_normalized[1].atan2(mouse_direction_normalized[0]);
                        ptry!(self.add_player_attack(
                                &stats, 
                                item,
                                self.player.borrow().x + 16.0 + angle.cos() * 25.0,
                                self.player.borrow().y + 22.0 + angle.sin() * 25.0,
                                angle * 180.0/PI));
                        attacked = true;
                    }
                }
            }
            if attacked {
                println!("attacked");
                let item = punwrap!(self.inventory.get_cur_held_item_mut(), Expected, "attacked with no item?");
                item.time_til_usable = stats.cooldown.unwrap_or(0.0);
            }
                
        }
        Ok(())
    }
    pub fn process_mouse_input(&mut self, mouse_position: MousePosition, mouse_left: bool, mouse_right: bool) -> Result<(), PError>{

        if mouse_left{
            let stats = ptry!(self.inventory.get_combined_stats());
            let pitem = self.inventory.get_cur_held_item();
            let mut attacked = false;
            if let Some(item) = pitem {
                if item.time_til_usable <= 0.0 && item.item_type == ItemType::RangedWeapon {
                    let mouse_direction_unnormalized = [(mouse_position.x_world - self.player.borrow().x - 16.0), (mouse_position.y_world - self.player.borrow().y - 22.0)];
                    let magnitude = f32::sqrt(mouse_direction_unnormalized[0].powf(2.0) + mouse_direction_unnormalized[1].powf(2.0));
                    let mouse_direction_normalized = [
                        mouse_direction_unnormalized[0] / magnitude,
                        mouse_direction_unnormalized[1] / magnitude
                    ];
                    let shots = stats.shots.unwrap_or(1.0).floor() as usize;
                    if shots > 1 && (item.item_type == ItemType::RangedWeapon || item.item_type == ItemType::MagicWeapon) {
                        let mut spread = f32::min(PI/8.0, PI/shots as f32);
                        spread /= stats.focus.unwrap_or(1.0);
                        let angle = mouse_direction_normalized[1].atan2(mouse_direction_normalized[0]) - (shots as f32 - 1.0) * spread/2.0;
                        for i in 0..shots {
                            let ang_adjusted = angle + spread * i as f32;
                            ptry!(self.add_player_attack(
                                    &stats,
                                    item, 
                                    self.player.borrow().x + 16.0 + ang_adjusted.cos() * 25.0,
                                    self.player.borrow().y + 22.0 + ang_adjusted.sin() * 25.0,
                                    ang_adjusted * 180.0/PI));
                        }
                        attacked = true;
                    } else {
                        let angle = mouse_direction_normalized[1].atan2(mouse_direction_normalized[0]);
                        ptry!(self.add_player_attack(
                                &stats, 
                                item,
                                self.player.borrow().x + 16.0 + angle.cos() * 25.0,
                                self.player.borrow().y + 22.0 + angle.sin() * 25.0,
                                angle * 180.0/PI));
                        attacked = true;
                    }
                }
            }
            if attacked {
                let item = punwrap!(self.inventory.get_cur_held_item_mut(), Expected, "attacked with no item?");
                item.time_til_usable = stats.cooldown.unwrap_or(0.0);
            }
                
        }
        Ok(())
    }
    pub fn process_input(&mut self, keys: &FxHashMap<String,bool>, camera: &mut Camera) -> Result<(), PError>{
        ptry!(self.process_player_input(keys));
        let player = self.player.borrow();
        camera.update_camera_position(player.x, player.y);
        drop(player);
        let held_potentially = &self.inventory.get_cur_held_item();
        if held_potentially.is_some() {
            let sprite = &held_potentially.unwrap().sprite;
            self.player.borrow_mut().holding_texture_sprite = self.sprites.get_sprite_id(sprite);
        }else{
            self.player.borrow_mut().holding_texture_sprite = None; 
        }
        Ok(())
        
    }
    pub fn create_item_with_archetype(&self, archetype: String) -> Result<Item, PError> {
        let archetype_i = punwrap!(self.get_item_archetype(&archetype), NotFound, "could not find item archetype {}", archetype);        
        Ok(Item {
            name: archetype_i.name.clone(),
            attack_sprite: archetype_i.attack_sprite.clone(),
            item_type: archetype_i.item_type.clone(),
            width_to_length_ratio: archetype_i.width_to_length_ratio,
            lore: archetype_i.lore.clone(),
            sprite: archetype_i.sprite.clone(),
            stats: archetype_i.stats.get_variation(),
            time_til_usable: archetype_i.stats.cooldown.map(|x| x.get_variation()).unwrap_or(0.0)
        })
    }
    pub fn get_item_archetype(&self, archetype: &String) -> Option<&ItemArchetype>{
        self.item_archetype_lookup.get(archetype)
    }
    pub fn update_damage_text(&self, camera: &mut Camera) -> Result<(), PError> {
        let mut dt_to_remove = Vec::new();
        for (i, damage_text) in self.damage_text.borrow_mut().iter_mut().enumerate(){
            let text_mut_ref = punwrap!(camera.get_world_text_mut(damage_text.world_text_id),Invalid, "damage text descriptor with index {} and value {:?} refers to non-existent world text with id {}", i, damage_text, damage_text.world_text_id);
            text_mut_ref.y -= 0.6;
            text_mut_ref.color[3] -= 0.016_666_668;
            damage_text.lifespan += 1.0;
            if damage_text.lifespan > 60.0 {
                ptry!(camera.remove_world_text(damage_text.world_text_id), NotFound, "Trying to remove non-existent world text with id {} refered to be damage text descriptor with index {} and value {:?}", damage_text.world_text_id, i, damage_text);
                dt_to_remove.push(i);
            }
        }
        for (offset, dt )in dt_to_remove.iter().enumerate(){
            self.damage_text.borrow_mut().remove(*dt - offset);
        }
        Ok(())
    }
    pub fn process_inventory_close(&mut self) -> Result<(), PError> {
        let iwt = self.inventory.items_waiting_to_be_dropped.clone();
        for item in iwt.iter(){
            let i = punwrap!(self.inventory.get_item(item), "Item with id {} waiting to drop does not exist?", item);
            let rand_direction = [rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5];
            let rand_direction_normalized = [rand_direction[0] / f32::sqrt(rand_direction[0].powf(2.0) + rand_direction[1].powf(2.0)), rand_direction[1] / f32::sqrt(rand_direction[0].powf(2.0) + rand_direction[1].powf(2.0))];
            self.items_on_floor.borrow_mut().push(
                ItemOnFloor {
                    x: self.player.borrow().x + (rand_direction_normalized[0]) * 50.0,
                    y: self.player.borrow().y + (rand_direction_normalized[1]) * 50.0,
                    item: i.clone()
                }
            );
            ptry!(self.inventory.remove_item(*item), "while closing inventory");
        }
        self.inventory.items_waiting_to_be_dropped.clear();
        Ok(())
    }
    pub fn update_items_on_ground(&mut self) -> Result<(), PError> {
        let mut items_on_ground = self.items_on_floor.borrow_mut();
        let player = self.player.borrow();
        let px = player.x + 16.0;
        let py = player.y + 22.0;
        let mut to_be_removed = Vec::new();
        for (i, item) in items_on_ground.iter_mut().enumerate(){
            let dir_to_player = [px - item.x, py - item.y];
            let dist_from_player = f32::sqrt(dir_to_player[0].powf(2.0) + dir_to_player[1].powf(2.0));
            let dir_to_player_normalized = [dir_to_player[0] / dist_from_player, dir_to_player[1] / dist_from_player];
            if dist_from_player <= 120.0 && dist_from_player > 15.0 {
                let speed = 2.6/120.0 * (120.0 - dist_from_player) + 0.2;
                item.x += dir_to_player_normalized[0] * speed;
                item.y += dir_to_player_normalized[1] * speed;
            }
            else if dist_from_player <= 15.0 {
                let e = error_prolif_allow!(
                    self.inventory.add_to_slot(item.item.clone()),
                    NoSpace);
                if e.is_err(){
                    continue;
                }
                to_be_removed.push(i);
            }
        }
        for (offset, item) in to_be_removed.iter().enumerate(){
            items_on_ground.remove(*item - offset);
        }
        Ok(())
    }
    pub fn update_items_in_inventory_cd(&mut self) -> Result<(), PError> {
        ptry!(self.inventory.update_items_cd());
        Ok(())
    }
}
