
use crate::{error::PError, game_engine::{camera, world}, ptry, rendering_engine::abstractions::UIEFull};
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
    pub async fn run(&mut self, frames: usize) -> Result<(), PError>{
        for _i in 0..frames{
            ptry!(self.camera.render(&mut self.world, UIEFull {
                sprites: vec![],
                text: vec![],
            }, 1152.0, 720.0));
            ptry!(self.state.update(&mut self.world, &mut self.camera));    
        }  
        Ok(())
    }
}