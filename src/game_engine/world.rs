use std::collections::HashMap;
use crate::vertex::Vertex;
use winit::dpi::PhysicalSize;
#[derive(Debug)]
struct Chunk{  // 16x16 blocks of 16x16 = chunks are 256x256 pixels, so a chunk with x =0, y =0, is pixels 0-255, 0-255
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
    chunks: Vec<Chunk>,
    player: Player,
    element_id: usize,
    sprites: Vec<Sprite>,
    sprite_lookup: HashMap<usize,usize> // corresponds element_ids to sprite_ids ie. to get the sprite for element_id x, just do sprite_lookup[x]
}

impl World{ // World will render chunks within 8 of the player, ie. a circle of 2048 pixels radius
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
        (coord as f32 / 256.0).floor() as usize
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

    pub fn get_render_data(&self, window_size: PhysicalSize<u32>) -> RenderData{
        let mut render_data = RenderData::new();
        let player_chunk_x = World::coord_to_chunk_coord(self.player.x);
        let player_chunk_y = World::coord_to_chunk_coord(self.player.y);

        for chunk in self.chunks.iter(){
            if (usize::pow(chunk.x,2) + usize::pow(chunk.y,2)) <= 64{
                for terrain in chunk.terrain_ids.iter(){
                    let potentially_sprite_id = self.get_sprite(*terrain);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &self.sprites[sprite_id];
                    
                    render_data.vertex.extend(sprite.draw_data(chunk.terrain[*terrain].x, chunk.terrain[*terrain].y, 64, 64, window_size).vertex);
                    render_data.index.extend(sprite.draw_data(chunk.terrain[*terrain].x, chunk.terrain[*terrain].y, 64, 64, window_size).index);
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
struct Terrain{
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
    fn draw_data(&self, screen_x: usize, screen_y: usize, screen_w: usize, screen_h: usize, window_size: PhysicalSize<u32>) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / window_size.width as f32;
        let screen_to_render_ratio_y = 2.0 / window_size.height as f32;
        
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

        let index = vec![0, 1, 2, 0, 2, 3];

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