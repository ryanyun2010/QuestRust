use core::arch;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::format;
use std::vec;
use wgpu::util::DeviceExt;
use wgpu_text::glyph_brush::HorizontalAlign;
use winit::event::{ElementState, MouseButton, *};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;
use crate::game_engine::entities::Entity;
use crate::game_engine::json_parsing::{entity_json, terrain_json, JSON_parser, ParsedData};
use crate::game_engine::starting_level_generator::match_terrain_tags;
use crate::game_engine::terrain::Terrain;
use crate::game_engine::ui::UIElementDescriptor;
use crate::game_engine::world::World;
use crate::game_engine::camera::{self, Camera};
use crate::game_engine::input_handling::{self, InputEvent, InputState};
use super::input_handling_layer::LevelEditorInputLayer;
use crate::rendering_engine::abstractions::{RenderData, SpriteIDContainer};
use crate::rendering_engine::state::State;
use crate::game_engine::player::Player;
use crate::game_engine::command_line_input;
use crate::state::BACKGROUND_COLOR;
#[derive(Debug, Clone, PartialEq)]
pub enum EditableProperty{
    EntityX,
    EntityY,
    EntityArchetype,
    EntitySprite,
    TerrainX,
    TerrainY,
    TerrainW,
    TerrainH,
    TerrainArchetype
}
#[derive(Debug, Clone, PartialEq)]
pub enum EntityPropertyValue{
    X(f32),
    Y(f32),
    Archetype(String),
    Sprite(String)
}
#[derive(Debug, Clone, PartialEq)]
pub enum TerrainPropertyValue{
    X(usize),
    Y(usize),
    W(usize),
    H(usize),
    Archetype(String),
}
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
#[derive(Debug, Clone, PartialEq)]
pub enum QueryType{
    Position,
    FollowingObject
}
#[derive(Debug, Clone)]
pub struct QueryResult{
    pub query_type: QueryType,
    pub position: Option<(usize, usize)>,
    pub objects: Vec<QueriedObject>,
}
#[derive(Debug, Clone)]
pub struct QueriedObject{
    pub object: ObjectJSONContainer,
    pub element_id: usize
}
pub struct LevelEditor{
    pub highlighted: Option<usize>,
    pub grid: Vec<Terrain>,
    pub grid_sprite: Option<usize>,
    pub parser: JSON_parser,
    pub parsed_data: ParsedData,
    pub world: World,
    pub sprites: SpriteIDContainer,
    pub not_real_elements: HashMap<usize, bool>,
    pub object_descriptor_hash: HashMap<usize, ObjectJSONContainer>,
    pub mouse_position: MousePosition,
    pub query_at_text: Option<usize>,
    pub last_query: Option<QueryResult>,
    pub cur_editing: Option<EditableProperty>,
    pub typed: String,
    pub query_unique_ui_elements: Vec<usize>,
    pub query_unique_text_elements: Vec<usize>
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum MouseClick{
    Left,
    Right
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectJSONContainer { // usize here is the index in the starting_level_json entity/json
    Entity((entity_json, usize)),
    Terrain((terrain_json, usize))
}

macro_rules! update_entity_property_json {
    ($self:ident, $property:ident, $new_value:expr, $type:ty) => {{
        let new_property: $type = $new_value;
        let last_query = $self.last_query.clone().unwrap();
        if last_query.query_type != QueryType::FollowingObject{
            return;
        }
        let entity_id = last_query.objects[0].element_id;
        let object = last_query.objects[0].object.clone();
        let mut new_object = object.clone();
        match &mut new_object {
            ObjectJSONContainer::Entity(ref mut obj) => {
                obj.0.$property = new_property.clone();
            }
            _ => {}
        }
        $self.last_query = Some(
            QueryResult {
                query_type: QueryType::FollowingObject,
                position: None,
                objects: vec![
                    QueriedObject{
                        element_id: entity_id,
                        object: new_object.clone()
                    }]
            }
        );
        match object {
            ObjectJSONContainer::Entity(mut obj) => {
                $self.parser.starting_level_json.entities[obj.1].$property = new_property.clone();
            },
            _ => {}
        }
        match $self.object_descriptor_hash.get_mut(&entity_id).unwrap() {
            ObjectJSONContainer::Entity(entity) => {
                entity.0.$property = new_property.clone();
            },
            _ => {}
        }
        entity_id
    }
}}
macro_rules! update_terrain_property_json {
    ($self:ident, $property:ident, $new_value:expr, $type:ty) => {{
        let new_property: $type = $new_value;
        let last_query = $self.last_query.clone().unwrap();
        if last_query.query_type != QueryType::FollowingObject{
            return;
        }
        let mut terrain_object = last_query.objects[0].clone();
        match terrain_object.object.clone(){
            ObjectJSONContainer::Terrain(terrain) => {
                let parser_id = terrain.1;
                $self.parser.starting_level_json.terrain[parser_id].$property = new_property.clone();
            }
            _ => {}
        }

        match &mut terrain_object.object{
            ObjectJSONContainer::Terrain(ref mut terrain) => {
                terrain.0.$property = new_property.clone();
            }
            _ => {}
        }
        let (world, sprites, hash) = super::generate_world::generate_world_from_json_parsed_data(&$self.parser.convert());
        $self.object_descriptor_hash = hash;
        let player_x = $self.world.player.borrow().x.floor();
        let player_y = $self.world.player.borrow().y.floor();
        $self.world = world;
        $self.sprites = sprites;
        $self.last_query = Some(QueryResult{
            query_type: QueryType::FollowingObject,
            position: None,
            objects: vec![terrain_object]
        });
        $self.world.set_level_editor();
        $self.world.player.borrow_mut().x = player_x;
        $self.world.player.borrow_mut().y = player_y;
    }
}}



impl LevelEditor{
    pub fn new(world: World, sprites: SpriteIDContainer, parser: JSON_parser, hash: HashMap<usize, ObjectJSONContainer>) -> Self{
        let parsed_data = parser.clone().convert();
        Self {
            highlighted: None,
            world: world,
            parser: parser,
            grid: Vec::new(),
            grid_sprite: None,
            parsed_data: parsed_data,
            sprites: sprites,
            not_real_elements: HashMap::new(),
            object_descriptor_hash: hash,
            last_query: None,
            mouse_position: MousePosition::default(),
            query_at_text: None,
            typed: String::new(),
            cur_editing: None,
            query_unique_ui_elements: Vec::new(),
            query_unique_text_elements: Vec::new()
        }
    }
    pub fn init(&mut self, camera: &mut Camera, input_handling_layer: &mut LevelEditorInputLayer){
        self.world.set_level_editor();
        self.add_level_editor_grid(self.sprites.get_sprite("grid").unwrap());
        self.grid_sprite = Some(self.sprites.get_sprite("grid").unwrap());
        camera.add_ui_element("menu".to_string(), crate::game_engine::ui::UIElementDescriptor {
            x: 932.0,
            y: 40.0,
            width: 200.0,
            height: 640.0,
            texture_id: self.sprites.get_texture_id("level_editor_menu_background").unwrap(),
            visible: true
        });
        camera.add_text("Level Editor".to_string(), 938.0, 45.0,60.0, 24.5, 24.5, [0.7,0.7,0.7,1.0], HorizontalAlign::Left);
        camera.add_ui_element("save_button".to_string(), crate::game_engine::ui::UIElementDescriptor {
            x: 1008.0,
            y: 45.0,
            width: 40.0,
            height: 12.5,
            texture_id: self.sprites.get_texture_id("level_editor_button_background").unwrap(),
            visible: true
        });
        camera.add_text("Save".to_string(), 1028.0, 45.0,60.0, 22.0, 22.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center);
        self.query_at_text = Some(camera.add_text("".to_string(), 942.0, 70.0,180.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
        input_handling_layer.add_key_down_callback(Box::new(|state: &InputState, event: &InputEvent, level_editor: &mut LevelEditor, camera: &mut Camera, state_obj: &State |{
            level_editor.key_down(event);
        }));
        input_handling_layer.add_mouse_click_callback(Box::new(|state: &InputState, event: &InputEvent, level_editor: &mut LevelEditor, camera: &mut Camera, state_obj: &State|{
            level_editor.on_click(event, camera);
        }));
        input_handling_layer.add_mouse_move_callback(Box::new(|state: &InputState, event: &InputEvent, level_editor: &mut LevelEditor, camera: &mut Camera, state_obj: &State|{
            level_editor.on_mouse_move(state, camera, state_obj);
        }));

    }
    pub fn on_mouse_move(&mut self, state: &InputState, camera: &mut Camera, state_obj: &State){
        let x = state.mouse_position.0;
        let y = state.mouse_position.1;
        self.mouse_position.x_screen = x as f32/state_obj.config.width as f32 * camera.viewpoint_width as f32;
        self.mouse_position.y_screen = y as f32 /state_obj.config.height as f32 * camera.viewpoint_height as f32;
        self.mouse_position.x_world = self.mouse_position.x_screen + camera.camera_x;
        self.mouse_position.y_world = self.mouse_position.y_screen + camera.camera_y;
    }
    pub fn save_edits(&self) -> Result<(), Box<dyn Error>>{
        self.parser.write("src/game_data/entity_archetypes.json", "src/game_data/entity_attack_patterns.json", "src/game_data/entity_attacks.json", "src/game_data/sprites.json", "src/game_data/starting_level.json")
    }
    pub fn query_stuff_at(&self, x: usize, y: usize) -> QueryResult{
        println!("Query at {}, {}", x, y);
        let chunk_to_query = self.world.get_chunk_from_xy(x, y);
        let mut vec_objects = Vec::new();
        if chunk_to_query.is_none(){
            return QueryResult{
                query_type: QueryType::Position,
                position: Some((x, y)),
                objects: vec_objects
            };
        }
        let chunk_id = chunk_to_query.unwrap();
        let chunk = &self.world.chunks.borrow()[chunk_id];
        for entity_id in chunk.entities_ids.iter(){
            let entity = &self.world.get_entity(*entity_id).unwrap();
            if entity.x <= x as f32 && entity.x + 32.0 >= x as f32 && entity.y <= y as f32 && entity.y + 32.0 >= y as f32 {
                let descriptor = self.object_descriptor_hash.get(entity_id).unwrap().clone();
                vec_objects.push(QueriedObject{element_id: *entity_id, object: descriptor});
            }
        }

        for terrain_id in chunk.terrain_ids.iter(){
            let terrain = self.world.get_terrain(*terrain_id).unwrap();
            if *self.not_real_elements.get(&terrain_id).unwrap_or(&false){
                continue;
            }
            let x_for_terrain = (x as f32 / 32.0).floor() as usize * 32 + 16;
            let y_for_terrain = (y as f32 / 32.0).floor() as usize * 32 + 16;
            if terrain.x <= x_for_terrain && terrain.x + 32 >= x && terrain.y <= y_for_terrain && terrain.y + 32 >= y{
                let descriptor = self.object_descriptor_hash.get(terrain_id).unwrap().clone();
                vec_objects.push(QueriedObject{element_id: *terrain_id, object: descriptor});
            }
        }   
        return QueryResult{
            query_type: QueryType::Position,
            position: Some((x, y)),
            objects: vec_objects
        };
    }
    pub fn update_terrain_property(&mut self, property: EditableProperty, new_value: TerrainPropertyValue){
        match property{
            EditableProperty::TerrainX => {
                let mut nv = 0;
                match new_value{
                    TerrainPropertyValue::X(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                update_terrain_property_json!(self, x, nv, usize); 
            },
            EditableProperty::TerrainY => {
                let mut nv = 0;
                match new_value{
                    TerrainPropertyValue::Y(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                update_terrain_property_json!(self, y, nv, usize);
            },
            EditableProperty::TerrainW => {
                let mut nv = 0;
                match new_value{
                    TerrainPropertyValue::W(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                update_terrain_property_json!(self, width, nv, usize);
            },
            EditableProperty::TerrainH => {
                let mut nv = 0;
                match new_value{
                    TerrainPropertyValue::H(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                update_terrain_property_json!(self, height, nv, usize);
            },
            EditableProperty::TerrainArchetype => {
                let mut nv = String::new();
                match new_value{
                    TerrainPropertyValue::Archetype(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                update_terrain_property_json!(self, terrain_archetype, nv, String);
            },
            _ => {}
        }
    }
    pub fn update_entity_property(&mut self, property: EditableProperty, new_value: EntityPropertyValue){
        match property{
            EditableProperty::EntityX => {
                let mut nv = 0.0;
                match new_value{
                    EntityPropertyValue::X(f) => {
                        nv = f;
                    },
                    _ => {}
                }

                let entity_id = update_entity_property_json!(self, x, nv, f32);
                self.world.entities.borrow_mut().get_mut(&entity_id).unwrap().x = nv;
            },
            EditableProperty::EntityY => {
                let mut nv = 0.0;
                match new_value{
                    EntityPropertyValue::Y(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                let entity_id = update_entity_property_json!(self, y, nv, f32);
                self.world.entities.borrow_mut().get_mut(&entity_id).unwrap().y = nv;
            },
            EditableProperty::EntitySprite => {
                let mut nv = String::new();
                match new_value{
                    EntityPropertyValue::Sprite(f) => {
                        nv = f;
                    },
                    _ => {}
                }
                let entity_id = update_entity_property_json!(self, sprite, nv.clone(), String);
                self.world.set_sprite(entity_id, self.sprites.get_sprite(&nv).expect(format!("Could not find sprite: {}", nv).as_str()));
            },
            EditableProperty::EntityArchetype => {
                let mut nv = String::new();
                match new_value{
                    EntityPropertyValue::Archetype(f) => {
                        nv = f;
                    },
                    _ => {}
                } 
                let entity_id = self.last_query.clone().unwrap().objects[0].element_id;
                let new_archetype_json = self.parser.get_entity_archetype_json(&nv).expect(format!("Could not find archetype {}", nv).as_str());
                let new_archetype_vec = self.parser.convert_archetype(new_archetype_json, &self.parsed_data);
                update_entity_property_json!(self, archetype, nv, String);
                self.world.entity_tags_lookup.insert(entity_id, new_archetype_vec.clone());
            }
            _ => {}
        }
    }
    pub fn check_click_on_menu(mouseX: f32, mouseY: f32) -> bool{
        if mouseX > 932.0 && mouseX < 1132.0 && mouseY > 40.0 && mouseY < 680.0{
            return true;
        }
        return false;
    }
    pub fn key_down(&mut self, event: &InputEvent){
        let key = event.key_down.clone().unwrap_or("".to_string());
        if self.cur_editing.is_some(){
            match key.to_lowercase().as_str(){
                "enter" => {
                    match self.cur_editing.as_ref().unwrap(){
                        EditableProperty::EntityX => {
                            let potential_new_value = self.typed.parse::<f32>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::X(self.typed.parse::<f32>().unwrap()));
                        },
                        EditableProperty::EntityY => {
                            let potential_new_value = self.typed.parse::<f32>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Y(self.typed.parse::<f32>().unwrap()));
                        },
                        EditableProperty::EntitySprite => {
                            if self.sprites.get_sprite(&self.typed).is_none(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Sprite(self.typed.clone()));
                        },
                        EditableProperty::EntityArchetype => {
                            if self.parser.get_entity_archetype_json(&self.typed).is_none(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Archetype(self.typed.clone()));
                            
                        },
                        EditableProperty::TerrainX => {
                            let potential_new_value = self.typed.parse::<usize>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::X(self.typed.parse::<usize>().unwrap()));
                        },
                        EditableProperty::TerrainY => {
                            let potential_new_value = self.typed.parse::<usize>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::Y(self.typed.parse::<usize>().unwrap()));
                        },
                        EditableProperty::TerrainW => {
                            let potential_new_value = self.typed.parse::<usize>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::W(self.typed.parse::<usize>().unwrap()));
                        },
                        EditableProperty::TerrainH => {
                            let potential_new_value = self.typed.parse::<usize>();
                            if potential_new_value.is_err(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::H(self.typed.parse::<usize>().unwrap()));
                        },
                        EditableProperty::TerrainArchetype => {
                            if self.parser.get_terrain_archetype_json(&self.typed).is_none(){
                                self.typed = String::new();
                                self.cur_editing = None;
                                return;
                            }
                            self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::Archetype(self.typed.clone()));
                        },
                        _ => {}
                    }
                    self.typed = String::new();
                    self.cur_editing = None;
                },
                "delete" => {
                    if self.typed.len() > 0{
                        self.typed.pop();
                    }
                },
                _ => {
                    match self.cur_editing.as_ref().unwrap(){
                        EditableProperty::EntityX | EditableProperty::EntityY | EditableProperty::TerrainX | EditableProperty::TerrainY | EditableProperty::TerrainW | EditableProperty::TerrainH  => {
                            if key.chars().all(char::is_numeric) {
                                self.typed.push_str(key.as_str());
                            }
                        },
                        EditableProperty::EntitySprite | EditableProperty::EntityArchetype | EditableProperty::TerrainArchetype => {
                            self.typed.push_str(key.as_str());
                        }
                    }
                    
                }
            }
        }
    }
    pub fn on_click(&mut self, event: &InputEvent, camera: &Camera){
        if event.left_clicked {
            let elements = camera.get_ui_elements_at(self.mouse_position.x_screen as usize, self.mouse_position.y_screen as usize);
            if (!LevelEditor::check_click_on_menu(self.mouse_position.x_screen, self.mouse_position.y_screen)){
                self.last_query = Some(self.query_stuff_at(self.mouse_position.x_world.floor() as usize, self.mouse_position.y_world.floor() as usize));
                self.cur_editing = None;
            }
            for element_name in elements.iter() {
                if element_name.contains("level_editor_query_button_"){
                    let index = element_name.replace("level_editor_query_button_", "").parse::<usize>().unwrap();
                    self.last_query = Some(
                        QueryResult {
                            query_type: QueryType::FollowingObject,
                            position: None,
                            objects: vec![
                                self.last_query.clone().unwrap().objects[index].clone()
                            ]
                        }
                    );
                }
                if element_name.contains("level_editor_query_edit_"){
                    let thing_to_edit = element_name.replace("level_editor_query_edit_", "");
                    match thing_to_edit.as_str() {
                        "entity_x" => self.cur_editing = Some(EditableProperty::EntityX),
                        "entity_y" => self.cur_editing = Some(EditableProperty::EntityY),
                        "entity_sprite" => self.cur_editing = Some(EditableProperty::EntitySprite),
                        "entity_archetype" => self.cur_editing = Some(EditableProperty::EntityArchetype),
                        "terrain_x" => self.cur_editing = Some(EditableProperty::TerrainX),
                        "terrain_y" => self.cur_editing = Some(EditableProperty::TerrainY),
                        "terrain_w" => self.cur_editing = Some(EditableProperty::TerrainW),
                        "terrain_h" => self.cur_editing = Some(EditableProperty::TerrainH),
                        "terrain_archetype" => self.cur_editing = Some(EditableProperty::TerrainArchetype),
                        _ => {}
                    }
                }
                if element_name == "save_button"{
                    match self.save_edits(){
                        Ok(_) => {
                            println!("Saved");
                        },
                        Err(e) => {
                            println!("Error saving: {}", e);
                        }
                    }
                }
            }
        } else if event.right_clicked{
            if self.highlighted.is_some(){
                let terrain_id = self.highlighted.unwrap();
                let terrain = self.world.get_terrain(terrain_id).unwrap();
                let x = terrain.x;
                let y = terrain.y;
                let terrain_json = crate::game_engine::json_parsing::terrain_json {
                    x: x/32,
                    y: y/32,
                    width: 1,
                    height: 1,
                    terrain_archetype: String::from("wall" )
                };
                self.parser.starting_level_json.terrain.push(terrain_json.clone());
                let new_ter_descriptor_id = self.parser.starting_level_json.terrain.len() - 1;
                let new_terrain = self.world.add_terrain(x, y);
                self.object_descriptor_hash.insert(new_terrain, ObjectJSONContainer::Terrain((terrain_json, new_ter_descriptor_id)));
                self.world.set_sprite(new_terrain, self.sprites.get_sprite("wall").unwrap());
            }
        }
    }
    pub fn highlight_square(&mut self){
        if (!LevelEditor::check_click_on_menu(self.mouse_position.x_screen, self.mouse_position.y_screen)){
            let sprite_id = self.sprites.get_sprite("highlight").unwrap();
            let tx = (self.mouse_position.x_world as f32 / 32.0).floor() as usize * 32;
            let ty = (self.mouse_position.y_world as f32 / 32.0).floor() as usize * 32;
            let terrain = self.world.add_terrain(tx, ty);
            self.world.set_sprite(terrain, sprite_id); 
            if self.highlighted.is_some(){
                self.world.remove_terrain(self.highlighted.unwrap());
            }
            self.not_real_elements.insert(terrain, true);
            self.highlighted = Some(terrain);
        } else {
            if self.highlighted.is_some(){
                self.world.remove_terrain(self.highlighted.unwrap());
            }
            self.highlighted = None;
        }
    }
    pub fn add_level_editor_grid(&mut self, sprite_id: usize){
        for x in 0..1000{
            for y in 0..1000{
                self.grid.push(Terrain {
                    element_id: 0,
                    x: x * 32,
                    y: y * 32
                });

            }
        }
    }
    pub fn update_camera_ui(&mut self, camera: &mut Camera){
        if self.query_at_text.is_none() || self.last_query.is_none(){
            return;
        }
        let lq = self.last_query.as_ref().unwrap();
    
        for ui in self.query_unique_ui_elements.clone(){
            camera.remove_ui_element(ui);
        }
        for text in self.query_unique_text_elements.clone(){
            camera.remove_text(text);
        }
        match lq.query_type{
            QueryType::FollowingObject => {
                let queried_object = self.last_query.clone().unwrap().objects[0].clone();
                match queried_object.object.clone(){
                    ObjectJSONContainer::Entity(entity) => {
                        camera.text.get_mut(&self.query_at_text.unwrap()).unwrap().text = format!("Following Entity");
                    },
                    ObjectJSONContainer::Terrain(terrain) => {
                        camera.text.get_mut(&self.query_at_text.unwrap()).unwrap().text = format!("Following Terrain");
                    }
                }
                let (unique_ui, unique_text) = self.display_query_element(camera, queried_object.object);
                self.query_unique_ui_elements.extend(unique_ui);
                self.query_unique_text_elements.extend(unique_text);
            },
            QueryType::Position => {
                camera.text.get_mut(&self.query_at_text.unwrap()).unwrap().text = format!("Query at: {}, {}", lq.position.unwrap().0, lq.position.unwrap().1);
                let mut i = 0;
                for element in self.last_query.clone().unwrap().objects{
                    self.query_unique_ui_elements.push(camera.add_ui_element(format!("level_editor_query_button_{}", i), UIElementDescriptor{
                        x: 942.0 + 45.0 * i as f32,
                        y: 90.0,
                        width: 40.0,
                        height: 15.0,
                        texture_id: self.sprites.get_texture_id("level_editor_button_background").unwrap(),
                        visible: true
                    }));
                    let text: String = match element.object {
                        ObjectJSONContainer::Entity(entity) => {
                            format!("{}. Entity", i + 1)
                        },
                        ObjectJSONContainer::Terrain(terrain) => {
                            format!("{}. Terrain", i + 1)
                        }
                    };
                    self.query_unique_text_elements.push(camera.add_text(text, 962.0 + 45.0 * i as f32, 93.0, 40.0, 18.0, 18.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center));
                    i+= 1;
                }
            }
        }
    }
    pub fn display_query_element(&self, camera: &mut Camera, element: ObjectJSONContainer) -> (Vec<usize>, Vec<usize>){
        let mut unique_ui = Vec::new();
        let mut unique_text = Vec::new();

        let mut edit_button = |name: &str, x: f32, y: f32, unique_ui: &mut Vec<usize>, unique_text: &mut Vec<usize>, camera: &mut Camera| {
            unique_ui.push(
                camera.add_ui_element(format!("level_editor_query_edit_{}",name), UIElementDescriptor {
                    x,
                    y,
                    width: 25.0,
                    height: 9.0,
                    texture_id: self.sprites.get_texture_id("level_editor_button_background").unwrap(),
                    visible: true
                })
            );
            unique_text.push(camera.add_text(String::from("Edit"), x + 11.5, y + 1.0, 25.0, 9.0, 13.0, [1.0, 1.0, 1.0, 1.0], HorizontalAlign::Center));
        };
        let mut potentially_editable_button_display = |label: &str, edit_name: &str, being_edited: bool, value_normal: String, x: f32, y: f32, font_size: f32, unique_ui: &mut Vec<usize>, unique_text: &mut Vec<usize>, camera: &mut Camera| {
            if being_edited{
                unique_text.push(camera.add_text(format!("{}: {}",label,  self.typed), x, y, 200.0, 20.0, font_size, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
            } else {
                unique_text.push(camera.add_text(format!("{}: {}", label, value_normal), x, y, 200.0, 20.0, font_size, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                edit_button(edit_name, x + 1.0, y + font_size/2.0 + 2.0, unique_ui, unique_text, camera);
            }
        };

        match element {
            ObjectJSONContainer::Entity(ej) => {
                let entity = ej.0;
                unique_text.push(camera.add_text(format!("Entity:"), 945.0, 115.0, 50.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                potentially_editable_button_display(
                    "x", "entity_x", self.cur_editing == Some(EditableProperty::EntityX), 
                    entity.x.to_string(), 946.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, camera);

                potentially_editable_button_display(
                    "y", "entity_y", self.cur_editing == Some(EditableProperty::EntityY), 
                    entity.y.to_string(), 1036.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, camera);

                potentially_editable_button_display(
                    "sprite", "entity_sprite", self.cur_editing == Some(EditableProperty::EntitySprite), 
                    entity.sprite.to_string(), 946.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, camera);

                potentially_editable_button_display(
                    "Archetype", "entity_archetype", self.cur_editing == Some(EditableProperty::EntityArchetype), 
                    entity.archetype.to_string(), 945.0, 195.0, 25.0, &mut unique_ui, &mut unique_text, camera);

                let archetype = self.parser.get_entity_archetype_json(&entity.archetype).unwrap();
                unique_text.push(camera.add_text(format!("Monster Type: {}", archetype.monster_type), 945.0, 225.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Range: {}", archetype.range), 1036.0, 225.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Aggro Range: {}", archetype.aggro_range), 945.0, 240.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Movement Speed: {}", archetype.movement_speed), 1036.0, 240.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Attack Type: {}", archetype.attack_type), 945.0, 255.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Attack Pattern: {}", archetype.attack_pattern), 945.0, 270.0, 290.0, 30.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                unique_text.push(camera.add_text(format!("Basic Tags: {:?}", archetype.basic_tags), 945.0, 285.0, 200.0, 30.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
            },
            ObjectJSONContainer::Terrain(tj) => {
                let terrain = tj.0;
                unique_text.push(camera.add_text(format!("Terrain Block:"), 945.0, 115.0, 200.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                potentially_editable_button_display(
                    "x", "terrain_x", self.cur_editing == Some(EditableProperty::TerrainX), 
                    terrain.x.to_string(), 946.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, camera);
                potentially_editable_button_display(
                    "y", "terrain_y", self.cur_editing == Some(EditableProperty::TerrainY), 
                    terrain.y.to_string(), 1036.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, camera);
                potentially_editable_button_display(
                    "w", "terrain_w", self.cur_editing == Some(EditableProperty::TerrainW), 
                    terrain.width.to_string(), 946.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, camera);
                potentially_editable_button_display(
                    "h", "terrain_h", self.cur_editing == Some(EditableProperty::TerrainH), 
                    terrain.height.to_string(), 1036.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, camera);
                let descriptor = self.parsed_data.get_terrain_archetype(&terrain.terrain_archetype).expect(format!("Could not find terrain archetype: {}", terrain.terrain_archetype).as_str()).clone();
                potentially_editable_button_display(
                    "Archetype", "terrain_archetype", self.cur_editing == Some(EditableProperty::TerrainArchetype), 
                    terrain.terrain_archetype.to_string(), 946.0, 190.0, 25.0, &mut unique_ui, &mut unique_text, camera);
                unique_text.push(camera.add_text(format!("Type: {:?}", descriptor.r#type), 946.0, 220.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                if (descriptor.random_chances.is_some()){
                    unique_text.push(camera.add_text(format!("Random Chances: {:?}", descriptor.random_chances.unwrap_or(Vec::new())), 946.0, 235.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                    unique_text.push(camera.add_text(format!("Sprites: {:?}", descriptor.sprites), 946.0, 250.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                    unique_text.push(camera.add_text(format!("Basic Tags: {:?}", descriptor.basic_tags), 946.0, 265.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                }else{
                    unique_text.push(camera.add_text(format!("Sprites: {:?}", descriptor.sprites), 946.0, 235.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                    unique_text.push(camera.add_text(format!("Basic Tags: {:?}", descriptor.basic_tags), 946.0, 250.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
                }
            }
        }
        return (unique_ui, unique_text);
    }
    pub fn process_input(&mut self, state: &InputState, camera: &mut Camera){
        if self.cur_editing.is_some(){
            return;
        }
        let keys = state.keys_down.clone();
        self.world.process_player_input(&keys);
        let mut player = self.world.player.borrow_mut();
        if player.y < 360.0 {
            player.y = 360.0;
        }
        if player.x < 576.0 {
            player.x = 576.0;
        }
        self.mouse_position.x_world = camera.camera_x + self.mouse_position.x_screen;
        self.mouse_position.y_world = camera.camera_y + self.mouse_position.y_screen;
        camera.update_camera_position(&self.world, player.x.floor(), player.y.floor());
    }
}

impl World {
    pub fn set_level_editor(&mut self){
        self.level_editor = true;
        self.player.borrow_mut().movement_speed = 5.0;
    }
}

impl State<'_>{
    pub fn set_level_editor(&mut self){
        self.level_editor = true;
    }
    pub fn level_editor_highlight_square(&mut self, level_editor: &mut LevelEditor){
        level_editor.highlight_square();
    }
    pub fn level_editor_update(&mut self, level_editor: &mut LevelEditor, camera: &mut Camera, state: &InputState){
        level_editor.process_input(state, camera);
        self.level_editor_highlight_square(level_editor);
        level_editor.update_camera_ui(camera);
    }
    pub fn level_editor_render(&mut self, level_editor: &mut LevelEditor, camera: &mut Camera) -> Result<(), wgpu::SurfaceError>{
        let render_data = &camera.level_editor_render(level_editor);
        
        let vertices = &render_data.vertex;
        if vertices.len() < 1 {
            return Ok(());
        }
        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let indicies = &render_data.index;
        let num_indicies = indicies.len() as u32;

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indicies),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let sections = camera.get_sections(self.config.width as f32, self.config.height as f32);
            self.text_brush.queue(&self.device, &self.queue, sections).unwrap();
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[0]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indicies,0, 0..1);
            self.text_brush.draw(&mut render_pass)
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

pub async fn run(mut level_editor: &mut LevelEditor, camera: &mut Camera, sprites_json_to_load: Vec<String>, input_handling_layer: &mut LevelEditorInputLayer) {
    let event_loop = EventLoop::new().unwrap();
    let title = "Level Editor";
    let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
    let mut state_obj = State::new(&window, sprites_json_to_load.clone()).await;
    state_obj.set_level_editor();

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == state_obj.window().id() =>{
            match event {
                WindowEvent::KeyboardInput {  event,.. } => { 
                    let event = event.clone();
                    input_handling_layer.key_event(event, level_editor, camera, &state_obj);
                },
                WindowEvent::CloseRequested => {
                    let response = command_line_input::prompt_string("Save changes? (y/n)");
                    if response.unwrap_or(String::from("n")) == "y"{
                        level_editor.save_edits();
                    }
                    control_flow.exit();
                },
                WindowEvent::Resized(physical_size) => {
                    state_obj.resize(physical_size);
                },
                WindowEvent::CursorMoved {position, ..} => {
                    input_handling_layer.mouse_move(position, level_editor, camera, &state_obj);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    input_handling_layer.mouse_click(state, button, level_editor, camera, &state_obj);
                },
                WindowEvent::RedrawRequested => {
                    state_obj.window().request_redraw();
                    state_obj.level_editor_update(&mut level_editor, camera, &input_handling_layer.state);
                    match state_obj.level_editor_render(&mut level_editor, camera) {
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

