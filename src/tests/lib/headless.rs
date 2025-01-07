use std::collections::HashMap;

use crate::{game_engine::{camera, world}, rendering_engine::abstractions::SpriteIDContainer};
use super::headless_state::HeadlessState;

pub struct HeadlessGame {
    pub world: world::World,
    pub camera: camera::Camera,
    pub state: HeadlessState,
}

impl HeadlessGame{
    pub fn new(world: world::World, camera: camera::Camera) -> Self{
        let state = HeadlessState::new();
        Self{
            world,
            camera,
            state,
        }
    }
    pub async fn run(&mut self, frames: usize){
        for _i in 0..frames{
            self.camera.render(&mut self.world, &SpriteIDContainer{
                sprites: HashMap::new(),
                texture_ids: HashMap::new(),
            });
            self.state.update(&mut self.world, &mut self.camera);    
        }  
    }
}