use std::{collections::HashMap, path::Path};
use image::io::Reader;
use wgpu_text::glyph_brush::{HorizontalAlign, Layout, Section as TextSection, Text};
use crate::game_engine::{camera::Camera, json_parsing::{sprite_sheet_json, sprite_sheet_sprite_json, sprites_json_descriptor}, ui::UIESprite, utils::{get_rotated_corners, Rectangle}};

use super::{sprite_sheet_generation_abstraction::SpriteSheetSheet, vertex::Vertex};

#[derive(Debug, Clone, Copy)]
pub struct SpriteSheet {
    pub texture_id: i32,
    pub x_offset: usize,
    pub total_width: usize,
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
            let x = position[0] * (self.sprite_width + self.sprite_padding) + self.x_offset;
            let y = position[1] * (self.sprite_height + self.sprite_padding);
            sprites.push(Sprite {
                texture_index: self.texture_id,
                tex_x: (x as f64 / self.total_width as f64) as f32,
                tex_y: (y as f64 / self.height as f64) as f32,
                tex_w: (self.sprite_width as f64 / self.total_width as f64) as f32,
                tex_h: (self.sprite_height as f64 / self.height as f64) as f32,
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
    pub fn draw_data(&self, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset: u32, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData {

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

        let index: Vec<u32> = vec![index_offset, 1 + index_offset, 2 + index_offset, index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
    pub fn draw_data_rotated(&self, rotation:f32, screen_x: f32, screen_y: f32, screen_w: usize, screen_h: usize, window_size_width: usize, window_size_height: usize, index_offset:u32, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData {
            let v = get_rotated_corners(
                &Rectangle{
                    x: screen_x,
                    y: screen_y,
                    width: screen_w as f32,
                    height: screen_h as f32,
                    rotation
                }
            );
            let v_array: [(f32, f32); 4] = [
                (v[0].0 + (vertex_offset_x as f32), v[0].1 + (vertex_offset_y as f32)),
                (v[1].0 + (vertex_offset_x as f32), v[1].1 + (vertex_offset_y as f32)),
                (v[2].0 + (vertex_offset_x as f32), v[2].1 + (vertex_offset_y as f32)),
                (v[3].0 + (vertex_offset_x as f32), v[3].1 + (vertex_offset_y as f32))
            ];
            self.draw_data_p(v_array, window_size_width, window_size_height, index_offset)
        }
    pub fn draw_data_p(&self, points: [(f32, f32); 4], window_size_width: usize, window_size_height: usize, index_offset: u32) -> RenderData {
        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;

        let mut vertex = Vec::new();
        
        let mut ps = points;
        ps.reverse();
        let tex = [[self.tex_x, self.tex_y + self.tex_h], [self.tex_x + self.tex_w, self.tex_y + self.tex_h], [self.tex_x + self.tex_w, self.tex_y], [self.tex_x, self.tex_y]];

        for i in 0..4 {
            let (screen_x, screen_y) = ps[i];
            let x: f32 = screen_x * screen_to_render_ratio_x - 1.0;
            let y: f32 = -1.0 * (screen_y * screen_to_render_ratio_y - 1.0);
            vertex.push(Vertex {
                position: [x, y, 0.0],
                tex_coords: tex[i],
                index: self.texture_index,
            });
        }

        let index: Vec<u32> = vec![index_offset, 1 + index_offset, 2 + index_offset, index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}






#[derive(Debug, Clone)]
pub struct RenderData{
    pub vertex: Vec<Vertex>,
    pub index: Vec<u32>
}

impl Default for RenderData {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderData{
    pub fn new() -> Self{
        Self{ vertex: Vec::new(), index: Vec::new() }
    }
    pub fn offset(&mut self, index_offset: u32){
        for index in self.index.iter_mut(){
            *index += index_offset;
        }
    }
    pub fn to_full(&self) -> RenderDataFull{
        RenderDataFull{
            vertex: self.vertex.clone(),
            index: self.index.clone(),
            sections_a_b: Vec::new(),
            sections_b_b: Vec::new(),
            sections_a_t: Vec::new(),
            sections_b_t: Vec::new(),
            index_behind_text: 0,
        }
    }
}

pub struct RenderDataFull<'a>{
    pub vertex: Vec<Vertex>,
    pub index: Vec<u32>,
    pub sections_a_b: Vec<TextSection<'a>>,
    pub sections_b_b: Vec<TextSection<'a>>,
    pub sections_a_t: Vec<TextSection<'a>>,
    pub sections_b_t: Vec<TextSection<'a>>,
    pub index_behind_text: u32,

}

impl Default for RenderDataFull<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderDataFull<'_>{
    pub fn new() -> Self{
        Self{ vertex: Vec::new(), index: Vec::new(), sections_a_b: Vec::new(), sections_b_b: Vec::new(), sections_a_t: Vec::new(), sections_b_t: Vec::new(), index_behind_text: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteContainer{
    pub sprites: Vec<Sprite>,
    pub sprite_id_lookup: HashMap<String, usize>
}

impl Default for SpriteContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteContainer{
    pub fn new() -> SpriteContainer{
        SpriteContainer{
            sprites: Vec::new(),
            sprite_id_lookup: HashMap::new()
        }
    }
    pub fn create_from_json(descriptor: &sprites_json_descriptor) -> (Vec<String>, SpriteContainer){
        let mut sprites = Vec::new();
        let mut sprite_id_lookup = HashMap::new();
        let mut sprites_to_load = Vec::new();
        let mut sprite_sheets = descriptor.spritesheets.clone();
        for sprite in descriptor.basic_sprites.iter(){ // TODO: THIS IS VERY JANK CODE THAT PROBABLY SHOULDN"T BE DONE LIKE THIS BUT I DONT CARE TOO MUCH ABOUT OPTIMIZING THIS
            let size = get_image_dimensions(&sprite.path).expect(format!("Couldn't get image dimensions of image {}", &sprite.path).as_str());
            sprite_sheets.push(sprite_sheet_json{
                name: sprite.name.clone(),
                path: sprite.path.clone(),
                width: size.0 as usize,
                height: size.1 as usize,
                sprite_width: size.0 as usize,
                sprite_height: size.1 as usize,
                sprite_padding: 0,
                sprites: vec![sprite_sheet_sprite_json{
                    name: sprite.name.clone(),
                    x: 0,
                    y: 0
                }]
            }
                
                );
        }
        let sss = SpriteSheetSheet::create_from_json(&sprite_sheets, 0);
        sprites_to_load.push(sss.path.clone());
        for (i, sheet) in sprite_sheets.iter().enumerate(){
            sprites_to_load.push(sheet.path.clone());
            let mut sprite_positions = Vec::new();
            let mut names = Vec::new();
            for sprite in sheet.sprites.iter(){
                sprite_positions.push([sprite.x, sprite.y]);
                names.push(sprite.name.clone());
            }
            let spritesd = sss.sheets[i].create_sprites(sprite_positions);
            for i in 0..spritesd.len(){
                sprites.push(spritesd[i]);
                sprite_id_lookup.insert(names[i].to_string(), sprites.len() - 1);
            }
        }
        (sprites_to_load, SpriteContainer{
            sprites,
            sprite_id_lookup
        })
    }

    pub fn get_sprite_by_name(&self, name: &str) -> Option<&Sprite>{
        let id = self.sprite_id_lookup.get(name)?;
        self.sprites.get(*id)
    }
    pub fn get_sprite_id(&self, name: &str) -> Option<usize>{
        self.sprite_id_lookup.get(name).copied()
    }
    pub fn get_sprite(&self, id: usize) -> Option<&Sprite>{
        self.sprites.get(id)
    }
    pub fn get_texture_index_by_name(&self, name: &str) -> Option<i32>{
        let id = self.sprite_id_lookup.get(name)?;
        self.sprites.get(*id).map(|s| s.texture_index)
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
    pub fn get_section(&self, camera: &Camera, screen_width: f32, screen_height: f32, x_offset: f32, y_offset: f32) -> TextSection<'_>{
        TextSection::default().add_text(
            Text::new(self.text.as_str())
            .with_scale(self.font_size)
            .with_color(self.color)
        ).with_screen_position(((self.x + x_offset)/camera.viewpoint_width as f32 * screen_width, (self.y + y_offset)/camera.viewpoint_height as f32 * screen_height))
        .with_layout(Layout::default().h_align(self.align))
        .with_bounds((self.w/camera.viewpoint_width as f32 * screen_width,self.h/camera.viewpoint_height as f32 * screen_height))
    }
}

fn get_image_dimensions(file_path: &str) -> Result<(u32, u32), image::error::ImageError> {
    let path = Path::new(file_path);
    let reader = Reader::open(path)?;
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
}




#[derive(Debug, Clone)]
pub struct UIEFull {
    pub sprites: Vec<UIESprite>,
    pub text: Vec<TextSprite>,
}