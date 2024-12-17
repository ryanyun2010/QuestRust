use super::world::World;

impl World {
    pub fn magic_lookup(&self, element_id: usize) {
        match element_id {
            _ => {panic!("Unknown element ID: {:?}.", element_id)}
        }
    }
}