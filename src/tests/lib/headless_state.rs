use std::collections::HashMap;

use crate::error::PError;
use crate::game_engine::world::World;
use crate::camera::Camera;
use crate::ptry;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    start,
    play,
    inventory,
    death,
}
pub struct HeadlessState{
    pub keys_down: HashMap<String, bool>,
    pub left_mouse_button_down: bool,
    pub right_mouse_button_down: bool,
}

impl Default for HeadlessState {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlessState{
    pub fn new() -> Self{
        Self{
            keys_down: HashMap::new(),
            left_mouse_button_down: false,
            right_mouse_button_down: false,
        }
    }
    pub fn update(&self, world: &mut World, camera: &mut Camera) -> Result<(), PError>{
        world.generate_collision_cache_and_damage_cache();
        world.process_input(&self.keys_down, camera);
        ptry!(world.update_entities());
        world.update_entity_attacks();
        world.update_player_attacks(camera);
        world.kill_entities_to_be_killed();
        Ok(())
    }
    
}


