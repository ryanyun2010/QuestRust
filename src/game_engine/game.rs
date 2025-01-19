use std::collections::HashMap;

use winit::{event, keyboard::{Key, NamedKey}};

use crate::{error::PError, error_prolif, ptry, rendering_engine::{abstractions::RenderDataFull, renderer::Renderer, vertex::Vertex}};

use super::{camera::Camera, world::World};
#[derive(Debug, Copy, Clone)]
pub struct MousePosition{
    pub x_world: f32,
    pub y_world: f32,
    pub x_screen: f32,
    pub y_screen: f32,
}

impl MousePosition{
    pub fn default() -> Self{
        Self {
            x_world: 0.0,
            y_world: 0.0,
            x_screen: 0.0,
            y_screen: 0.0,
        }
    }
}
pub struct InputState {
    pub keys_down: HashMap<String, bool>,
    pub mouse_position: MousePosition,
    pub mouse_left: bool,
    pub mouse_right: bool,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    start,
    play,
    death,
}
pub struct Game<'a> {
    pub world: World,
    pub camera: Camera,
    pub renderer: Renderer<'a>,
    pub input: InputState,
    pub state: GameState,
}

impl<'a> Game<'a> {
    pub fn new(world: World, camera: Camera, renderer: Renderer<'a>) -> Game<'a> {
        let cx = camera.camera_x;
        let cy = camera.camera_y;
        Self {
            world: world,
            camera: camera,
            renderer: renderer,
            state: GameState::start,
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
    pub fn process_mouse_move(&mut self, x: f64, y: f64){
        if self.state == GameState::play {
            self.input.mouse_position.x_screen = x as f32 / self.renderer.size.width as f32 * self.camera.viewpoint_width as f32;
            self.input.mouse_position.y_screen = y as f32 / self.renderer.size.height as f32 * self.camera.viewpoint_height as f32;
        }else{
            self.input.mouse_position.x_screen = x as f32 / self.renderer.size.width as f32 * self.camera.viewpoint_width as f32;
            self.input.mouse_position.y_screen = y as f32 / self.renderer.size.height as f32 * self.camera.viewpoint_height as f32;
            self.input.mouse_position.x_world = self.camera.camera_x + self.input.mouse_position.x_screen;
            self.input.mouse_position.y_world = self.camera.camera_y + self.input.mouse_position.y_screen;
        }
    }
    pub fn process_mouse_click(&mut self, state: event::ElementState, button: event::MouseButton){
        if self.state == GameState::start {
            self.state = GameState::play;
            return;
        }
        match button {
            event::MouseButton::Left => {
                self.input.mouse_left = state == event::ElementState::Pressed;
            },
            event::MouseButton::Right => {
                self.input.mouse_right = state == event::ElementState::Pressed;
            },
            _ => {}
        }
        if state == event::ElementState::Pressed {
            self.world.on_mouse_click(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right, self.camera.viewpoint_width as f32, self.camera.viewpoint_height as f32);
        }

    }
    pub fn process_input(&mut self){
        if self.state == GameState::play {
            self.world.process_input(&self.input.keys_down, &mut self.camera);
            self.world.process_mouse_input(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right);
        }
    }
    pub fn render(&mut self) -> Result<(), PError> {
        if self.state == GameState::start {
            self.renderer.render(
                self.world.sprites.get_sprite_by_name("start_screen").expect("No start_screen sprite?").draw_data(0.0, 0.0, self.camera.viewpoint_width, self.camera.viewpoint_height, self.camera.viewpoint_width, self.camera.viewpoint_height, 0, 0, 0).to_full()
            );
            return Ok(());
        }
        let data = ptry!(self.camera.render(&mut self.world, self.renderer.config.width as f32, self.renderer.config.height as f32));

        match self.renderer.render(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                error_prolif!(PError::new(
                    crate::error::PE::SurfaceError(e),
                    vec![]
                ))
            }
        }
    }
    pub fn update(&mut self) -> Result<(), PError>{
        self.camera.update_ui(&mut self.world);
        self.world.generate_collision_cache_and_damage_cache();
        self.process_input();
        self.world.update_entities();
        self.world.update_entity_attacks();
        self.world.update_player_attacks(&mut self.camera);
        match self.world.update_damage_text(&mut self.camera) {
            Ok(_) => {},
            Err(e) => {
                error_prolif!(e);
            }
        }
        self.world.kill_entities_to_be_killed();
        self.input.mouse_position.x_world = self.camera.camera_x + self.input.mouse_position.x_screen;
        self.input.mouse_position.y_world = self.camera.camera_y + self.input.mouse_position.y_screen;
        Ok(())
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
        if key.is_none(){
            return;
        }
        let string_key = key.unwrap().to_string().to_lowercase();
        let press = match event.state {
            event::ElementState::Pressed => true,
            event::ElementState::Released => false,
        };

        if press {
            self.on_key_down(&string_key);
        }
        
        self.input.keys_down.insert(string_key, press);
    }

    pub fn on_key_down(&mut self, key: &String){
       self.world.on_key_down(key);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.size = new_size;
            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;
            self.renderer.surface.configure(&self.renderer.device, &self.renderer.config);
            self.renderer.text_brush_a.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
            self.renderer.text_brush_b.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.renderer.window()
    }

    
}
