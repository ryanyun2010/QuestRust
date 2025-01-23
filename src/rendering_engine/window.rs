
use winit::{
    event::*, event_loop::EventLoop, window::WindowBuilder
};
use crate::error::PE;
use crate::game_engine::game::Game;
use crate::print_error;
use crate::renderer::Renderer;
use crate::world::World;
use crate::camera::Camera;

pub async fn run(world: World, camera: Camera, sprites_json_to_load: &[String]) {
    let event_loop = EventLoop::new().unwrap();
    let title = "小丰";
    let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let renderer = Renderer::new(&window, sprites_json_to_load).await;
    let mut game = Game::new(world, camera, renderer);
    let mut focused: bool = false;

    let mut rolling_average = Vec::new();
    event_loop.run(move |event, control_flow| match event {
        
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == game.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    match game.key_input(event) {
                        Ok(_) => {}
                        Err(e) => {
                            print_error!(e);
                            control_flow.exit();
                        }
                    }
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
                    let time = std::time::Instant::now();
                    match game.update() {
                        Ok(_) => {}
                        Err(e) => {
                            print_error!(e);
                            control_flow.exit();
                        }
                    }
                    rolling_average.push(time.elapsed().as_nanos());
                    if rolling_average.len() > 100 {
                        rolling_average.remove(0);
                    }
                    let sum: u128 = rolling_average.iter().sum();
                    let avg = sum / rolling_average.len() as u128;
                    println!("Average frame time: {} ms", avg as f64/1_000_000.0);

                    match game.render() {
                        Ok(_) => {}
                        Err(e) => {
                            match e.error {
                                PE::SurfaceError(se) => {
                                    match se {
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                                            print_error!("Surface lost or outdated");
                                            game.resize(game.renderer.size)
                                        },
                                        wgpu::SurfaceError::OutOfMemory => {
                                            print_error!("Out of memory");
                                            control_flow.exit();
                                        }
                                        wgpu::SurfaceError::Timeout => {
                                            print_error!("Surface timeout");
                                            log::warn!("Surface timeout")
                                        }
                                        _ => {
                                            print_error!(format!("Surface error: {}", se));
                                            control_flow.exit();
                                        }
                                    }
                                }
                                _ => {
                                    print_error!(e);
                                    control_flow.exit();
                                }
                            }
                        }
                    }
                }
                _ => {}
            }   
        }
        _ => {}
    }).unwrap();
}

