use std::collections::HashMap;
use winit::event::{ElementState, MouseButton, *};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use super::json_parsing::{entity_json, terrain_json, JSON_parser, ParsedData};
use super::starting_level_generator::match_terrain_tags;
use super::world::World;
use super::camera::Camera;
use crate::rendering_engine::abstractions::{RenderData, SpriteIDContainer};
use crate::rendering_engine::state::State;
use super::player::Player;

pub struct LevelEditor{
    pub highlighted: Option<usize>,
    pub parser: JSON_parser,
    pub world: World,
    pub sprites: SpriteIDContainer,
    pub not_real_elements: HashMap<usize, bool>,
    pub object_descriptor_hash: HashMap<usize, ObjectJSON>
}

#[derive(Debug, Clone)]
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
            object_descriptor_hash: hash
        }
    }
    pub fn init(&mut self){
        self.world.set_level_editor();
        self.add_level_editor_grid(self.sprites.get_sprite("grid"));
    }
    pub fn save_edits(&self){
        self.parser.write("src/game_data/entity_archetypes.json", "src/game_data/entity_attack_patterns.json", "src/game_data/entity_attacks.json", "src/game_data/sprites.json", "src/game_data/starting_level.json").expect("d");
    }
    pub fn query_stuff_at(&self, x: usize, y: usize) -> Vec<ObjectJSON>{
        println!("Query at {}, {}", x, y);
        let chunk_to_query = self.world.get_chunk_from_xy(x, y);
        if chunk_to_query.is_none(){
            return Vec::new();
        }else{
            let chunk_id = chunk_to_query.unwrap();
            let chunk = self.world.chunks.borrow()[chunk_id].clone();
            for entity_id in chunk.entities_ids{
                let entity = &self.world.get_entity(entity_id).unwrap();
                if entity.x <= x as f32 && entity.x + 32.0 >= x as f32 && entity.y <= y as f32 && entity.y + 32.0 >= y as f32 {
                    println!("Found Entity {:?}", entity);
                    println!("From JSON: {:?}", self.object_descriptor_hash.get(&entity_id).unwrap());
                }
            }
            for terrain_id in chunk.terrain_ids{
                let terrain = self.world.get_terrain(terrain_id).unwrap();
                if *self.not_real_elements.get(&terrain_id).unwrap_or(&false){
                    continue;
                }
                if terrain.x <= x && terrain.x + 32 >= x && terrain.y <= y && terrain.y + 32 >= y{
                    println!("Found Terrain: {:?}", terrain);
                    println!("From JSON: {:?}", self.object_descriptor_hash.get(&terrain_id).unwrap());
                }
                
            }
            return Vec::new();
        }

    }
    pub fn process_mouse_input (&mut self, left_mouse_button_down: bool, right_mouse_button_down: bool){
        if right_mouse_button_down{
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
                /* this should probably replace the terrain under it honestly or at least you shouldnt be placing a whole bunch of terrain on top of eachother accidentally */
            }
        }
        if left_mouse_button_down{
            if self.highlighted.is_some(){
                let terrain_id = self.highlighted.unwrap();
                let terrain = self.world.get_terrain(terrain_id).unwrap();
                let x = terrain.x;
                let y = terrain.y;
                self.query_stuff_at(x + 16, y + 16);
            
            }
        }
    }
    pub fn highlight_square(&mut self, x: f64, y: f64, width: u32, height: u32, camera_width: usize, camera_height: usize, camera_x: f32, camera_y: f32){
        let sprite_id = self.sprites.get_sprite("highlight");
        let tx = (((((x as f32)/width as f32) * camera_width as f32) + camera_x) / 32.0).floor() as usize * 32;
        let ty = (((((y as f32)/height as f32) * camera_height as f32) + camera_y)/ 32.0).floor() as usize * 32;
        let terrain = self.world.add_terrain(tx, ty);
        self.world.set_sprite(terrain, sprite_id); 
        if self.highlighted.is_some(){
            self.world.sprite_lookup.remove(&self.highlighted.unwrap());
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
    pub fn level_editor_update_camera_position(&mut self, world: &World){
        let player = world.player.borrow().clone();
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

        for i in 0..self.ui_elements.len(){
            let element = &self.ui_elements[i];
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
    pub fn level_editor_highlight_square(&mut self, level_editor: &mut LevelEditor, x: f64, y: f64, camera: &Camera){
        level_editor.highlight_square(x, y, self.size.width, self.size.height, camera.viewpoint_width, camera.viewpoint_height, camera.camera_x, camera.camera_y);
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
    pub fn level_editor_update(&self, level_editor: &mut LevelEditor, camera: &mut Camera){
        level_editor.world.level_editor_process_input(self.keys_down.clone());
        level_editor.process_mouse_input(self.left_mouse_button_down, self.right_mouse_button_down);
        camera.level_editor_update_camera_position(&level_editor.world);
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
                    state_obj.level_editor_highlight_square(&mut level_editor,   position.x, position.y, &camera);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    state_obj.level_editor_process_mouse_input(state, button);
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