use crate::rendering_engine::abstractions::RenderData;
use crate::error::PError;
use crate::punwrap;

use super::entity_components::{CollisionBox, Fire, Poison};
use super::world::World;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PlayerDir {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownRight,
    DownLeft,
    Left,
    Right
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerState {
    Idle,
    Walking,
    AttackingRanged,
    AttackingMelee,
    ChargingAbility,
    EndingAbility
}

pub const EXP_REQS: [f32; 3] = [
    100.0, // exp required to level from 0 to 1
    150.0, // exp required to level from 1 to 2
    200.0, // ...
];
pub const MAX_LEVEL: usize = 2;

#[derive(Clone, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub sprite_id: usize,
    pub health: f32,
    pub max_health: i32,
    pub movement_speed: f32,
    pub holding_texture_sprite: Option<usize>,
    pub collision_box: CollisionBox,
    pub direction: PlayerDir,
    pub player_state: PlayerState,
    pub exp: f32,
    pub level: usize,
    pub fire: Option<Fire>,
    pub poison: Vec<Poison>,
    pub anim_frame: usize // 0-119
}
impl Player {
    pub fn new(x: f32, y: f32, health: f32, max_health: i32, movement_speed: f32, sprite_id: usize) -> Self {
        Self {
            x,
            y,
            collision_box: CollisionBox{
                w: 32.0, 
                h: 32.0,
                x_offset: 3.0,
                y_offset: 18.0},
            health,
            max_health,
            sprite_id,
            movement_speed,
            holding_texture_sprite: None,
            direction: PlayerDir::Down,
            player_state: PlayerState::Idle,
            exp: 0.0,
            level: 0,
            fire: Some(
                    Fire {
                        damage: 5.0,
                        lifetime: 205.0,
                        time_per_tick: 50.0,
                        time_alive: 0.0
                    }
                ),
            poison: vec![
                Poison {
                    damage: 1.0,
                    time_alive: 0.0,
                    time_per_tick: 1.0,
                    lifetime: 5.0
                }
            ],
            anim_frame: 0
        }
    }
    pub fn get_held_item_position(&self) -> (f32, f32) {
        match self.direction {
            PlayerDir::Up => {
                (self.x.floor() + 21.0, self.y.floor() + 15.0)
            },
            PlayerDir::Down => {
                (self.x.floor() + 16.0, self.y.floor() + 28.0)
            },
            PlayerDir::Right | PlayerDir::DownRight | PlayerDir::UpRight => {
                (self.x.floor() + 28.0, self.y.floor() + 21.0)
            },
            PlayerDir::Left | PlayerDir::DownLeft | PlayerDir::UpLeft => {
                (self.x.floor() - 13.0, self.y.floor() + 21.0)
            }
        }
    }

    pub fn add_exp(&mut self, exp: f32) {
        self.exp += exp; 
        while self.level < MAX_LEVEL && self.exp > EXP_REQS[self.level] {
            self.exp -= EXP_REQS[self.level];
            self.level += 1;
        }
    }



    pub fn draw_data(&self, world: &World, window_size_width: usize, window_size_height: usize, index_offset:u32, vertex_offset_x: i32, vertex_offset_y: i32) -> Result<RenderData, PError>{
        let mut d = RenderData::new();
        let held_item_pos = self.get_held_item_position();
        if self.direction == PlayerDir::Up {
            if let Some(holding_sprite) = self.holding_texture_sprite{
                let sprite = punwrap!(world.sprites.get_sprite(holding_sprite), Expected, "held item sprite doesnt exist");
                let s = sprite.draw_data(held_item_pos.0, held_item_pos.1, 24, 24,window_size_width, window_size_height, index_offset, vertex_offset_x, vertex_offset_y);
                d.vertex.extend(s.vertex);
                d.index.extend(s.index);
            }
            let sprite = punwrap!(world.sprites.get_sprite(self.sprite_id), Expected, "player sprite doesn't exist");
            let dd = sprite.draw_data(self.x.floor(), self.y.floor(), 38, 52,window_size_width, window_size_height, index_offset + d.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
            d.vertex.extend(dd.vertex);
            d.index.extend(dd.index);
        }else {
            let sprite = punwrap!(world.sprites.get_sprite(self.sprite_id), Expected, "player sprite doesn't exist");
            let dd = sprite.draw_data(self.x.floor(), self.y.floor(), 38, 52,window_size_width, window_size_height, index_offset + d.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
            d.vertex.extend(dd.vertex);
            d.index.extend(dd.index);
            if let Some(holding_sprite) = self.holding_texture_sprite{
                let sprite = punwrap!(world.sprites.get_sprite(holding_sprite), Expected, "held item sprite doesnt exist");
                let s = sprite.draw_data(held_item_pos.0, held_item_pos.1, 24, 24,window_size_width, window_size_height, index_offset + d.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
                d.vertex.extend(s.vertex);
                d.index.extend(s.index);
            }
        }
        if self.fire.is_some() {
            let sprite = if self.anim_frame < 60 {
               punwrap!(world.sprites.get_sprite_by_name("fire1"), MissingExpectedGlobalSprite, "no fire1 sprite?") 
            }else {
               punwrap!(world.sprites.get_sprite_by_name("fire2"), MissingExpectedGlobalSprite, "no fire2 sprite?")
            };
            let dd = sprite.draw_data(self.x.floor(), self.y.floor(), 38, 52,window_size_width, window_size_height, index_offset + d.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
            d.vertex.extend(dd.vertex);
            d.index.extend(dd.index);
        }


        Ok(d)
    }
}
