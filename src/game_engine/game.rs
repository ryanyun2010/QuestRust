use std::collections::HashMap;

use winit::{event, keyboard::{Key, NamedKey}};

use crate::rendering_engine::{abstractions::{RenderDataFull, SpriteIDContainer}, renderer::Renderer};

use super::{camera::Camera, level_editor::MousePosition, world::World};

pub struct InputState {
    pub keys_down: HashMap<String, bool>,
    pub mouse_position: MousePosition,
    pub mouse_left: bool,
    pub mouse_right: bool,
}
pub struct Game<'a> {
    pub world: World,
    pub camera: Camera,
    pub sprites: SpriteIDContainer,
    pub renderer: Renderer<'a>,
    pub input: InputState,
}

impl<'a> Game<'a> {
    pub fn new(world: World, camera: Camera, renderer: Renderer<'a>, sprites: SpriteIDContainer) -> Game<'a> {
        let cx = camera.camera_x;
        let cy = camera.camera_y;
        Self {
            world: world,
            camera: camera,
            renderer: renderer,
            sprites: sprites,
            input: InputState {
                keys_down: HashMap::new(),
                mouse_position: MousePosition { 
                    x_screen: 0.0,
                    y_screen: 0.0,
                    x_world: cx,
                    y_world: cy,
                },
                mouse_left: false,
                mouse_right: false,
            },
        }
    }
    pub fn process_input(&mut self){
        self.world.process_input(self.input.keys_down.clone(), &mut self.camera);
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(self.camera.render(&mut self.world))
    }
    pub fn update(&mut self){
        self.camera.update_ui(&mut self.world);
        self.world.generate_collision_cache();
        self.process_input();
        self.world.update_entities();
    }
    pub fn key_input(&mut self, event: winit::event::KeyEvent) {
        let mut key = event.logical_key.to_text();

        match event.logical_key {
            Key::Named(NamedKey::ArrowLeft) => {
                key = Some("ArrowLeft");
            },
            Key::Named(NamedKey::ArrowRight) => {
                key = Some("ArrowRight");
            },
            Key::Named(NamedKey::ArrowUp) => {
                key = Some("ArrowUp");
            },
            Key::Named(NamedKey::ArrowDown) => {
                key = Some("ArrowDown");
            }
            _ => {}
        }
        let key = event.logical_key.to_text();
        if key.is_none(){
            return;
        }
        let string_key = key.unwrap().to_string().to_lowercase();
        let press = match event.state {
            event::ElementState::Pressed => true,
            event::ElementState::Released => false,
        };

        if press {
            self.on_key_down(string_key.clone());
        }
        
        self.input.keys_down.insert(string_key, press);
    }



    pub fn on_key_down(&mut self, key: String){
       
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.size = new_size;
            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;
            self.renderer.surface.configure(&self.renderer.device, &self.renderer.config);
            self.renderer.text_brush.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.renderer.window()
    }

    
}