use std::collections::HashMap;
use crate::vertex::Vertex;
use crate::entities::EntityTags;
use winit::keyboard::Key;

#[derive(Debug)]
pub struct Chunk{  // 32x32 blocks of 32x32 = chunks are 1024x1024 pixels but 1024 * RETINA SCALE accounting for retina, so a chunk with x =0, y =0, is pixels 0-1023, 0-1023
    pub chunk_id: usize,
    x: usize,
    y: usize,
    pub terrain_ids: Vec<usize>,
    pub entities_ids: Vec<usize>,
    pub terrain: Vec<Terrain>,
    pub entities: Vec<Entity>
}

impl Chunk{
}

pub struct World{
    pub chunks: Vec<Chunk>,
    pub player: Player,
    element_id: usize,
    pub sprites: Vec<Sprite>,
    pub sprite_lookup: HashMap<usize,usize>, // corresponds element_ids to sprite_ids ie. to get the sprite for element_id x, just do sprite_lookup[x]
    pub chunk_lookup: HashMap<[usize; 2],usize>, // corresponds chunk x,y to id
    /* TODO: MAKE THIS ACTUALLY WORK */ pub terrain_lookup: HashMap<usize,usize>, // corresponds element_ids of terrain to chunk_ids
    /* TODO: MAKE THIS ACTUALLY WORK */ pub entity_lookup: HashMap<usize,usize>, // corresponds element_ids of entities to chunk_ids
    /* TODO: MAKE THIS ACTUALLY WORK */ pub entity_tags_lookup: HashMap<usize,EntityTags>, // corresponds element_ids of entities to the entity's tags
}

impl World{ 
    pub fn new() -> Self{
        let mut chunks = Vec::new();
        let mut player = Player::new();
        let mut element_id = 0;
        let mut sprites = Vec::new();
        let mut sprite_lookup = HashMap::new();
        let mut chunk_lookup = HashMap::new();
        let mut terrain_lookup = HashMap::new();
        let mut entity_lookup = HashMap::new();
        let mut entity_tags_lookup = HashMap::new();
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
        }
    }
    
    pub fn new_chunk(&mut self, chunk_x: usize, chunk_y: usize) -> usize{
        let new_chunk_id = self.chunks.len() as usize;
        self.chunks.push(
            Chunk{
                chunk_id: new_chunk_id,
                x: chunk_x,
                y: chunk_y,
                terrain: Vec::new(),
                entities: Vec::new(),
                terrain_ids: Vec::new(),
                entities_ids: Vec::new(),
            });
        self.chunk_lookup.insert([chunk_x, chunk_y], new_chunk_id);
        new_chunk_id
    }
    pub fn coord_to_chunk_coord(coord: usize) -> usize{
        (coord as f32 / 1024.0).floor() as usize
    }
    pub fn get_chunk_from_xy(&self, x: usize, y: usize) -> Option<usize>{
        let chunk_x = World::coord_to_chunk_coord(x);
        let chunk_y = World::coord_to_chunk_coord(y);
        self.chunk_lookup.get(&[chunk_x, chunk_y]).copied()
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
        self.chunks[chunk_id].terrain.push(new_terrain);
        self.element_id - 1
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
        if *keys.get("w").unwrap_or(&false){
            self.player.y -= 2;
        }
        if *keys.get("a").unwrap_or(&false){
            self.player.x -= 2;
        }
        if *keys.get("s").unwrap_or(&false){
            self.player.y += 2;
        }
        if *keys.get("d").unwrap_or(&false){
            self.player.x += 2;
        }
        if self.player.y < 3 {
            self.player.y = 3;
        }
        if self.player.x < 3 {
            self.player.x = 3;
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
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Sprite{
    pub texture_index: i32,
}

impl Sprite{
    pub fn draw_data(&self, screen_x: usize, screen_y: usize, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y = 2.0 / window_size_height as f32;
        
        let w = screen_w as f32 * screen_to_render_ratio_x;
        let h = screen_h as f32 * screen_to_render_ratio_y;

        let x = ((screen_x as f32) + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y = -1.0 * (((screen_y as f32) + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


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

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub texture_index: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 576,
            y: 360,
            texture_index: 3,
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
