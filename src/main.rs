pub mod window;
pub mod state;
pub mod vertex;

fn main() {
    pollster::block_on(window::run());
}