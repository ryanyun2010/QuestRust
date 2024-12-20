use crate::vertex::Vertex;
use crate::rendering_engine::abstractions::Sprite;
use crate::rendering_engine::abstractions::RenderData;

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
    pub holding_texture_sprite: Option<Sprite>
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 596.0,
            y: 400.0,
            health: 100,
            max_health: 100,
            texture_index: 3,
            movement_speed: 2.8284,
            hunger: 100,
            max_hunger: 100,
            holding_texture_sprite: Some(Sprite {texture_index: 12})
        }
    }
    pub fn draw_data(&self, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;
        
        let w: f32 = 32 as f32 * screen_to_render_ratio_x;
        let h: f32 = 32 as f32 * screen_to_render_ratio_y;

        let x: f32 = ((self.x as f32) + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y: f32 = -1.0 * (((self.y as f32) + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


        let mut vertex: Vec<Vertex> = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_index },
        ];

        let mut index: Vec<u16> = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        if self.holding_texture_sprite.is_none(){
            return RenderData { vertex, index }
        }else{
            let sprite = self.holding_texture_sprite.unwrap();
            let d = sprite.draw_data(self.x + 16.0, self.y + 8.0, 24, 24,window_size_width, window_size_height, index_offset + 4, vertex_offset_x, vertex_offset_y);
            index.extend(d.index);
            vertex.extend(d.vertex);
            return RenderData { vertex, index }
        }
    }
}
