
use winit::{
    event::*, event_loop::EventLoop, keyboard::{Key, KeyCode, PhysicalKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::WindowBuilder
};

use crate::state::State;
use crate::world::World;
use crate::camera::Camera;
use winit::event::WindowEvent::KeyboardInput;

use super::abstractions::SpriteIDContainer;

pub async fn run(world: &mut World, camera: &mut Camera, sprites_json_to_load: Vec<String>,sprites: SpriteIDContainer,  level_editor: bool){
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("RustTest").with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let mut state = State::new(&window, sprites_json_to_load.clone()).await;
    if level_editor{
        state.set_level_editor();
    }

    let mut focused: bool = false;

    event_loop.run(move |event, control_flow| match event {
        
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == state.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    let event = event.clone();
                    state.input(event);
                },
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                    if (level_editor){
                        state.level_editor_highlight_square(world,&camera,  position.x, position.y, sprites.get_sprite("highlight"));
                    }
                }
                WindowEvent::Focused(bool) => {
                    focused = bool;
                    if focused {
                        state.window().request_redraw();
                    }
                },
                WindowEvent::RedrawRequested => {
                    if focused{
                        state.window().request_redraw();
                    }
                    state.update(world, camera);
                    match state.render(world, camera) {
                        Ok(_) => {}
                        Err(
                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        ) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("OutOfMemory");
                            control_flow.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout")
                        }
                    }
                    
                }
                _ => {}
            }   
        }
        _ => {}
    }).unwrap();
}

