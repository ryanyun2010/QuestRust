pub mod window;
pub mod state;
pub mod vertex;
pub mod texture;

fn main() {
    pollster::block_on(window::run());
}