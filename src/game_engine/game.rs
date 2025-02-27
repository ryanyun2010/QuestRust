use compact_str::{CompactString, ToCompactString};
use rustc_hash::FxHashMap;

use winit::{event, keyboard::{Key, NamedKey}};

use crate::{error::PError, ptry, punwrap, rendering_engine::renderer::Renderer};

use super::{camera::Camera, world::World};
#[derive(Debug, Copy, Clone)]
pub struct MousePosition{
    pub x_world: f32,
    pub y_world: f32,
    pub x_screen: f32,
    pub y_screen: f32,
}

impl Default for MousePosition{
    fn default() -> Self{
        Self {
            x_world: 0.0,
            y_world: 0.0,
            x_screen: 0.0,
            y_screen: 0.0,
        }
    }
}
pub struct InputState {
    pub keys_down: FxHashMap<CompactString, bool>,
    pub mouse_position: MousePosition,
    pub mouse_left: bool,
    pub mouse_right: bool,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    start,
    play,
    inventory,
    death,
}
pub struct Game<'a> {
    pub world: World,
    pub camera: Camera,
    pub renderer: Renderer<'a>,
    input: InputState,
    state: GameState,
}

impl<'a> Game<'a> {
    pub fn new(world: World, camera: Camera, renderer: Renderer<'a>) -> Game<'a> {
        let cx = camera.camera_x;
        let cy = camera.camera_y;
        Self {
            world,
            camera,
            renderer,
            state: GameState::start,
            input: InputState {
                keys_down: FxHashMap::default(),
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
    pub fn process_mouse_click(&mut self, state: event::ElementState, button: event::MouseButton) -> Result<(), PError>{
        if self.state == GameState::start {
            self.state = GameState::play;
            return Ok(());
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
            ptry!(self.on_mouse_click());
        }
        Ok(())

    }
    pub fn on_mouse_click(&mut self) -> Result<(), PError> {
        if self.state == GameState::play {
            ptry!(self.world.on_mouse_click(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right, self.camera.viewpoint_width as f32, self.camera.viewpoint_height as f32));
        }else if self.state == GameState::inventory {
            ptry!(self.world.inventory.on_mouse_click(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right));
        }
        Ok(())
    }
    pub fn process_input(&mut self) -> Result<(), PError> {
        if self.state == GameState::play {
            ptry!(self.world.process_mouse_input(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right));
            ptry!(self.world.process_input(&self.input.keys_down, &mut self.camera, &self.input));
        }else if self.state == GameState::inventory {
            self.world.inventory.process_mouse_input(self.input.mouse_position, self.input.mouse_left, self.input.mouse_right);
            self.world.inventory.process_input(&self.input.keys_down);
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), PError> {
        if self.state == GameState::start {
            match self.renderer.render(
                punwrap!(self.world.sprites.get_sprite_by_name("start_screen"), MissingExpectedGlobalSprite, "no start screen sprite").draw_data(0.0, 0.0, self.camera.viewpoint_width, self.camera.viewpoint_height, self.camera.viewpoint_width, self.camera.viewpoint_height, 0, 0, 0).to_full()
            ) {
                Ok(_) => {}
                Err(e) => {
                    return Err(PError::new(crate::error::PE::SurfaceError(e), vec![]));
                }
            }
            return Ok(());
        }
        let uie = ptry!(self.world.inventory.render_ui());
        match self.renderer.render(ptry!(self.camera.render(&mut self.world, uie, self.renderer.config.width as f32, self.renderer.config.height as f32))){
            Ok(_) => {Ok(())}
            Err(e) => {
                Err(PError::new(crate::error::PE::SurfaceError(e), vec![]))
            }
        }
    }
    pub fn update(&mut self) -> Result<(), PError> {
        if self.state == GameState::play {
            ptry!(self.camera.update_ui(&mut self.world));
            ptry!(self.world.generate_collision_cache_and_damage_cache());
            ptry!(self.process_input());
            ptry!(self.world.update_entities());
            ptry!(self.world.update_entity_attacks());
            ptry!(self.world.update_player_abilities(&self.input));
            self.world.update_player_attacks(&mut self.camera);
            ptry!(self.world.update_damage_text(&mut self.camera));
            ptry!(self.world.update_items_on_ground());
            ptry!(self.world.kill_entities_to_be_killed());
            self.input.mouse_position.x_world = self.camera.camera_x + self.input.mouse_position.x_screen;
            self.input.mouse_position.y_world = self.camera.camera_y + self.input.mouse_position.y_screen;
            ptry!(self.world.update_items_in_inventory_cd());
            if self.world.player.borrow().health <= 0.0 {
                panic!("\n\nplayer died\n\n");
            }
        }else if self.state == GameState::inventory {
            ptry!(self.camera.update_ui(&mut self.world));
            ptry!(self.process_input());
            self.input.mouse_position.x_world = self.camera.camera_x + self.input.mouse_position.x_screen;
            self.input.mouse_position.y_world = self.camera.camera_y + self.input.mouse_position.y_screen;
        }
        Ok(())
    }
    pub fn key_input(&mut self, event: winit::event::KeyEvent) -> Result<(), PError> {

        if self.state == GameState::start {
            self.state = GameState::play;
            return Ok(());
        }

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
            return Ok(());
        }
        let string_key = key.unwrap().to_compact_string().to_lowercase();
        let press = match event.state {
            event::ElementState::Pressed => true,
            event::ElementState::Released => false,
        };

        if press {
            ptry!(self.on_key_down(&string_key));
        }
        
        self.input.keys_down.insert(string_key, press);
        Ok(())
    }

    pub fn on_key_down(&mut self, key: &CompactString) -> Result<(), PError>{
        if key == "e" {
            self.state = match self.state {
                GameState::play => {
                    self.world.inventory.show_inventory();
                    GameState::inventory
                },
                GameState::inventory => {
                    match self.world.inventory.hide_inventory() {
                        Ok(_) => {
                            ptry!(self.world.process_inventory_close())
                        },
                        Err(e) => {
                            println!("Error hiding inventory: {:?}", e);
                        }
                    }
                    GameState::play
                },
                _ => self.state,
            };
            return Ok(());
        }
        if self.state == GameState::play {
            ptry!(self.world.on_key_down(key, &self.input));
        } else if self.state == GameState::inventory {
            self.world.inventory.on_key_down(key);
        }
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.size = new_size;
            self.renderer.config.width = new_size.width;
            self.renderer.config.height = new_size.height;
            self.renderer.surface.configure(&self.renderer.device, &self.renderer.config);
            self.renderer.text_brush_a_b.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
            self.renderer.text_brush_b_b.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
            self.renderer.text_brush_a_t.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
            self.renderer.text_brush_b_t.resize_view(self.renderer.config.width as f32, self.renderer.config.height as f32, &self.renderer.queue);
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        self.renderer.window()
    }

    
}
