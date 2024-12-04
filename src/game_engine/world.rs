use std::collections::HashMap;
use std::hash::Hash;
use crate::vertex::Vertex;
use crate::entities::EntityTags;
use winit::keyboard::Key;
use std::cell::RefCell;

use super::entities::EntityAttackPattern;

#[derive(Debug)]
pub struct Chunk{  // 32x32 blocks of 32x32 = chunks are 1024x1024 pixels but 1024 * RETINA SCALE accounting for retina, so a chunk with x =0, y =0, is pixels 0-1023, 0-1023
    pub chunk_id: usize,
    x: usize,
    y: usize,
    pub terrain_ids: Vec<usize>,
    pub entities_ids: Vec<usize>,
    
}

impl Chunk{
}

pub struct World{
    pub chunks: Vec<Chunk>,
    pub player: RefCell<Player>,
    element_id: usize,
    pub sprites: Vec<Sprite>,
    pub sprite_lookup: HashMap<usize,usize>, // corresponds element_ids to sprite_ids ie. to get the sprite for element_id x, just do sprite_lookup[x]
    pub chunk_lookup: HashMap<[usize; 2],usize>, // corresponds chunk x,y to id
    pub terrain_lookup: HashMap<usize,usize>, // corresponds element_ids of terrain to chunk_ids
    pub terrain: HashMap<usize, Terrain>, // corresponds element id to Terrain element
    pub entities: RefCell<HashMap<usize, Entity>>, // corresponds element id to Entity element
    pub entity_lookup: HashMap<usize,usize>, // corresponds element_ids of entities to chunk_ids
    pub entity_tags_lookup: HashMap<usize,Vec<EntityTags>>, // corresponds element_ids of entities to the entity's tags
    pub loaded_chunks: Vec<usize>, // chunk ids that are currently loaded
}

impl World{ 
    pub fn new() -> Self{
        let mut chunks = Vec::new();
        let mut player = RefCell::new(Player::new());
        let mut element_id = 0;
        let mut sprites = Vec::new();
        let mut sprite_lookup = HashMap::new();
        let mut chunk_lookup = HashMap::new();
        let mut terrain_lookup = HashMap::new();
        let mut entity_lookup = HashMap::new();
        let mut entity_tags_lookup = HashMap::new();
        let mut terrain = HashMap::new();
        let mut entities = RefCell::new(HashMap::new());
        let mut loaded_chunks = Vec::new(); 
        Self{
            chunks: chunks,
            player: player,
            element_id: element_id,
            sprites: sprites,
            sprite_lookup: sprite_lookup,
            chunk_lookup: chunk_lookup,
            terrain_lookup: terrain_lookup,
            entity_lookup: entity_lookup,
            entity_tags_lookup: entity_tags_lookup,
            terrain: terrain,
            entities: entities,
            loaded_chunks: loaded_chunks
        }
    }
    
    pub fn new_chunk(&mut self, chunk_x: usize, chunk_y: usize) -> usize{
        let new_chunk_id = self.chunks.len() as usize;
        self.chunks.push(
            Chunk{
                chunk_id: new_chunk_id,
                x: chunk_x,
                y: chunk_y,
                terrain_ids: Vec::new(),
                entities_ids: Vec::new(),
            });
        self.chunk_lookup.insert([chunk_x, chunk_y], new_chunk_id);
        new_chunk_id
    }
    pub fn get_entity(&self, element_id: usize) -> Option<Entity>{
        let k = &element_id;
        let borrow = self.entities.borrow();
        borrow.get(k).cloned()
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
        let chunk_x = World::coord_to_chunk_coord(x);
        let chunk_y = World::coord_to_chunk_coord(y);
        self.chunk_lookup.get(&[chunk_x, chunk_y]).copied()
    }
    
