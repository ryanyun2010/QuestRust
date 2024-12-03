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
    let mut world = world::World::new(); // 36 x 22.5 blocks
    let mut camera = camera::Camera::new(1152,720);
    
    let outside_sprite = world.add_sprite(6);
    let dirt_sprite = world.add_sprite(5);
    let dirt2_sprite = world.add_sprite(4);
    let wall_sprite = world.add_sprite(7);
    for n in 0..17 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            world.set_sprite(new_terrain,outside_sprite);
        }
    }
    for m in 0..70 {
        let new_terrain = world.add_terrain(544,m*32);
        world.set_sprite(new_terrain,wall_sprite);
    }
    for n in 18..35 {
        for m in 0..70 {
            let new_terrain = world.add_terrain(n*32,m*32);
            let x: u8 = random();
            if x > 150{
            world.set_sprite(new_terrain,dirt_sprite);
            } else{
                world.set_sprite(new_terrain,dirt2_sprite); 
            }
        }
    }
    // println!("{:?}",world.chunks[0]);
    
    pollster::block_on(window::run(&mut world, &mut camera));
}