use std::collections::HashMap;
use crate::vertex::Vertex;
use winit::dpi::PhysicalSize;
#[derive(Debug)]
pub struct Chunk{  // 32x32 blocks of 32x32 = chunks are 1024x1024 pixels but 1024 * RETINA SCALE accounting for retina, so a chunk with x =0, y =0, is pixels 0-1023, 0-1023
    chunk_id: usize,
    x: usize,
    y: usize,
    terrain_ids: Vec<usize>,
    entities_ids: Vec<usize>,
    terrain: Vec<Terrain>,
    entities: Vec<Entity>
}

impl Chunk{
}

pub struct World{
    pub chunks: Vec<Chunk>,
    player: Player,
    element_id: usize,
    sprites: Vec<Sprite>,
    pub sprite_lookup: HashMap<usize,usize> // corresponds element_ids to sprite_ids ie. to get the sprite for element_id x, just do sprite_lookup[x]
}

impl World{ // World will render chunks within 4 of the player, ie. a circle of 4096 * RETINA SCALE pixels radius
    pub fn new() -> Self{
        let mut chunks = Vec::new();
        let mut player = Player::new();
        let mut element_id = 0;
        let mut sprites = Vec::new();
        let mut sprite_lookup = HashMap::new();
        Self{
            chunks: chunks,
            player: player,
            element_id: element_id,
            sprites: sprites,
            sprite_lookup: sprite_lookup
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
        new_chunk_id
    }
    fn coord_to_chunk_coord(coord: usize) -> usize{
        (coord as f32 / 1024.0).floor() as usize
    }
    fn get_chunk_from_xy(&self, x: usize, y: usize) -> Option<usize>{
        let chunk_x = World::coord_to_chunk_coord(x);
        let chunk_y = World::coord_to_chunk_coord(y);

        for chunk in self.chunks.iter(){
            if chunk.x == chunk_x && chunk.y == chunk_y{
                return Some(chunk.chunk_id);
            }
        }

        None
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

    pub fn get_render_data(&self, window_size: PhysicalSize<u32>, RETINA_SCALE: f64) -> RenderData{
        let mut render_data = RenderData::new();
        let player_chunk_x = World::coord_to_chunk_coord(self.player.x);
        let player_chunk_y = World::coord_to_chunk_coord(self.player.y);

        for chunk in self.chunks.iter(){
            if (usize::pow(chunk.x,2) + usize::pow(chunk.y,2)) <= 16{
                for terrain in chunk.terrain_ids.iter(){
                    
                    let potentially_sprite_id = self.get_sprite(*terrain);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &self.sprites[sprite_id];

                    let index_offset = render_data.vertex.len() as u16;
                    let draw_data = sprite.draw_data(chunk.terrain[*terrain].x, chunk.terrain[*terrain].y, 32, 32, window_size, RETINA_SCALE, index_offset);
                    
                    render_data.vertex.extend(draw_data.vertex);
                    render_data.index.extend(draw_data.index);
                }
            }
        }
        render_data
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
}
#[derive(Copy, Clone, Debug)]
struct Terrain{ // terrain is always 32x32 pixels
    element_id: usize,
    x: usize,
    y: usize
}

#[derive(Copy, Clone, Debug)]
struct Entity{
    element_id: usize,
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
struct Sprite{
    texture_index: i32,
}

impl Sprite{
    fn draw_data(&self, screen_x: usize, screen_y: usize, screen_w: usize, screen_h: usize, window_size: PhysicalSize<u32>, RETINA_SCALE: f64, index_offset:u16) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / window_size.width as f32 * RETINA_SCALE as f32;
        let screen_to_render_ratio_y = 2.0 / window_size.height as f32 * RETINA_SCALE as f32;
        
        let x = screen_x as f32 * screen_to_render_ratio_x - 1.0;
        let y = screen_y as f32 * screen_to_render_ratio_y - 1.0;

        let w = screen_w as f32 * screen_to_render_ratio_x;
        let h = screen_h as f32 * screen_to_render_ratio_y;

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
    fn new() -> Self{
        Self{ vertex: Vec::new(), index: Vec::new() }
    }
}

struct Player {
    x: usize,
    y: usize,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}



/*
RenderData { 
vertex: [
Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], index: 0 },
Vertex { position: [-0.92, -1.0, 0.0], tex_coords: [1.0, 1.0], index: 0 },
Vertex { position: [-0.92, -0.8933333, 0.0], tex_coords: [1.0, 0.0], index: 0 },
Vertex { position: [-1.0, -0.8933333, 0.0], tex_coords: [0.0, 0.0], index: 0 },
 Vertex { position: [-0.92, -1.0, 0.0], tex_coords: [0.0, 1.0], index: 0 },
 Vertex { position: [-0.84000003, -1.0, 0.0], tex_coords: [1.0, 1.0], index: 0 },
 Vertex { position: [-0.84000003, -0.8933333, 0.0], tex_coords: [1.0, 0.0], index: 0 }, 
 ertex { position: [-0.92, -0.8933333, 0.0], tex_coords: [0.0, 0.0], index: 0 }],
 index: [0, 1, 2, 0, 2, 3, 0, 1, 2, 0, 2, 3] }*/