pub mod rendering_engine;
use rendering_engine::window;
use rendering_engine::state;
use rendering_engine::vertex;
use rendering_engine::texture;
pub mod game_engine;
use game_engine::world;

fn main() {
    let mut world = world::World::new();
    let new_sprite = world.add_sprite(0);
    let new_sprite2 = world.add_sprite(0);
    let new_terrain = world.add_terrain(0,0);
    let new_terrain2 = world.add_terrain(32,0);
    world.set_sprite(new_terrain,new_sprite);
    world.set_sprite(new_terrain2,new_sprite2);
    println!("{:?}",world.chunks[0]);
    pollster::block_on(window::run(&world));
}