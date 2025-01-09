use crate::vertex::Vertex;
use crate::rendering_engine::abstractions::RenderData;

use super::inventory::Hotbar;
use super::inventory::ItemContainer;
use super::inventory::ItemContainerPointer;
use super::world::World;
#[derive(Clone, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub texture_index: i32,
    pub health: f32,
    pub max_health: i32,
    pub movement_speed: f32,
    pub holding_texture_sprite: Option<usize>,
    pub inventory: [[ItemContainerPointer; 6]; 6],
    pub hotbar: Hotbar,
    pub mouse_slot: ItemContainerPointer, //The item your mouse carries
}
#[macro_export]
macro_rules! repeat_token {
    () => {
        
    };
}
impl Player {
    pub fn new(x: f32, y: f32, health: f32, max_health: i32, movement_speed: f32, texture_index: i32) -> Self {
        Self {
            x: x,
            y: y,
            health: health,
            max_health: max_health,
            texture_index: texture_index,
            movement_speed: movement_speed,
            holding_texture_sprite: None,
            inventory: [
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)],
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)],
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)],
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)],
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)],
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)]
            ],
            hotbar: Hotbar::Normal(
                [ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None), ItemContainerPointer::new(None)]
            ),
            mouse_slot: ItemContainerPointer::new(None)
        }
    }
    pub fn draw_data(&self, world: &World, window_size_width: usize, window_size_height: usize, index_offset:u16, vertex_offset_x: i32, vertex_offset_y: i32) -> RenderData{
        let screen_to_render_ratio_x: f32 = 2.0 / window_size_width as f32;
        let screen_to_render_ratio_y: f32 = 2.0 / window_size_height as f32;
        
        let w: f32 = 32 as f32 * screen_to_render_ratio_x;
        let h: f32 = 32 as f32 * screen_to_render_ratio_y;

        let x: f32 = ((self.x.floor() as f32) + (vertex_offset_x as f32)) * screen_to_render_ratio_x - 1.0;
        let y: f32 = -1.0 * (((self.y.floor() as f32) + (vertex_offset_y as f32)) * screen_to_render_ratio_y - 1.0) - h;


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
            let sprite = world.sprites[self.holding_texture_sprite.unwrap() as usize];
            let d = sprite.draw_data(self.x.floor() + 16.0, self.y.floor() + 8.0, 24, 24,window_size_width, window_size_height, index_offset + 4, vertex_offset_x, vertex_offset_y);
            index.extend(d.index);
            vertex.extend(d.vertex);
            return RenderData { vertex, index }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerEffect{
    pub archetype: String,
    pub time_alive: f32,
    pub x: f32,
    pub y: f32,
    pub direction: [f32; 2]
}
impl PlayerEffect{
    pub fn new(archetype: String, time_alive: f32, x: f32, y: f32, direction: [f32; 2]) -> Self{
        Self{
            archetype,
            time_alive,
            x,
            y,
            direction
        }
    }
}