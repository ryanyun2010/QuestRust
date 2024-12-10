use super::world::World;

impl World {
    pub fn magicLookup(&self, element_id: usize) {
        match element_id {
            _ => {panic!("Unknown element ID: {:?}.", element_id)}
        }
    }
}