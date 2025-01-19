macro_rules! print_error {
    ($e:expr) => {
        colorize_println(format!("{}", $e), Colors::BrightRedFg);
    };
}

use std::time::Instant;

use winit::{
    event::*, event_loop::EventLoop, window::{WindowBuilder}
};
use crate::game_engine::game::Game;
use crate::renderer::Renderer;
use crate::world::World;
use crate::camera::Camera;
use colorized::*;

pub async fn run(world: World, camera: Camera, sprites_json_to_load: &Vec<String>) {
    let event_loop = EventLoop::new().unwrap();
    let title = "小丰";
    let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let renderer = Renderer::new(&window, sprites_json_to_load).await;
    let mut game = Game::new(world, camera, renderer);
    let mut focused: bool = false;

    event_loop.run(move |event, control_flow| match event {
        
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == game.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    game.key_input(event);
                },
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    game.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                    game.process_mouse_move(position.x, position.y);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    game.process_mouse_click(state, button);
                },
                WindowEvent::Focused(bool) => {
                    focused = bool;
                    if focused {
                        game.window().request_redraw();
                    }
                },
                WindowEvent::RedrawRequested => {
                    if focused{
                        game.window().request_redraw();
                    }
                    let time = Instant::now();
                    match game.update() {
                        Ok(_) => {}
                        Err(e) => {
                            print_error!(e);
                            control_flow.exit();
                        }
                    }
                    // println!("Time: {:?}", time.elapsed());
                    match game.render() {
                        Ok(_) => {}
                        Err(
                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        ) => game.resize(game.renderer.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            print_error!("Out of Memory");
                            control_flow.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            print_error!("Surface Timeout");
                        }
                        Err(e) => {
                            print_error!(e);
                            control_flow.exit();
                        }
                        
                    }
                    
                }
                _ => {}
            }   
        }
        _ => {}
    }).unwrap();
}

