use crate::rendering_engine::abstractions::RenderData;

use super::entity_components::CollisionBox;
use super::inventory::Hotbar;
use super::inventory::ItemContainerPointer;
use super::world::World;
#[derive(Clone, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub sprite_id: usize,
    pub health: f32,
    pub max_health: i32,
    pub movement_speed: f32,
    pub holding_texture_sprite: Option<usize>,
    pub inventory: [[ItemContainerPointer; 6]; 6],
    pub collision_box: CollisionBox,
    pub hotbar: Hotbar,
    pub mouse_slot: ItemContainerPointer, //The item your mouse carries
}
#[macro_export]
macro_rules! repeat_token {
    () => {
        
    };
}
impl Player {
    pub fn new(x: f32, y: f32, health: f32, max_health: i32, movement_speed: f32, sprite_id: usize) -> Self {
        Self {
            x: x,
            y: y,
            collision_box: CollisionBox{
                w: 32.0, 
                h: 32.0,
                x_offset: 3.0,
                y_offset: 18.0},
            health: health,
            max_health: max_health,
            sprite_id: sprite_id,
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
        let sprite = world.sprites.get_sprite(self.sprite_id).expect("Could not find player sprite?");
        let mut dd = sprite.draw_data(self.x.floor(), self.y.floor(), 38, 52,window_size_width, window_size_height, index_offset, vertex_offset_x, vertex_offset_y);
        

        if self.holding_texture_sprite.is_none(){
            return dd;
        }else{
            let sprite = world.sprites.get_sprite(self.holding_texture_sprite.unwrap() as usize).expect("Could not find player sprite?");
            let d = sprite.draw_data(self.x.floor() + 16.0, self.y.floor() + 8.0, 24, 24,window_size_width, window_size_height, index_offset + dd.vertex.len() as u16, vertex_offset_x, vertex_offset_y);
            dd.index.extend(d.index);
            dd.vertex.extend(d.vertex);
            return dd;
        }
    }
}
