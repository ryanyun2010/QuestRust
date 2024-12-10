use std::collections::HashMap;
use std::hash::Hash;
use crate::vertex::Vertex;
use crate::entities::EntityTags;
use winit::keyboard::Key;
use std::cell::RefCell;
use crate::game_engine::terrain::Terrain;
use crate::entities::Entity;
use super::entities::EntityAttackPattern;
use super::terrain::{self, TerrainTags};

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
}
// OKAY RYAN WE NEED MAJOR REFORMS.
// OVER TIME, LET'S MOVE THESE INTO MULTIPLE IMPL STATEMENTS IN THEIR RESPECTIVE MODULES.
impl World{ 
    pub fn new() -> Self{
        let mut chunks: RefCell<Vec<Chunk>> = RefCell::new(Vec::new());
        let mut player: RefCell<Player> = RefCell::new(Player::new());
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
            loaded_chunks
        }
    }
    
    pub fn new_chunk(&self, chunk_x: usize, chunk_y: usize) -> usize{
        let new_chunk_id: usize = self.chunks.borrow().len() as usize;
        
        self.chunks.borrow_mut().push(
            Chunk{
                chunk_id: new_chunk_id,
                x: chunk_x,
                y: chunk_y,
                terrain_ids: Vec::new(),
                entities_ids: Vec::new(),
            });
        // println!("d:: {:?}", new_chunk_id);
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
            chunk_id = self.new_chunk(World::coord_to_chunk_coord(new_terrain.x), World::coord_to_chunk_coord(new_terrain.y));
        }else{
            chunk_id = chunk_id_potentially.unwrap();
        }

        self.element_id += 1;
        self.chunks.borrow_mut()[chunk_id].terrain_ids.push(self.element_id - 1);
        self.terrain.insert(self.element_id - 1, new_terrain);
        self.terrain_lookup.insert(self.element_id - 1, chunk_id);
        self.element_id - 1
    }

    pub fn add_tag(&mut self, element_id: usize, tag: EntityTags){
        let mut tags: Vec<EntityTags> = self.entity_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone();
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
            player.y += (direction[1] / magnitude * player.movement_speed).round();
            player.x += (direction[0] / magnitude * player.movement_speed).round();
        }

        if player.y < 3.0 {
            player.y = 3.0;
        }
        if player.x < 3.0 {
            player.x = 3.0;
        }
    }

    pub fn add_tags(&mut self, element_id: usize, tags: Vec<EntityTags>){ //Change this to allow an enum of a vector of tags of various types.
        let mut d = self.entity_tags_lookup.get(&element_id).unwrap_or(&Vec::new()).clone(); 
        d.extend(tags);
        self.entity_tags_lookup.insert(element_id, d);
    }

    
}

#[derive(Copy, Clone, Debug)]
pub struct Sprite{
    pub texture_index: i32,
}

impl Sprite{
    pub fn draw_data(&self, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;
        
        let w: f32 = (screen_w as f32) * screen_to_render_ratio_x;
        let h: f32 = (screen_h as f32) * screen_to_render_ratio_y;

        let x: f32 = (screen_x + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y: f32 = -1.0 * ((screen_y + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


        let vertex: Vec<Vertex> = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_index },
        ];

        let index: Vec<u16> = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

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
    pub hunger: usize,
    pub max_hunger: usize,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 576.0,
            y: 360.0,
            health: 100,
            max_health: 100,
            texture_index: 3,
            movement_speed: 2.8284,
            hunger: 100,
            max_hunger: 100,
        }
    }
    pub fn draw_data(&self, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;
        
        let w: f32 = 32 as f32 * screen_to_render_ratio_x;
        let h: f32 = 32 as f32 * screen_to_render_ratio_y;

        let x: f32 = ((self.x as f32) + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y: f32 = -1.0 * (((self.y as f32) + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


        let vertex: Vec<Vertex> = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_index },
        ];

        let index: Vec<u16> = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}
