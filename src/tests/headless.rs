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
}

impl HeadlessGame{
    pub fn new(world: world::World, camera: camera::Camera) -> Self{
        Self{
            world,
            camera,
        }
    }
    pub async fn run(&mut self, frames: usize){
        let mut title = "Rust Game";
        let time_tracker = std::time::Instant::now();
        let mut State = super::headless_state::HeadlessState::new();

        for i in 0..frames{
            self.camera.render(&mut self.world);
            State.update(&mut self.world, &mut self.camera);    
        }  
    }
}