use std::collections::HashMap;
use wgpu_text::glyph_brush::{HorizontalAlign, Layout, Section as TextSection, Text};
use crate::game_engine::{camera::Camera, json_parsing::sprites_json_descriptor};

use super::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct SpriteSheet {
    pub texture_id: i32,
    pub width: usize,
    pub height: usize,
    pub sprite_width: usize,
    pub sprite_height: usize,
    pub sprite_padding: usize
}
impl SpriteSheet{
    pub fn create_sprites(&self, sprite_positions: Vec<[usize;2]>) -> Vec<Sprite>{
        let mut sprites = Vec::new();
        for position in sprite_positions {
            let x = position[0] * (self.sprite_width + self.sprite_padding);
            let y = position[1] * (self.sprite_height + self.sprite_padding);
            sprites.push(Sprite {
                texture_index: self.texture_id,
                tex_x: x as f32 / self.width as f32,
                tex_y: y as f32 / self.height as f32,
                tex_w: self.sprite_width as f32 / self.width as f32,
                tex_h: self.sprite_height as f32 / self.height as f32,
            });
        }
        sprites
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Sprite{
    pub texture_index: i32,
    pub tex_x: f32,
    pub tex_y: f32,
    pub tex_w: f32,
    pub tex_h: f32
}
impl Sprite {
    pub fn draw_data(&self, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData {

        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;
        
        let w: f32 = (screen_w as f32) * screen_to_render_ratio_x;
        let h: f32 = (screen_h as f32) * screen_to_render_ratio_y;

        let x: f32 = (screen_x + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y: f32 = -1.0 * ((screen_y + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;
        let vertex: Vec<Vertex> = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [self.tex_x, self.tex_y + self.tex_h], index: self.texture_index },
            Vertex { position: [x + w, y, 0.0], tex_coords: [self.tex_x + self.tex_w, self.tex_y + self.tex_h], index: self.texture_index },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [self.tex_x + self.tex_w, self.tex_y], index: self.texture_index },
            Vertex { position: [x, y + h, 0.0], tex_coords: [self.tex_x, self.tex_y], index: self.texture_index },
        ];

        let index: Vec<u16> = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}






#[derive(Debug, Clone)]
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
    pub fn rotated_90(&self) -> RenderData{
        if self.vertex.len() != 4 {
            panic!("Rotated only works with 4 vertices");
        }
        let mut clone = self.clone();
        let first = clone.vertex[0].tex_coords;
        clone.vertex[0].tex_coords = clone.vertex[3].tex_coords;
        clone.vertex[3].tex_coords = clone.vertex[2].tex_coords;
        clone.vertex[2].tex_coords = clone.vertex[1].tex_coords;
        clone.vertex[1].tex_coords = first;
        return clone;
    }
    pub fn rotated(&self, angle: f32) -> RenderData{
        if self.vertex.len() != 4 {
            panic!("Rotated only works with 4 vertices");
        }
        let mut clone = self.clone();
        let vert = crate::game_engine::utils::get_rotated_corners(&crate::game_engine::utils::Rectangle {
            x: self.vertex[0].position[0],
            y: self.vertex[0].position[1],
            width: self.vertex[1].position[0] - self.vertex[0].position[0],
            height: self.vertex[3].position[1] - self.vertex[0].position[1],
            rotation: angle 
        });
        clone.vertex[0].position = [vert[0].0, vert[0].1, 0.0];
        clone.vertex[1].position = [vert[1].0, vert[1].1, 0.0];
        clone.vertex[2].position = [vert[2].0, vert[2].1, 0.0];
        clone.vertex[3].position = [vert[3].0, vert[3].1, 0.0];
        return clone;
    }
    pub fn flipped_x(&self) -> RenderData {
        if self.vertex.len() != 4 {
            panic!("Flip only works with 4 vertices");
        }
        let mut clone = self.clone();
        let left = [clone.vertex[0].tex_coords,clone.vertex[3].tex_coords];
        clone.vertex[0].tex_coords = clone.vertex[1].tex_coords;
        clone.vertex[3].tex_coords = clone.vertex[2].tex_coords;
        clone.vertex[1].tex_coords = left[0];
        clone.vertex[2].tex_coords = left[1];
        return clone;
    }
    pub fn flipped_y(&self) -> RenderData {
        if self.vertex.len() != 4 {
            panic!("Flip only works with 4 vertices");
        }
        let mut clone = self.clone();
        let top = [clone.vertex[0].tex_coords,clone.vertex[1].tex_coords];
        clone.vertex[0].tex_coords = clone.vertex[2].tex_coords;
        clone.vertex[1].tex_coords = clone.vertex[3].tex_coords;
        clone.vertex[2].tex_coords = top[0];
        clone.vertex[3].tex_coords = top[1];
        return clone;
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
pub struct SpriteContainer{
    pub sprites: Vec<Sprite>,
    pub sprite_id_lookup: HashMap<String, usize>
}

impl SpriteContainer{
    pub fn new() -> SpriteContainer{
        SpriteContainer{
            sprites: Vec::new(),
            sprite_id_lookup: HashMap::new()
        }
    }
    pub fn create_from_json(descriptor: &sprites_json_descriptor) -> (Vec<String>, SpriteContainer){
        let mut id = 0;
        let mut sprites = Vec::new();
        let mut sprite_id_lookup = HashMap::new();
        let mut sprites_to_load = Vec::new();
        
        for sprite in descriptor.basic_sprites.iter(){
            sprites.push(Sprite {
                texture_index: id,
                tex_x: 0.0,
                tex_y: 0.0,
                tex_w: 1.0,
                tex_h: 1.0
            });
            sprites_to_load.push(sprite.path.clone());
            sprite_id_lookup.insert(sprite.name.clone(), sprites.len() - 1);
            id += 1;
        }
        for sheet in descriptor.spritesheets.iter(){
                
            let spritesheet = SpriteSheet {
                texture_id: id,
                width: sheet.width,
                height: sheet.height,
                sprite_width: sheet.sprite_width,
                sprite_height: sheet.sprite_height,
                sprite_padding: sheet.sprite_padding
            };
            println!("{} {}", id, sheet.path);
            sprites_to_load.push(sheet.path.clone());
            let mut sprite_positions = Vec::new();
            let mut names = Vec::new();
            for sprite in sheet.sprites.iter(){
                sprite_positions.push([sprite.x, sprite.y]);
                names.push(sprite.name.clone());
            }
            let spritesd = spritesheet.create_sprites(sprite_positions);
            for i in 0..spritesd.len(){
                println!("{} {}", i, names[i]);
                sprites.push(spritesd[i]);
                sprite_id_lookup.insert(names[i].to_string(), sprites.len() - 1);
            }

            id += 1;
        }
        return (sprites_to_load, SpriteContainer{
            sprites,
            sprite_id_lookup
        });
    }

    pub fn get_sprite_by_name(&self, name: &str) -> Option<&Sprite>{
        let potential_id = self.sprite_id_lookup.get(name);
        if potential_id.is_none(){
            return None;
        }else{
            return Some(&self.sprites[*potential_id.unwrap()]);
        }
    }
    pub fn get_sprite_id(&self, name: &str) -> Option<usize>{
        self.sprite_id_lookup.get(name).cloned()
    }
    pub fn get_sprite(&self, id: usize) -> Option<&Sprite>{
        self.sprites.get(id)
    }
    pub fn get_texture_index_by_name(&self, name: &str) -> Option<i32>{
        let potential_id = self.sprite_id_lookup.get(name);
        if potential_id.is_none(){
            return None;
        }else{
            return Some(self.sprites[*potential_id.unwrap()].texture_index);
        }
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
