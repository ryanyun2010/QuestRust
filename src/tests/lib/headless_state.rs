use compact_str::CompactString;
use rustc_hash::FxHashMap;

use crate::error::PError;
use crate::game_engine::game::{InputState, MousePosition};
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
    pub keys_down: FxHashMap<CompactString, bool>,
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
            keys_down: FxHashMap::default(),
            left_mouse_button_down: false,
            right_mouse_button_down: false,
        }
    }
    pub fn update(&self, world: &mut World, camera: &mut Camera) -> Result<(), PError>{
        ptry!(world.generate_collision_cache_and_damage_cache());
        ptry!(world.process_input(&self.keys_down, camera, &InputState {
            keys_down: self.keys_down.clone(),
            mouse_position: MousePosition::default(),
            mouse_left: false,
            mouse_right: false
            
        }));
        ptry!(world.update_entities());
        ptry!(world.update_entity_attacks(camera));
        ptry!(world.update_player_abilities(&InputState {
            keys_down: self.keys_down.clone(),
            mouse_position: MousePosition::default(),
            mouse_left: false,
            mouse_right: false
            
        }));
        // TODO: MAKE THIS BETTER
        world.update_player_attacks(camera);
        ptry!(world.update_items_in_inventory_cd());
        ptry!(world.kill_entities_to_be_killed());
        ptry!(world.update_items_on_ground());
        Ok(())
    }
    
}


