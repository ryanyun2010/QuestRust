pub mod rendering_engine;
use rendering_engine::window;
use rendering_engine::state;
use rendering_engine::vertex;
use rendering_engine::texture;
pub mod game_engine;
use game_engine::world;

fn main() {
    pollster::block_on(window::run());
}