pub mod window;
pub mod state;

fn main() {
    pollster::block_on(window::run());
}