    pub fn get_chunk_from_id(&self, chunk_id: usize) -> Option<&Chunk>{
        if chunk_id >= self.chunks.len(){
            return None
        }else{
            return Some(&self.chunks[chunk_id]);
        }
    }
    pub fn get_entity_tags(&self, element_id: usize) -> Option<&Vec<EntityTags>>{
        self.entity_tags_lookup.get(&element_id)
    }
    pub fn add_terrain(&mut self, x: usize, y: usize) -> usize{
        let new_terrain = Terrain{ element_id: self.element_id, x: x, y: y };
        let chunk_id_potentially = self.get_chunk_from_xy(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y));
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y));
        }else{
            chunk_id = chunk_id_potentially.unwrap();
        }

        self.element_id += 1;
        self.chunks[chunk_id].terrain_ids.push(self.element_id - 1);
        self.terrain.insert(self.element_id - 1, new_terrain);
        self.terrain_lookup.insert(self.element_id - 1, chunk_id);
        self.element_id - 1
    }

    pub fn add_entity(&mut self, x: f32, y: f32) -> usize{
        let new_entity: Entity = Entity::new(self.element_id,x,y);
        let chunk_id_potentially = self.get_chunk_from_xy(World::coord_to_chunk_coord(new_entity.x.floor() as usize), World::coord_to_chunk_coord(new_entity.y.floor() as usize));
        let chunk_id: usize;
        if chunk_id_potentially.is_none() {
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_entity.x.floor() as usize), World::coord_to_chunk_coord(new_entity.y.floor() as usize));
        } else{
            chunk_id = chunk_id_potentially.unwrap();
        }
        self.element_id += 1;
        self.chunks[chunk_id].entities_ids.push(self.element_id - 1);
        self.entities.borrow_mut().insert(self.element_id - 1, new_entity);
        self.entity_lookup.insert(self.element_id - 1, chunk_id);
        // self.entity_tags_lookup.insert(self.element_id - 1, tags);
        self.element_id - 1
    }

    pub fn add_tag(&mut self, element_id: usize, tag: EntityTags){
        let mut tags = self.entity_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone();
        tags.push(tag);
        self.entity_tags_lookup.insert(element_id, tags);
    }

    pub fn lookup_terrain_chunk(&self, element_id: usize) -> Option<usize>{
        self.terrain_lookup.get(&element_id).copied()
    }

    pub fn add_sprite(&mut self, texture_index: i32) -> usize{
        self.sprites.push(Sprite{ texture_index: texture_index });
        self.sprites.len() - 1
    }

    pub fn set_sprite(&mut self, element_id: usize, sprite_id: usize){
        self.sprite_lookup.insert(element_id, sprite_id);
    }

    pub fn get_sprite(&self, element_id: usize) -> Option<usize>{
        self.sprite_lookup.get(&element_id).copied()
    }
    pub fn process_input(&mut self, keys: HashMap<String,bool>){
        let mut direction: [f32; 2] = [0.0,0.0];
        let mut player = self.player.borrow_mut();
        if *keys.get("w").unwrap_or(&false) || *keys.get("ArrowUp").unwrap_or(&false){
            direction[1] -= 2.0;
        }
        if *keys.get("a").unwrap_or(&false) || *keys.get("ArrowLeft").unwrap_or(&false){
            direction[0] -= 2.0;
        }
        if *keys.get("s").unwrap_or(&false) || *keys.get("ArrowDown").unwrap_or(&false){
            direction[1] += 2.0;
        }
        if *keys.get("d").unwrap_or(&false) || *keys.get("ArrowRight").unwrap_or(&false){
            direction[0] += 2.0;
        }

        let magnitude = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        
        if magnitude > 0.0{
            player.y += direction[1] / magnitude * player.movement_speed;
            player.x += direction[0] / magnitude * player.movement_speed;
        }

        if player.y < 3.0 {
            player.y = 3.0;
        }
        if player.x < 3.0 {
            player.x = 3.0;
        }
    }
    pub fn update_entities(&self) {
        let player = self.player.borrow().clone();
        for chunk in self.loaded_chunks.iter() {
            let chunkref = self.get_chunk_from_id(*chunk).unwrap();
            for entity_id in chunkref.entities_ids.iter() {
                self.update_entity(entity_id, player.x, player.y);
            }
        }
    }
    
    pub fn update_entity(&self, entity_id: &usize, player_x: f32, player_y: f32) {
        let entity_tags = self.get_entity_tags(*entity_id).unwrap();
        let mut entity_mut_hash = self.entities.borrow_mut();
        let mut entity = entity_mut_hash.get_mut(entity_id).unwrap();
        let mut distance: f64 = f64::MAX;
        let mut follows_player = false;
        let mut aggroed_to_player = false;
        let mut aggro_range = 0;
        let mut attack_range = 0;
        let mut movement_speed = 1.0;
        let mut aggressive = false;
        let mut attacks= None; 
        let mut can_attack_player = false;
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
            let attack_pattern = attacks.unwrap();
            if entity.cur_attack_cooldown <= 0.0 {
                self.player.borrow_mut().health -= attack_pattern.attacks[entity.cur_attack].attack();
                entity.cur_attack += 1;
                if entity.cur_attack >= attack_pattern.attacks.len(){
                    entity.cur_attack = 0;
                }
                entity.cur_attack_cooldown = attack_pattern.attack_cooldowns[entity.cur_attack];                
                println!("Attacking player {:?}", self.player.borrow().health);
            }else{
                entity.cur_attack_cooldown -= 1.0/60.0;
            }

            
        }
        if aggroed_to_player {
            let direction = [player_x - entity.x, player_y - entity.y];
            if((direction[0].abs() + direction[1].abs()) > 0.0){
                let magnitude = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
                entity.x += direction[0] / magnitude * movement_speed as f32;
                entity.y += direction[1] / magnitude * movement_speed as f32;
            }
        }
    }
    
    
    
}
#[derive(Copy, Clone, Debug)]
pub struct Terrain{ // terrain is always 32x32 pixels
    pub element_id: usize,
    pub x: usize,
    pub y: usize
}

#[derive(Copy, Clone, Debug)]
pub struct Entity{
    pub element_id: usize,
    pub x: f32,
    pub y: f32,
    pub aggroed_to_player: bool,
    pub cur_attack: usize,
    pub cur_attack_cooldown: f32,
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

#[derive(Copy, Clone, Debug)]
pub struct Sprite{
    pub texture_index: i32,
}

impl Sprite{
    pub fn draw_data(&self, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y = 2.0 / window_size_height as f32;
        
        let w = (screen_w as f32) * screen_to_render_ratio_x;
        let h = (screen_h as f32) * screen_to_render_ratio_y;

        let x = (screen_x + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y = -1.0 * ((screen_y + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


        let vertex = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_index },
        ];

        let index = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}

#[derive(Debug)]
pub struct RenderData{
    pub vertex: Vec<Vertex>,
    pub index: Vec<u16>
}

impl RenderData{
    pub fn new() -> Self{
        Self{ vertex: Vec::new(), index: Vec::new() }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub texture_index: i32,
    pub health: i32,
    pub max_health: i32,
    pub movement_speed: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 576.0,
            y: 360.0,
            health: 100,
            max_health: 100,
            texture_index: 3,
            movement_speed: 3.0,
        }
    }
    pub fn draw_data(&self, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y = 2.0 / window_size_height as f32;
        
        let w = 32 as f32 * screen_to_render_ratio_x;
        let h = 32 as f32 * screen_to_render_ratio_y;

        let x = ((self.x as f32) + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y = -1.0 * (((self.y as f32) + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


        let vertex = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_index },
        ];

        let index = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}
