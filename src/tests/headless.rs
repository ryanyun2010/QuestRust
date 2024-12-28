use core::time;

use winit::event::Event;
use winit::event_loop;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::game_engine::world;
use crate::game_engine::camera;
use crate::game_engine::json_parsing;
use crate::rendering_engine::abstractions;
use crate::rendering_engine::state;
use winit::event::WindowEvent;

pub struct HeadlessGame {
    pub world: world::World,
    pub camera: camera::Camera,
    pub state: super::headless_state::HeadlessState,
}

impl HeadlessGame{
    pub fn new(world: world::World, camera: camera::Camera) -> Self{
        let mut State = super::headless_state::HeadlessState::new();
        Self{
            world,
            camera,
            state: State,
        }
    }
    pub async fn run(&mut self, frames: usize){
        let time_tracker = std::time::Instant::now();
        for i in 0..frames{
            self.camera.render(&mut self.world);
            self.state.update(&mut self.world, &mut self.camera);    
        }  
    }
}