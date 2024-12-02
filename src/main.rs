use rand::prelude::*;
pub mod rendering_engine;
use rendering_engine::window;
use rendering_engine::state;
use rendering_engine::vertex;
use rendering_engine::texture;
pub mod game_engine;
use game_engine::*;
// use game_engine::world;
// use game_engine::camera;
// use game_engine::loot;
// use game_engine::entities;

fn main() {
    let mut world = world::World::new();
    let mut camera = camera::Camera::new(2000,1400);
    
    let grass_sprite = world.add_sprite(0);
    let panda_sprite = world.add_sprite(1);
    for n in 0..20 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,grass_sprite);
        }
    }
    for n in 21..35 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,panda_sprite);
        }
    }
    println!("{:?}",world.chunks[0]);
    
    pollster::block_on(window::run(&mut world, &mut camera));
}