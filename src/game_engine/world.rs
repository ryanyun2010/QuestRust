use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;
use crate::rendering_engine::abstractions::Sprite;
use crate::rendering_engine::abstractions::RenderData;
use crate::vertex::Vertex;
use crate::entities::EntityTags;
use winit::keyboard::Key;
use std::cell::RefCell;
use crate::entities::Entity;
use super::json_parsing::ParsedData;
use super::player::Player;
use super::entities::EntityAttackPattern;
use super::terrain::{self, Terrain, TerrainTags};


pub enum EntityDirectionOptions{
    Up,
    Down,
    Left,
    Right,
    None
}
#[derive(Debug, Clone)]
pub struct Chunk{  // 32x32 blocks of 32x32 = chunks are 1024x1024 pixels but 1024 * RETINA SCALE accounting for retina, so a chunk with x =0, y =0, is pixels 0-1023, 0-1023
    pub chunk_id: usize,
    x: usize,
    y: usize,
    pub terrain_ids: Vec<usize>,
    pub entities_ids: Vec<usize>,
    
}

impl Chunk{
}
// TODO: ENTITY CHUNKING HAS A CRAZY AMOUNT OF BUGS HERE
pub struct World{
    pub chunks: RefCell<Vec<Chunk>>,
    pub player: RefCell<Player>,
    pub element_id: usize,
    pub sprites: Vec<Sprite>,
    pub sprite_lookup: HashMap<usize,usize>, // corresponds element_ids to sprite_ids ie. to get the sprite for element_id x, just do sprite_lookup[x]
    pub chunk_lookup: RefCell<HashMap<[usize; 2],usize>>, // corresponds chunk x,y to id
    pub terrain_lookup: HashMap<usize,usize>, // corresponds element_ids of terrain to chunk_ids
    pub terrain: HashMap<usize, Terrain>, // corresponds element id to Terrain element
    pub entities: RefCell<HashMap<usize, Entity>>, // corresponds element id to Entity element
    pub entity_lookup: RefCell<HashMap<usize,usize>>, // corresponds element_ids of entities to chunk_ids
    pub entity_tags_lookup: HashMap<usize,Vec<EntityTags>>, // corresponds element_ids of entities to the entity's tags
    pub terrain_tags_lookup: HashMap<usize,Vec<TerrainTags>>, // corresponds element_ids of entities to the entity's tags
    pub loaded_chunks: Vec<usize>, // chunk ids that are currently loaded
    pub collision_cache: RefCell<HashMap<[usize; 2], Vec<usize>>>, // collision x,y, to element id, collision tiles are 64x64
}
// OKAY RYAN WE NEED MAJOR REFORMS.
// OVER TIME, LET'S MOVE THESE INTO MULTIPLE IMPL STATEMENTS IN THEIR RESPECTIVE MODULES.
impl World{ 
    pub fn new(player: Player) -> Self{
        let mut chunks: RefCell<Vec<Chunk>> = RefCell::new(Vec::new());
        let mut player: RefCell<Player> = RefCell::new(player);
        let mut element_id: usize = 0;
        let mut sprites: Vec<Sprite> = Vec::new();
        let mut sprite_lookup: HashMap<usize, usize> = HashMap::new();
        let mut chunk_lookup: RefCell<HashMap<[usize; 2], usize>> = RefCell::new(HashMap::new());
        let mut terrain_lookup: HashMap<usize, usize> = HashMap::new();
        let mut entity_lookup: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());
        let mut entity_tags_lookup: HashMap<usize, Vec<EntityTags>> = HashMap::new();
        let mut terrain_tags_lookup: HashMap<usize, Vec<TerrainTags>> = HashMap::new();
        let mut terrain: HashMap<usize, Terrain> = HashMap::new();
        let mut entities: RefCell<HashMap<usize, Entity>> = RefCell::new(HashMap::new());
        let mut loaded_chunks: Vec<usize> = Vec::new(); 
        let mut collision_cache: RefCell<HashMap<[usize; 2], Vec<usize>>> = RefCell::new(HashMap::new());
        Self{
            chunks,
            player,
            element_id,
            sprites,
            sprite_lookup,
            chunk_lookup,
            terrain_lookup,
            entity_lookup,
            entity_tags_lookup,
            terrain_tags_lookup,
            terrain,
            entities,
            loaded_chunks,
            collision_cache
        }
    }
    
    pub fn new_chunk(&self, chunk_x: usize, chunk_y: usize, chunkref: Option<&mut std::cell::RefMut<'_, Vec<Chunk>>>) -> usize{
        let mut new_chunk_id: usize = 0;
        if chunkref.is_none(){
            new_chunk_id = self.chunks.borrow().len() as usize; 
            self.chunks.borrow_mut().push(
                Chunk{
                    chunk_id: new_chunk_id,
                    x: chunk_x,
                    y: chunk_y,
                    terrain_ids: Vec::new(),
                    entities_ids: Vec::new(),
                });
        }else{
            let cr = chunkref.unwrap();
            new_chunk_id = cr.len() as usize; 
            cr.push(
                Chunk{
                    chunk_id: new_chunk_id,
                    x: chunk_x,
                    y: chunk_y,
                    terrain_ids: Vec::new(),
                    entities_ids: Vec::new(),
                });
        }
        self.chunk_lookup.borrow_mut().insert([chunk_x, chunk_y], new_chunk_id);
        new_chunk_id
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
        
        let new_terrain: Terrain = Terrain{ element_id: self.element_id, x: x, y: y };
        
        let chunk_id_potentially: Option<usize> = self.get_chunk_from_chunk_xy(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y));
        
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y), None);
        }else{
            chunk_id = chunk_id_potentially.unwrap();
        }

        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].terrain_ids.push(self.element_id - 1);
        self.terrain.insert(self.element_id - 1, new_terrain);
        self.terrain_lookup.insert(self.element_id - 1, chunk_id);
        self.element_id - 1
    }

    pub fn add_entity_tag(&mut self, element_id: usize, tag: EntityTags){
        let mut tags: Vec<EntityTags> = self.entity_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone();
        tags.push(tag);
        self.entity_tags_lookup.insert(element_id, tags);
    }
    pub fn get_terrain_tiles(x: usize, y: usize, w: usize, h: usize) -> Vec<[usize; 2]>{
        let mut tiles: Vec<[usize; 2]> = Vec::new();
        let left_x = (x as f32 / 32.0).floor() as usize;
        let right_x = ((x as f32 + w as f32) / 32.0).floor() as usize;
        let top_y = (y as f32 / 32.0).floor() as usize;
        let bot_y = ((y as f32 + h as f32)/ 32.0).floor() as usize;
        for x in left_x..(right_x + 1){
            for y in top_y..(bot_y + 1){
                tiles.push([x,y]);
            }
        }
        tiles
    }
    pub fn generate_collision_cache(&mut self){
        let mut collision_cache_ref = self.collision_cache.borrow_mut();
        collision_cache_ref.clear();
        for chunk in self.loaded_chunks.iter(){ 
            let chunk = &self.chunks.borrow()[*chunk];
            for terrain_id in chunk.terrain_ids.iter(){
                let terrain = self.terrain.get(terrain_id).unwrap();
                let terrain_tags_potentially = self.terrain_tags_lookup.get(terrain_id);
                if terrain_tags_potentially.is_none(){
                    continue;
                }
                let terrain_tags = terrain_tags_potentially.unwrap();
                for tag in terrain_tags.iter(){
                    match tag{
                        TerrainTags::BlocksMovement => {
                            let tiles_blocked: Vec<[usize; 2]> = World::get_terrain_tiles(terrain.x, terrain.y, 32, 32);
                            for tile in tiles_blocked.iter(){
                                let mut collision_cache_entry = collision_cache_ref.get(&[tile[0],tile[1]]).unwrap_or(&Vec::new()).clone();
                                collision_cache_entry.push(*terrain_id);
                                collision_cache_ref.insert([tile[0],tile[1]], collision_cache_entry);
                            }
                        }
                        _ => ()
                    }
                }
            }

            for entity_id in chunk.entities_ids.iter(){
                let entity = self.entities.borrow().get(entity_id).unwrap().clone();
                let entity_tags_potentially = self.entity_tags_lookup.get(entity_id);
                if entity_tags_potentially.is_none(){
                    continue;
                }
                let entity_tags = entity_tags_potentially.unwrap();
                for tag in entity_tags.iter(){
                    match tag{
                        EntityTags::HasCollision => {
                            let tiles_blocked: Vec<[usize; 2]> = World::get_terrain_tiles(entity.x as usize, entity.y as usize, 32, 32);
                            for tile in tiles_blocked.iter(){
                                let mut collision_cache_entry = collision_cache_ref.get(&[tile[0],tile[1]]).unwrap_or(&Vec::new()).clone();
                                collision_cache_entry.push(*entity_id);
                                collision_cache_ref.insert([tile[0],tile[1]], collision_cache_entry);
                            }
                        }
                        _ => ()
                    }
                }
            }
        }  
    }

    pub fn check_collision(&self, player: bool, id_to_ignore: Option<usize>, x: usize, y: usize, w: usize, h: usize, entity: bool, entity_hash: Option<HashMap<usize, Entity>>) -> bool{
        
        if !player {
            let p = self.player.borrow();
            if p.x - 1.0 < (x + w) as f32 && p.x + 33.0 > x as f32 && p.y - 1.0 < (y + h) as f32 && p.y + 33.0 > y as f32{
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
        let mut eh;
        if entity {
            eh = entity_hash.unwrap();
        }else{
            eh = HashMap::new();
        }
        let idti: usize = id_to_ignore.unwrap_or(usize::MAX);
        for id in ids_to_check{
            if id == idti{
                continue;
            }
            let terrain_potentially = self.terrain.get(&id);
            
            if terrain_potentially.is_none(){
                if entity{
                    let entity = eh.get(&id).unwrap();
                    if entity.x < (x + w) as f32 && entity.x + 32.0 > x as f32 && entity.y < (y + h) as f32 && entity.y + 32.0 > y as f32{
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

   
    

    pub fn attempt_move_player(&self, player: &mut Player, movement: [f32; 2]){
        
        if self.check_collision(true, None,(player.x + movement[0]).floor() as usize, (player.y + movement[1]).floor() as usize, 32, 32, true, Some(self.entities.borrow().clone())){
            return;
        }
        player.x += movement[0];
        player.y += movement[1];
    }

    pub fn can_move_player(&self, player: &mut Player, movement: [f32; 2]) -> bool{
        if self.check_collision(true, None,(player.x + movement[0]).floor() as usize, (player.y + movement[1]).floor() as usize, 32, 32, true, Some(self.entities.borrow().clone())){
            return false;
        }
        true
    }

    pub fn add_terrain_tag(&mut self, element_id: usize, tag: TerrainTags){
        let mut tags: Vec<TerrainTags> = self.terrain_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone();
        tags.push(tag);
        self.terrain_tags_lookup.insert(element_id, tags);
    }

    pub fn lookup_terrain_chunk(&self, element_id: usize) -> Option<usize>{
        self.terrain_lookup.get(&element_id).copied()
    }

    pub fn add_sprite(&mut self, texture_index: i32) -> usize{
        self.sprites.push(Sprite{ texture_index: texture_index });
        self.sprites.len() - 1
    }

    pub fn create_entity_from_json_archetype(&mut self, x: f32, y: f32, archetype: &str, parser: &ParsedData) -> usize{
        let archetype = parser.get_archetype(archetype).expect(&format!("Archetype {} not found", archetype));
        let entity = self.add_entity(x, y);
        self.add_entity_tags(entity, archetype.clone());
        entity
    }

    pub fn set_sprite(&mut self, element_id: usize, sprite_id: usize){
        self.sprite_lookup.insert(element_id, sprite_id);
    }

    pub fn get_sprite(&self, element_id: usize) -> Option<usize>{
        self.sprite_lookup.get(&element_id).copied()
    }

    pub fn process_input(&mut self, keys: HashMap<String,bool>){
        let mut direction: [f32; 2] = [0.0,0.0];
        let mut player: std::cell::RefMut<'_, Player> = self.player.borrow_mut();
        if *keys.get("w").unwrap_or(&false) || *keys.get("ArrowUp").unwrap_or(&false){
            direction[1] -= 1.0;
        }
        if *keys.get("a").unwrap_or(&false) || *keys.get("ArrowLeft").unwrap_or(&false){
            direction[0] -= 1.0;
        }
        if *keys.get("s").unwrap_or(&false) || *keys.get("ArrowDown").unwrap_or(&false){
            direction[1] += 1.0;
        }
        if *keys.get("d").unwrap_or(&false) || *keys.get("ArrowRight").unwrap_or(&false){
            direction[0] += 1.0;
        }

        let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        
        if magnitude > 0.0{
            let movement = [(direction[0] / magnitude * player.movement_speed).round(), (direction[1] / magnitude * player.movement_speed).round()];
            let player_movement_speed = player.movement_speed.clone();
            
            if !self.can_move_player(&mut player, [movement[0], 0.0]){
                self.attempt_move_player(&mut player, [0.0, (direction[1] * player_movement_speed).round()]);
            }else if !self.can_move_player(&mut player, [0.0, movement[1]]){
                self.attempt_move_player(&mut player, [(direction[0] * player_movement_speed).round(), 0.0]);
            }else{
                self.attempt_move_player(&mut player, movement);
            }
        }

        if player.y < 3.0 {
            player.y = 3.0;
        }
        if player.x < 3.0 {
            player.x = 3.0;
        }
    }

    pub fn add_entity_tags(&mut self, element_id: usize, tags: Vec<EntityTags>){ //Change this to allow an enum of a vector of tags of various types.
        let mut d = self.entity_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone(); 
        d.extend(tags);
        self.entity_tags_lookup.insert(element_id, d);
    }
    pub fn add_terrain_tags(&mut self, element_id: usize, tags: Vec<TerrainTags>){ //Change this to allow an enum of a vector of tags of various types.
        let mut d: Vec<TerrainTags> = self.terrain_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone(); 
        d.extend(tags);
        self.terrain_tags_lookup.insert(element_id, d);
    }
    
}

