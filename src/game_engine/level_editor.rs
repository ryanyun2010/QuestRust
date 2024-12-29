use std::collections::HashMap;
use std::fmt::format;
use wgpu_text::glyph_brush::HorizontalAlign;
use winit::event::{ElementState, MouseButton, *};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use super::json_parsing::{entity_json, terrain_json, JSON_parser, ParsedData};
use super::starting_level_generator::match_terrain_tags;
use super::ui::UIElement;
use super::world::World;
use super::camera::{self, Camera};
use crate::rendering_engine::abstractions::{RenderData, SpriteIDContainer};
use crate::rendering_engine::state::State;
use super::player::Player;

pub struct LevelEditor{
    pub highlighted: Option<usize>,
    pub parser: JSON_parser,
    pub world: World,
    pub sprites: SpriteIDContainer,
    pub not_real_elements: HashMap<usize, bool>,
    pub object_descriptor_hash: HashMap<usize, ObjectJSON>,
    pub last_query: Option<Vec<ObjectJSON>>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_x_screen: f32,
    pub mouse_y_screen: f32,
    pub query_text: Option<usize>,
    pub query_unique_ui_elements: Vec<usize>,
    pub query_unique_text_elements: Vec<usize>,
    pub clicked_query_element: Option<usize>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum MouseClick{
    Left,
    Right
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectJSON {
    Entity(entity_json),
    Terrain(terrain_json)
}

impl LevelEditor{
    pub fn new(world: World, sprites: SpriteIDContainer, parser: JSON_parser, hash: HashMap<usize, ObjectJSON>) -> Self{
        Self {
            highlighted: None,
            world: world,
            parser: parser,
            sprites: sprites,
            not_real_elements: HashMap::new(),
            object_descriptor_hash: hash,
            last_query: None,
            mouse_x_screen: 0.0,
            mouse_y_screen: 0.0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            query_text: None,
            query_unique_ui_elements: Vec::new(),
            query_unique_text_elements: Vec::new(),
            clicked_query_element: None,
        }
    }
    pub fn init(&mut self, camera: &mut Camera){
        self.world.set_level_editor();
        self.add_level_editor_grid(self.sprites.get_sprite("grid"));
        camera.add_ui_element("menu".to_string(), super::ui::UIElement {
            x: 932.0,
            y: 40.0,
            width: 200.0,
            height: 640.0,
            texture_id: self.sprites.get_texture_id("level_editor_menu_background"),
            visible: true
        });
        camera.add_text("Level Editor".to_string(), 938.0, 45.0,60.0, 24.5, 24.5, [0.7,0.7,0.7,1.0], HorizontalAlign::Left);
        camera.add_ui_element("save_button".to_string(), super::ui::UIElement {
            x: 1008.0,
            y: 45.0,
            width: 40.0,
            height: 12.5,
            texture_id: self.sprites.get_texture_id("level_editor_button_background"),
            visible: true
        });
        camera.add_text("Save".to_string(), 1028.0, 45.0,60.0, 22.0, 22.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center);
        self.query_text = Some(camera.add_text("PLACEHOLDER".to_string(), 942.0, 120.0,180.0, 400.0, 28.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
    }
    pub fn save_edits(&self){
        self.parser.write("src/game_data/entity_archetypes.json", "src/game_data/entity_attack_patterns.json", "src/game_data/entity_attacks.json", "src/game_data/sprites.json", "src/game_data/starting_level.json").expect("d");
    }
    pub fn query_stuff_at(&self, x: usize, y: usize) -> Vec<ObjectJSON>{
        println!("Query at {}, {}", x, y);
        let chunk_to_query = self.world.get_chunk_from_xy(x, y);
        let mut vec_json = Vec::new();
        if chunk_to_query.is_none(){
            return vec_json;
        }else{
            let chunk_id = chunk_to_query.unwrap();
            let chunk = self.world.chunks.borrow()[chunk_id].clone();
            for entity_id in chunk.entities_ids{
                let entity = &self.world.get_entity(entity_id).unwrap();
                if entity.x <= x as f32 && entity.x + 32.0 >= x as f32 && entity.y <= y as f32 && entity.y + 32.0 >= y as f32 {
                    let descriptor = self.object_descriptor_hash.get(&entity_id).unwrap().clone();
                    vec_json.push(descriptor);
                }
            }

            for terrain_id in chunk.terrain_ids{
                let terrain = self.world.get_terrain(terrain_id).unwrap();
                if *self.not_real_elements.get(&terrain_id).unwrap_or(&false){
                    continue;
                }
                let x_for_terrain = (x as f32 / 32.0).floor() as usize * 32 + 16;
                let y_for_terrain = (y as f32 / 32.0).floor() as usize * 32 + 16;
                if terrain.x <= x_for_terrain && terrain.x + 32 >= x && terrain.y <= y_for_terrain && terrain.y + 32 >= y{
                    let descriptor = self.object_descriptor_hash.get(&terrain_id).unwrap().clone();
                    vec_json.push(descriptor);
                }
            }   
        }
        return vec_json;
    }
    pub fn on_click(&mut self, mouse: MouseClick){
        if mouse == MouseClick::Left{
            if self.highlighted.is_some(){
                self.last_query = Some(self.query_stuff_at(self.mouse_x_screen.floor() as usize, self.mouse_y_screen.floor() as usize));
            }
            for i in 0..self.last_query.clone().unwrap_or(Vec::new()).len(){
                if self.mouse_x_screen > 942.0 + 45.0 * i as f32 && self.mouse_x_screen < 942.0 + 45.0 * i as f32 + 40.0 && self.mouse_y_screen > 90.0 && self.mouse_y_screen < 105.0{
                    self.clicked_query_element = Some(i);
                }
            }
        } else if mouse == MouseClick::Right{
            if self.highlighted.is_some(){
                let terrain_id = self.highlighted.unwrap();
                let terrain = self.world.get_terrain(terrain_id).unwrap();
                let x = terrain.x;
                let y = terrain.y;
                let terrain_json = super::json_parsing::terrain_json {
                    x: x/32,
                    y: y/32,
                    width: 1,
                    height: 1,
                    terrain_descriptor: super::json_parsing::terrain_descriptor_json {
                        r#type: String::from("basic"),
                        random_chances: None,
                        basic_tags: Vec::new(),
                        sprites: vec![String::from("wall")]
                    }
                };
                self.parser.starting_level_json.terrain.push(terrain_json.clone());
                let new_terrain = self.world.add_terrain(x, y);
                self.object_descriptor_hash.insert(new_terrain, ObjectJSON::Terrain(terrain_json));
                self.world.set_sprite(new_terrain, self.sprites.get_sprite("wall"));
            }
        }
    }
    pub fn process_mouse_input (&mut self, left_mouse_button_down: bool, right_mouse_button_down: bool){
    }
    pub fn highlight_square(&mut self){
        let sprite_id = self.sprites.get_sprite("highlight");
        let tx = (self.mouse_x as f32 / 32.0).floor() as usize * 32;
        let ty = (self.mouse_y as f32 / 32.0).floor() as usize * 32;
        if tx > 932 && tx < 1132 && ty > 40 && ty < 680{
            if self.highlighted.is_some(){
                self.world.remove_terrain(self.highlighted.unwrap());
            }
            self.highlighted = None; 
            return;
        }
        let terrain = self.world.add_terrain(tx, ty);
        self.world.set_sprite(terrain, sprite_id); 
        if self.highlighted.is_some(){
            self.world.remove_terrain(self.highlighted.unwrap());
        }
        self.not_real_elements.insert(terrain, true);
        self.highlighted = Some(terrain);
    }
    pub fn add_level_editor_grid(&mut self, sprite_id: usize){
        for x in 0..1000{
            for y in 0..1000{
                let terrain = self.world.add_terrain(x * 32, y * 32);
                self.world.set_sprite(terrain, sprite_id);
                self.not_real_elements.insert(terrain, true);
            }
        }
    }
    pub fn update_camera_ui(&mut self, camera: &mut Camera){
        if self.query_text.is_none(){
            return;
        }
        let mut i = 0;
        for ui in self.query_unique_ui_elements.clone(){
            camera.remove_ui_element(ui);
        }
        for text in self.query_unique_text_elements.clone(){
            camera.remove_text(text);
        }
        for element in self.last_query.clone().unwrap_or(Vec::new()){
            println!("Element {:?}", element);
            self.query_unique_ui_elements.push(camera.add_ui_element(format!("level_editor_query_button_{}", i), UIElement{
                x: 942.0 + 45.0 * i as f32,
                y: 90.0,
                width: 40.0,
                height: 15.0,
                texture_id: self.sprites.get_texture_id("level_editor_button_background"),
                visible: true
            }));
            let text: String = match element {
                ObjectJSON::Entity(entity) => {
                    format!("{}. Entity", i + 1)
                },
                ObjectJSON::Terrain(terrain) => {
                    format!("{}. Terrain", i + 1)
                }
            };
            self.query_unique_text_elements.push(camera.add_text(text, 962.0 + 45.0 * i as f32, 93.0, 40.0, 18.0, 18.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center));
            i+= 1;
        }
        if let Some(last_query) = &self.last_query {
            if let Some(clicked_query_element) = self.clicked_query_element {
                camera.text.get_mut(&self.query_text.unwrap()).unwrap().text = format!("{:?}",last_query[clicked_query_element]);
            }
        }
        
    }


}


impl World { // TODO: ALL THE CODE IN THIS IMPL SHOULD BE MOVED TO LEVEL EDITOR
    pub fn set_level_editor(&mut self){
        self.level_editor = true;
        self.player.borrow_mut().movement_speed = 5.0;
    }


    pub fn level_editor_process_input(&mut self, keys: HashMap<String,bool>){
        let mut direction: [f32; 2] = [0.0,0.0];
        let mut player: std::cell::RefMut<'_, Player> = self.player.borrow_mut();
        if *keys.get("w").unwrap_or(&false) || *keys.get("ArrowUp").unwrap_or(&false){
            direction[1] -= 1.0;
        }
        if *keys.get("a").unwrap_or(&false) || *keys.get("ArrowLeft").unwrap_or(&false){
            direction[0] -= 1.0;
        }
        if *keys.get("s").unwrap_or(&false) || *keys.get("ArrowDown").unwrap_or(&false){
            direction[1] += 1.0;
        }
        if *keys.get("d").unwrap_or(&false) || *keys.get("ArrowRight").unwrap_or(&false){
            direction[0] += 1.0;
        }

        let magnitude: f32 = f32::sqrt(direction[0].powf(2.0) + direction[1].powf(2.0));
        if magnitude > 0.0 {
            let movement = [(direction[0] / magnitude * player.movement_speed).round(), (direction[1] / magnitude * player.movement_speed).round()];
            if player.x + movement[0] < 576.0 && player.y + movement[1] < 360.0 {
                
            }else if player.x + movement[0] < 576.0{
                if direction[1].abs() > 0.0{
                    player.y += (direction[1]/ direction[1].abs() * player.movement_speed).round();
                }
            } else if player.y + movement[1] < 360.0{
                if direction[0].abs() > 0.0{
                    player.x += (direction[0]/ direction[0].abs() * player.movement_speed).round();
                }
            } else {
                player.x += movement[0];
                player.y += movement[1];
            }
        }

        if player.y < 360.0 {
            player.y = 360.0;
        }
        if player.x < 576.0 {
            player.x = 576.0;
        }
    }
    
}
impl Camera{
    pub fn set_level_editor(&mut self){
        self.level_editor = true;
    }
    pub fn level_editor_update_camera_position(&mut self, level_editor: &mut LevelEditor){
        let player = level_editor.world.player.borrow().clone();
        let mut direction = [player.x - (self.viewpoint_width / 2) as f32 - self.camera_x, player.y - (self.viewpoint_height / 2) as f32 - self.camera_y];
        if self.camera_x < 4.0 && direction[0] < 0.0{
            direction[0] = 0.0;
        }
        if self.camera_y < 4.0 && direction[1] < 0.0{
            direction[1] = 0.0;
        }

        
        let magnitude = (direction[0].powi(2) + direction[1].powi(2)).sqrt();

        
        if magnitude < 10.0{
            return;
        }
        self.camera_x += (direction[0]/magnitude * 5.0).round();
        self.camera_y += (direction[1]/magnitude * 5.0).round();

        if self.camera_x < 0.0{
            self.camera_x = 0.0;
        }
        if self.camera_y < 0.0{
            self.camera_y = 0.0;
        }
        level_editor.mouse_x = level_editor.mouse_x_screen + self.camera_x;
        level_editor.mouse_y = level_editor.mouse_y_screen + self.camera_y;
    }
    pub fn level_editor_render(&mut self, world: &mut World) -> RenderData{
        let mut render_data = RenderData::new();
        let mut terrain_data: RenderData = RenderData::new();
        let mut entity_data: RenderData = RenderData::new();
        let mut index_offset: u16 = 0;
        let player = world.player.borrow().clone(); 


        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
        let camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize) + 1;

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
        let camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize) + 1; 

        let mut chunks_loaded = Vec::new();
        for x in camera_left_chunk_x..camera_right_chunk_x{
            for y in camera_top_chunk_y..camera_bot_chunk_y{
                
                let chunk_id = world.get_chunk_from_chunk_xy(x,y);
                
                if chunk_id.is_none(){
                    continue;
                }
                let chunk = &world.chunks.borrow()[chunk_id.unwrap()];

                chunks_loaded.push(chunk_id.unwrap());
                for terrain_id in chunk.terrain_ids.iter(){
                    let potentially_sprite_id = world.get_sprite(*terrain_id);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &world.sprites[sprite_id];

                    
                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;
                    let terrain = world.get_terrain(*terrain_id).unwrap();
                    let draw_data = sprite.draw_data(terrain.x as f32, terrain.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
                    index_offset += 4;
                    terrain_data.vertex.extend(draw_data.vertex);
                    terrain_data.index.extend(draw_data.index);
                }

                for entity_id in chunk.entities_ids.iter(){
                    let potentially_sprite_id = world.get_sprite(*entity_id);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &world.sprites[sprite_id];
                    
                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;

                    let entity = world.get_entity(*entity_id).unwrap();

                    let draw_data = sprite.draw_data(entity.x, entity.y, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
                    index_offset += 4;
                    entity_data.vertex.extend(draw_data.vertex);
                    entity_data.index.extend(draw_data.index);
                }
            }
        }
        render_data.vertex.extend(terrain_data.vertex);
        render_data.vertex.extend(entity_data.vertex);
        render_data.index.extend(terrain_data.index);
        render_data.index.extend(entity_data.index);

        for (id, element) in self.ui_elements.iter(){
            if !element.visible{
                continue;
            }
            let index_offset = render_data.vertex.len() as u16;
            let draw_data = element.draw_data(self.viewpoint_width, self.viewpoint_height, index_offset);
            render_data.vertex.extend(draw_data.vertex);
            render_data.index.extend(draw_data.index);
        }
        
        render_data
    }
}
impl State<'_>{
    pub fn set_level_editor(&mut self){
        self.level_editor = true;
    }
    pub fn level_editor_highlight_square(&mut self, level_editor: &mut LevelEditor){
        level_editor.highlight_square();
    }
    pub fn level_editor_process_mouse_input(&mut self, state: ElementState, button: MouseButton){
        if state == ElementState::Pressed && button == MouseButton::Left{
            self.left_mouse_button_down = true;
        } else if state == ElementState::Released && button == MouseButton::Left{
            self.left_mouse_button_down = false;
        } else if state == ElementState::Pressed && button == MouseButton::Right{
            self.right_mouse_button_down = true;
        } else if state == ElementState::Released && button == MouseButton::Right{
            self.right_mouse_button_down = false;
        }
    }
    pub fn level_editor_update(&mut self, level_editor: &mut LevelEditor, camera: &mut Camera){
        level_editor.world.level_editor_process_input(self.keys_down.clone());
        level_editor.process_mouse_input(self.left_mouse_button_down, self.right_mouse_button_down);
        camera.level_editor_update_camera_position(level_editor);
        self.level_editor_highlight_square(level_editor);
        level_editor.update_camera_ui(camera);
    }
    pub fn process_mouse_position(&mut self, x: f64, y: f64, level_editor: &mut LevelEditor, camera: &Camera){
        level_editor.mouse_x_screen = x as f32/self.config.width as f32 * camera.viewpoint_width as f32;
        level_editor.mouse_y_screen = y as f32 /self.config.height as f32 * camera.viewpoint_height as f32;
        level_editor.mouse_x = level_editor.mouse_x_screen + camera.camera_x;
        level_editor.mouse_y = level_editor.mouse_y_screen + camera.camera_y;
    }
}

pub async fn run(mut level_editor: &mut LevelEditor, camera: &mut Camera, sprites_json_to_load: Vec<String>) {
    let event_loop = EventLoop::new().unwrap();
    let title = "Level Editor";
    let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let mut state_obj = State::new(&window, sprites_json_to_load.clone()).await;
    state_obj.set_level_editor();

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
                WindowEvent::CloseRequested => {
                    level_editor.save_edits();
                    control_flow.exit()
                },
                WindowEvent::Resized(physical_size) => {
                    state_obj.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                    state_obj.process_mouse_position(position.x, position.y, &mut level_editor, &camera);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    state_obj.level_editor_process_mouse_input(state, button);
                    if state == ElementState::Pressed{
                        match button{
                            MouseButton::Left => {
                                level_editor.on_click(MouseClick::Left);
                            },
                            MouseButton::Right => {;
                                level_editor.on_click(MouseClick::Right);
                            },
                            _ => {}
                        }
                    }

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
                    state_obj.level_editor_update(&mut level_editor, camera);
                    match state_obj.render(&mut level_editor.world, camera) {
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

pub fn level_editor_generate_world_from_json_parsed_data(data: &ParsedData) -> (World, SpriteIDContainer, HashMap<usize, ObjectJSON>) {

    
    let starting_level_descriptor = data.starting_level_descriptor.clone();
    let player_descriptor = starting_level_descriptor.player;
    let mut world = World::new(Player::new(player_descriptor.x, player_descriptor.y, player_descriptor.health, player_descriptor.max_health, player_descriptor.movement_speed, data.get_texture_id(&player_descriptor.sprite)));
    let sprites = SpriteIDContainer::generate_from_json_parsed_data(data, &mut world);
    let mut hash = HashMap::new();
    for entity_descriptor in starting_level_descriptor.entities.iter(){
        let entity = world.create_entity_from_json_archetype(entity_descriptor.x, entity_descriptor.y, &entity_descriptor.archetype, data);
        world.set_sprite(entity, sprites.get_sprite(&entity_descriptor.sprite));
        hash.insert(entity, ObjectJSON::Entity(entity_descriptor.clone()));
    }
    
    for terrain_json in starting_level_descriptor.terrain.iter(){
        let start_x = terrain_json.x;
        let start_y = terrain_json.y;
        let width = terrain_json.width;
        let height = terrain_json.height;
        let descriptor = terrain_json.terrain_descriptor.clone();
        let tags = descriptor.basic_tags.clone();
        match descriptor.r#type.as_str() {
            "basic" => {
                for x in start_x..start_x + width{
                    for y in start_y..start_y + height{
                        let terrain = world.add_terrain(x * 32, y * 32);
                        world.set_sprite(terrain, sprites.get_sprite(&descriptor.sprites[0]));
                        hash.insert(terrain, ObjectJSON::Terrain(terrain_json.clone()));
                        match_terrain_tags(&tags, terrain, &mut world);
                    }
                }
            },
            "randomness" => {
                println!("Randomness {:?}", descriptor);
                let random_chances = descriptor.random_chances.unwrap();
                let mut random_chances_adjusted = Vec::new();
                let mut sum_so_far = 0.0;
                for chance in random_chances{
                    random_chances_adjusted.push(chance + sum_so_far);
                    sum_so_far += chance;
                }
                for x in start_x..start_x + width{
                    for y in start_y..start_y + height{
                        let terrain = world.add_terrain(x * 32, y * 32);
                        let random_number = rand::random::<f32>();
                        for (index, chance) in random_chances_adjusted.iter().enumerate(){
                            if random_number < *chance{
                                world.set_sprite(terrain, sprites.get_sprite(&descriptor.sprites[index]));
                                break;
                            }
                        }
                        hash.insert(terrain, ObjectJSON::Terrain(terrain_json.clone()));
                        match_terrain_tags(&tags, terrain, &mut world);
                    }
                }
            },
            _ => {
                panic!("Unknown terrain type: {}", descriptor.r#type);
            }
        }


    }
    (world, sprites, hash)
}