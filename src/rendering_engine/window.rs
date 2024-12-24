
use winit::{
    event::*, event_loop::EventLoop, keyboard::{Key, KeyCode, PhysicalKey}, platform::modifier_supplement::KeyEventExtModifierSupplement, window::WindowBuilder
};

use crate::{game_engine::json_parsing::JSON_parser, state};
use crate::world::World;
use crate::camera::Camera;
use winit::event::WindowEvent::KeyboardInput;
use super::abstractions::SpriteIDContainer;

pub async fn run(world: &mut World, camera: &mut Camera, sprites_json_to_load: Vec<String>,sprites: SpriteIDContainer,  level_editor: bool, parser: &mut JSON_parser) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("RustTest").with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let mut State = state::State::new(&window, sprites_json_to_load.clone()).await;
    if level_editor{
        State.set_level_editor();
    }

    let mut focused: bool = false;

    event_loop.run(move |event, control_flow| match event {
        
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == State.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    let event = event.clone();
                    State.input(event);
                },
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    State.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                    if (level_editor){
                        State.level_editor_highlight_square(world,   position.x, position.y, sprites.get_sprite("highlight"), &camera);
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    if (level_editor){
                        State.level_editor_process_mouse_input(world, state, button);
                    }
                },
                WindowEvent::Focused(bool) => {
                    focused = bool;
                    if focused {
                        State.window().request_redraw();
                    }
                },
                WindowEvent::RedrawRequested => {
                    if focused{
                        State.window().request_redraw();
                    }
                    State.update(world, camera, parser);
                    match State.render(world, camera) {
                        Ok(_) => {}
                        Err(
                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        ) => State.resize(State.size),
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

