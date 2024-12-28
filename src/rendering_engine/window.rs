
use winit::{
    event::*, event_loop::EventLoop, window::WindowBuilder
};
use crate::state::State;
use crate::world::World;
use crate::camera::Camera;


pub async fn run(world: &mut World, camera: &mut Camera, sprites_json_to_load: Vec<String>) {
    let event_loop = EventLoop::new().unwrap();
    let title = "Rust Game";
    let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let mut state_obj = State::new(&window, sprites_json_to_load.clone()).await;

    let mut focused: bool = false;

    event_loop.run(move |event, control_flow| match event {
        
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == state_obj.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    let event = event.clone();
                    state_obj.input(event);
                },
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    state_obj.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                },
                WindowEvent::MouseInput { state, button, .. } => {
                },
                WindowEvent::Focused(bool) => {
                    focused = bool;
                    if focused {
                        state_obj.window().request_redraw();
                    }
                },
                WindowEvent::RedrawRequested => {
                    if focused{
                        state_obj.window().request_redraw();
                    }
                    state_obj.update(world, camera);
                    match state_obj.render(world, camera) {
                        Ok(_) => {}
                        Err(
                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        ) => state_obj.resize(state_obj.size),
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

