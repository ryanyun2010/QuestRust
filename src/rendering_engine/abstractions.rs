use std::collections::HashMap;
use wgpu_text::glyph_brush::{HorizontalAlign, Layout, Section as TextSection, Text};
use crate::game_engine::{camera::Camera, json_parsing::ParsedData, world::World};

use super::vertex::Vertex;


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
    pub fn offset(&mut self, index_offset: u16){
        for index in self.index.iter_mut(){
            *index += index_offset;
        }
    }
}

pub struct RenderDataFull<'a>{
    pub vertex: Vec<Vertex>,
    pub index: Vec<u16>,
    pub sections: Vec<TextSection<'a>>
}

impl RenderDataFull<'_>{
    pub fn new() -> Self{
        Self{ vertex: Vec::new(), index: Vec::new(), sections: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteIDContainer{
    pub sprites: HashMap<String, usize>,
    pub texture_ids: HashMap<String, i32>
}

impl SpriteIDContainer{
    pub fn generate_from_json_parsed_data(parser: &ParsedData, world: &mut World) -> Self{
        let mut sprites = HashMap::new();
        for (name, texture_id) in &parser.texture_ids {
            let sprite = world.add_sprite(texture_id.clone());
            sprites.insert(name.clone(), sprite);
        }
        let texture_ids = parser.texture_ids.clone();parser.texture_ids.clone();
        Self { sprites,  texture_ids}
    }
    pub fn get_sprite(&self, name: &str) -> Option<usize>{
        self.sprites.get(name).cloned()
    }
    pub fn get_texture_id(&self, name: &str) -> Option<i32>{
        self.texture_ids.get(name).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct TextSprite{
    pub text: String,
    pub font_size: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub align: HorizontalAlign
}

impl TextSprite{
    pub fn new(text: String, font_size: f32, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], align: HorizontalAlign) -> Self{
        Self { text, font_size, x, y, w, h, color, align}
    }
    pub fn get_section(&self, camera: &Camera, screen_width: f32, screen_height: f32) -> TextSection<'_>{
        TextSection::default().add_text(
            Text::new(self.text.as_str())
            .with_scale(self.font_size)
            .with_color(self.color)
        ).with_screen_position((self.x/camera.viewpoint_width as f32 * screen_width, self.y/camera.viewpoint_height as f32 * screen_height))
        .with_layout(Layout::default().h_align(self.align))
        .with_bounds((self.w/camera.viewpoint_width as f32 * screen_width,self.h/camera.viewpoint_height as f32 * screen_height))
    }
}
