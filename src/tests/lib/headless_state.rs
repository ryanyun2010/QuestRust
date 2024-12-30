use std::collections::HashMap;

use crate::game_engine::world::World;
use crate::camera::Camera;

pub struct HeadlessState{
    pub keys_down: HashMap<String, bool>,
    pub left_mouse_button_down: bool,
    pub right_mouse_button_down: bool,
}

impl HeadlessState{
    pub fn new() -> Self{
        Self{
            keys_down: HashMap::new(),
            left_mouse_button_down: false,
            right_mouse_button_down: false,
        }
    }
    pub fn update(&self, world: &mut World, camera: &mut Camera) {
        world.generate_collision_cache();
        world.process_input(self.keys_down.clone(), camera);
        world.update_entities();
    }
}