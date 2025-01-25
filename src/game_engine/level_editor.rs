// #![allow(warnings)]
// use std::collections::HashMap;
// use std::error::Error;
// use std::vec;
// use rustc_hash::FxHashMap;
// use wgpu_text::glyph_brush::HorizontalAlign;
// use winit::dpi::PhysicalSize;
// use winit::event::{ElementState, MouseButton, *};
// use winit::event_loop::EventLoop;
// use winit::keyboard::{Key, NamedKey};
// use winit::window::{Window, WindowBuilder};
// use super::game::{InputState, MousePosition};
// use super::json_parsing::{entity_json, terrain_json, JSON_parser, ParsedData};
// use super::starting_level_generator::match_terrain_tags;
// use super::terrain::Terrain;
// use super::ui::UIElementDescriptor;
// use super::world::World;
// use super::camera::Camera;use crate::rendering_engine::abstractions::{RenderData, RenderDataFull, SpriteContainer};
// use crate::rendering_engine::renderer::Renderer;
// use super::player::Player;
// use super::json_parsing::PATH_BUNDLE;
// #[derive(Debug, Clone, PartialEq)]
// pub enum EditableProperty{
//     EntityX,
//     EntityY,
//     EntityArchetype,
//     EntitySprite,
//     TerrainX,
//     TerrainY,
//     TerrainW,
//     TerrainH,
//     TerrainArchetype
// }
// #[derive(Debug, Clone, PartialEq)]
// pub enum EntityPropertyValue{
//     X(f32),
//     Y(f32),
//     Archetype(String),
//     Sprite(String)
// }
// #[derive(Debug, Clone, PartialEq)]
// pub enum TerrainPropertyValue{
//     X(usize),
//     Y(usize),
//     W(usize),
//     H(usize),
//     Archetype(String),
// }


// #[derive(Debug, Clone, PartialEq)]
// pub enum QueryType{
//     Position,
//     FollowingObject
// }
// #[derive(Debug, Clone)]
// pub struct QueryResult{
//     pub query_type: QueryType,
//     pub position: Option<(usize, usize)>,
//     pub objects: Vec<QueriedObject>,
// }
// #[derive(Debug, Clone)]
// pub struct QueriedObject{
//     pub object: ObjectJSONContainer,
//     pub element_id: usize
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum ObjectJSONContainer { // usize here is the index in the starting_level_json entity/json
//     Entity((entity_json, usize)),
//     Terrain((terrain_json, usize))
// }

// macro_rules! update_entity_property_json {
//     ($self:ident, $property:ident, $new_value:expr, $type:ty) => {{
//         let new_property: $type = $new_value;
//         let last_query = $self.last_query.clone().unwrap();
//         if last_query.query_type != QueryType::FollowingObject{
//             return;
//         }
//         let entity_id = last_query.objects[0].element_id;
//         let object = last_query.objects[0].object.clone();
//         let mut new_object = object.clone();
//         match &mut new_object {
//             ObjectJSONContainer::Entity(ref mut obj) => {
//                 obj.0.$property = new_property.clone();
//             }
//             _ => {}
//         }
//         $self.last_query = Some(
//             QueryResult {
//                 query_type: QueryType::FollowingObject,
//                 position: None,
//                 objects: vec![
//                     QueriedObject{
//                         element_id: entity_id,
//                         object: new_object.clone()
//                     }]
//             }
//         );
//         match object {
//             ObjectJSONContainer::Entity(mut obj) => {
//                 $self.parser.starting_level_json.entities[obj.1].$property = new_property.clone();
//             },
//             _ => {}
//         }
//         match $self.object_descriptor_hash.get_mut(&entity_id).unwrap() {
//             ObjectJSONContainer::Entity(entity) => {
//                 entity.0.$property = new_property.clone();
//             },
//             _ => {}
//         }
//         entity_id
//     }
// }}
// macro_rules! update_terrain_property_json {
//     ($self:ident, $property:ident, $new_value:expr, $type:ty) => {{
//         let new_property: $type = $new_value;
//         let last_query = $self.last_query.clone().unwrap();
//         if last_query.query_type != QueryType::FollowingObject{
//             return;
//         }
//         let mut terrain_object = last_query.objects[0].clone();
//         match terrain_object.object.clone(){
//             ObjectJSONContainer::Terrain(terrain) => {
//                 let parser_id = terrain.1;
//                 $self.parser.starting_level_json.terrain[parser_id].$property = new_property.clone();
//             }
//             _ => {}
//         }

//         match &mut terrain_object.object{
//             ObjectJSONContainer::Terrain(ref mut terrain) => {
//                 terrain.0.$property = new_property.clone();
//             }
//             _ => {}
//         }
//         let (world, sprites, hash) = level_editor_generate_world_from_json_parsed_data(&$self.parser.convert());
//         $self.object_descriptor_hash = hash;
//         let player_x = $self.world.player.borrow().x.floor();
//         let player_y = $self.world.player.borrow().y.floor();
//         $self.world = world;
//         $self.sprites = sprites;
//         $self.last_query = Some(QueryResult{
//             query_type: QueryType::FollowingObject,
//             position: None,
//             objects: vec![terrain_object]
//         });
//         $self.world.set_level_editor();
//         $self.world.player.borrow_mut().x = player_x;
//         $self.world.player.borrow_mut().y = player_y;
//     }
// }}

// pub struct LevelEditor<'a>{
//     pub highlighted: Option<usize>,
//     pub grid: Vec<Terrain>,
//     pub grid_sprite: Option<usize>,
//     pub parser: JSON_parser,
//     pub parsed_data: ParsedData,
//     pub world: World,
//     pub sprites: SpriteContainer,
//     pub not_real_elements: HashMap<usize, bool>,
//     pub object_descriptor_hash: HashMap<usize, ObjectJSONContainer>,
//     pub query_at_text: Option<usize>,
//     pub last_query: Option<QueryResult>,
//     pub cur_editing: Option<EditableProperty>,
//     pub typed: String,
//     pub query_unique_ui_elements: Vec<usize>,
//     pub query_unique_text_elements: Vec<usize>,
//     pub camera: Camera,
//     pub input: InputState,
//     pub renderer: Renderer<'a>,
// }
// impl<'a> LevelEditor<'a>{
//     pub fn new(world: World, renderer: Renderer<'a>, camera: Camera, sprites: SpriteContainer, parser: JSON_parser, hash: HashMap<usize, ObjectJSONContainer>) -> Self{
//         let parsed_data = parser.clone().convert();
//         Self {
//             highlighted: None,
//             world,
//             parser,
//             grid: Vec::new(),
//             grid_sprite: None,
//             camera,
//             parsed_data,
//             sprites,
//             not_real_elements: HashMap::new(),
//             object_descriptor_hash: hash,
//             last_query: None,
//             query_at_text: None,
//             typed: String::new(),
//             cur_editing: None,
//             query_unique_ui_elements: Vec::new(),
//             query_unique_text_elements: Vec::new(),
//             renderer,
//             input: InputState {
//                 keys_down: FxHashMap::default(),
//                 mouse_position: MousePosition::default(),
//                 mouse_left: false,
//                 mouse_right: false,
//             }
//         }
//     }
//     pub fn init(&mut self){
//         self.world.set_level_editor();
//         self.add_level_editor_grid(self.sprites.get_sprite_id("grid").unwrap());
//         self.grid_sprite = Some(self.sprites.get_sprite_id("grid").unwrap());
//         self.camera.add_ui_element("menu".to_string(), super::ui::UIElementDescriptor {
//             x: 932.0,
//             y: 40.0,
//             z: 1.0,
//             width: 200.0,
//             height: 640.0,
//             sprite: "level_editor_menu_background".to_string(),
//             visible: true
//         });
//         self.camera.add_text("Level Editor".to_string(), super::camera::Font::A,938.0, 45.0,60.0, 24.5, 24.5, [0.7,0.7,0.7,1.0], HorizontalAlign::Left);
//         self.camera.add_ui_element("save_button".to_string(), super::ui::UIElementDescriptor {
//             x: 1008.0,
//             y: 45.0,
//             width: 40.0,
//             z: 1.0,
//             height: 12.5,
//             sprite: "level_editor_button_background".to_string(),
//             visible: true
//         });
//         self.camera.add_text("Save".to_string(),super::camera::Font::A, 1028.0, 45.0,60.0, 22.0, 22.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center);
//         self.query_at_text = Some(self.camera.add_text("".to_string(),super::camera::Font::A, 942.0, 70.0,180.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//     }
//     pub fn save_edits(&self) -> Result<(), Box<dyn Error>>{
//         self.parser.write(PATH_BUNDLE)
//     }
//     pub fn query_stuff_at(&self, x: usize, y: usize) -> QueryResult{
//         println!("Query at {}, {}", x, y);
//         let chunk_to_query = self.world.get_chunk_from_xy(x, y);
//         let mut vec_objects = Vec::new();
//         if chunk_to_query.is_none(){
//             return QueryResult{
//                 query_type: QueryType::Position,
//                 position: Some((x, y)),
//                 objects: vec_objects
//             };
//         }
//         let chunk_id = chunk_to_query.unwrap();
//         let chunk = &self.world.chunks.borrow()[chunk_id];
//         for entity_id in chunk.entities_ids.iter(){
//             let position_component = self.world.entity_position_components.get(entity_id).expect("All entities should have a position component").borrow();
//             if position_component.x <= x as f32 && position_component.x + 32.0 >= x as f32 && position_component.y <= y as f32 && position_component.y + 32.0 >= y as f32 {
//                 let descriptor = self.object_descriptor_hash.get(entity_id).unwrap().clone();
//                 vec_objects.push(QueriedObject{element_id: *entity_id, object: descriptor});
//             }
//         }

//         for terrain_id in chunk.terrain_ids.iter(){
//             let terrain = self.world.get_terrain(*terrain_id).unwrap();
//             if *self.not_real_elements.get(&terrain_id).unwrap_or(&false){
//                 continue;
//             }
//             let x_for_terrain = (x as f32 / 32.0).floor() as usize * 32 + 16;
//             let y_for_terrain = (y as f32 / 32.0).floor() as usize * 32 + 16;
//             if terrain.x <= x_for_terrain && terrain.x + 32 >= x && terrain.y <= y_for_terrain && terrain.y + 32 >= y{
//                 let descriptor = self.object_descriptor_hash.get(terrain_id).unwrap().clone();
//                 vec_objects.push(QueriedObject{element_id: *terrain_id, object: descriptor});
//             }
//         }   
//         return QueryResult{
//             query_type: QueryType::Position,
//             position: Some((x, y)),
//             objects: vec_objects
//         };
//     }
//     pub fn update_terrain_property(&mut self, property: EditableProperty, new_value: TerrainPropertyValue){
//         match property{
//             EditableProperty::TerrainX => {
//                 let mut nv = 0;
//                 match new_value{
//                     TerrainPropertyValue::X(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 update_terrain_property_json!(self, x, nv, usize); 
//             },
//             EditableProperty::TerrainY => {
//                 let mut nv = 0;
//                 match new_value{
//                     TerrainPropertyValue::Y(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 update_terrain_property_json!(self, y, nv, usize);
//             },
//             EditableProperty::TerrainW => {
//                 let mut nv = 0;
//                 match new_value{
//                     TerrainPropertyValue::W(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 update_terrain_property_json!(self, width, nv, usize);
//             },
//             EditableProperty::TerrainH => {
//                 let mut nv = 0;
//                 match new_value{
//                     TerrainPropertyValue::H(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 update_terrain_property_json!(self, height, nv, usize);
//             },
//             EditableProperty::TerrainArchetype => {
//                 let mut nv = String::new();
//                 match new_value{
//                     TerrainPropertyValue::Archetype(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 update_terrain_property_json!(self, terrain_archetype, nv, String);
//             },
//             _ => {}
//         }
//     }
//     pub fn update_entity_property(&mut self, property: EditableProperty, new_value: EntityPropertyValue){
//         match property{
//             EditableProperty::EntityX => {
//                 let mut nv = 0.0;
//                 match new_value{
//                     EntityPropertyValue::X(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }

//                 let entity_id = update_entity_property_json!(self, x, nv, f32);
//                 self.world.entity_position_components.get(&entity_id).expect("All Entities should have a position component").borrow_mut().x = nv;
//             },
//             EditableProperty::EntityY => {
//                 let mut nv = 0.0;
//                 match new_value{
//                     EntityPropertyValue::Y(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 let entity_id = update_entity_property_json!(self, y, nv, f32);
//                 self.world.entity_position_components.get(&entity_id).expect("All Entities should have a position component").borrow_mut().y = nv;
//             },
//             EditableProperty::EntitySprite => {
//                 let mut nv = String::new();
//                 match new_value{
//                     EntityPropertyValue::Sprite(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 }
//                 let entity_id = update_entity_property_json!(self, sprite, nv.clone(), String);
//                 self.world.set_sprite(entity_id, self.sprites.get_sprite_id(&nv).expect(format!("Could not find sprite: {}", nv).as_str()));
//             },
//             EditableProperty::EntityArchetype => {
//                 let mut nv = String::new();
//                 match new_value{
//                     EntityPropertyValue::Archetype(f) => {
//                         nv = f;
//                     },
//                     _ => {}
//                 } 
//                 let entity_id = self.last_query.clone().unwrap().objects[0].element_id;
//                 update_entity_property_json!(self, archetype, nv.clone(), String);
//                 self.world.entity_archetype_lookup.insert(entity_id, nv);
//             }
//             _ => {}
//         }
//     }
//     pub fn check_click_on_menu(mouseX: f32, mouseY: f32) -> bool{
//         if mouseX > 932.0 && mouseX < 1132.0 && mouseY > 40.0 && mouseY < 680.0{
//             return true;
//         }
//         return false;
//     }
//     pub fn process_key_input(&mut self, event: KeyEvent){
//         let mut key = event.logical_key.to_text();
//         match event.logical_key{
//             Key::Named(NamedKey::Enter) => {
//                 key = Some("Enter");
//             },
//             Key::Named(NamedKey::Backspace) => {
//                 key = Some("Delete");
//             },
//             _ => {}
//         }
//         if key.is_none(){
//             return;
//         }
//         let string_key = key.unwrap().to_string().to_lowercase();
//         let press = match event.state {
//             ElementState::Pressed => true,
//             ElementState::Released => false,
//         };
//         self.input.keys_down.insert(string_key.clone(), press);
//         if press {
//             self.key_down(string_key.clone());
//         }
//     }
//     pub fn key_down(&mut self, key: String){
//         if self.cur_editing.is_some(){
//             match key.to_lowercase().as_str(){
//                 "enter" => {
//                     match self.cur_editing.as_ref().unwrap(){
//                         EditableProperty::EntityX => {
//                             let potential_new_value = self.typed.parse::<f32>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::X(self.typed.parse::<f32>().unwrap()));
//                         },
//                         EditableProperty::EntityY => {
//                             let potential_new_value = self.typed.parse::<f32>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Y(self.typed.parse::<f32>().unwrap()));
//                         },
//                         EditableProperty::EntitySprite => {
//                             if self.sprites.get_sprite_id(&self.typed).is_none(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Sprite(self.typed.clone()));
//                         },
//                         EditableProperty::EntityArchetype => {
//                             if self.parser.get_entity_archetype_json(&self.typed).is_none(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_entity_property(self.cur_editing.clone().unwrap(), EntityPropertyValue::Archetype(self.typed.clone()));
                            
//                         },
//                         EditableProperty::TerrainX => {
//                             let potential_new_value = self.typed.parse::<usize>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::X(self.typed.parse::<usize>().unwrap()));
//                         },
//                         EditableProperty::TerrainY => {
//                             let potential_new_value = self.typed.parse::<usize>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::Y(self.typed.parse::<usize>().unwrap()));
//                         },
//                         EditableProperty::TerrainW => {
//                             let potential_new_value = self.typed.parse::<usize>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::W(self.typed.parse::<usize>().unwrap()));
//                         },
//                         EditableProperty::TerrainH => {
//                             let potential_new_value = self.typed.parse::<usize>();
//                             if potential_new_value.is_err(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::H(self.typed.parse::<usize>().unwrap()));
//                         },
//                         EditableProperty::TerrainArchetype => {
//                             if self.parser.get_terrain_archetype_json(&self.typed).is_none(){
//                                 self.typed = String::new();
//                                 self.cur_editing = None;
//                                 return;
//                             }
//                             self.update_terrain_property(self.cur_editing.clone().unwrap(), TerrainPropertyValue::Archetype(self.typed.clone()));
//                         },
//                         _ => {}
//                     }
//                     self.typed = String::new();
//                     self.cur_editing = None;
//                 },
//                 "delete" => {
//                     if self.typed.len() > 0{
//                         self.typed.pop();
//                     }
//                 },
//                 _ => {
//                     match self.cur_editing.as_ref().unwrap(){
//                         EditableProperty::EntityX | EditableProperty::EntityY | EditableProperty::TerrainX | EditableProperty::TerrainY | EditableProperty::TerrainW | EditableProperty::TerrainH  => {
//                             if key.chars().all(char::is_numeric) {
//                                 self.typed.push_str(key.as_str());
//                             }
//                         },
//                         EditableProperty::EntitySprite | EditableProperty::EntityArchetype | EditableProperty::TerrainArchetype => {
//                             self.typed.push_str(key.as_str());
//                         }
//                     }
                    
//                 }
//             }
//         }
//     }
//     pub fn key_update(&mut self){
//         let keys = self.input.keys_down.clone();
//         if self.cur_editing.is_some(){
//             return;
//         }
//         self.world.process_player_input(&keys);
//         let mut player = self.world.player.borrow_mut();
//         if player.y < 360.0 {
//             player.y = 360.0;
//         }
//         if player.x < 576.0 {
//             player.x = 576.0;
//         }
//         self.input.mouse_position.x_world = self.camera.camera_x + self.input.mouse_position.x_screen;
//         self.input.mouse_position.y_world = self.camera.camera_y + self.input.mouse_position.y_screen;
//         self.camera.update_camera_position(player.x.floor(), player.y.floor());
//     }
//     pub fn on_click(&mut self, mouse: MouseButton){
//         if mouse == MouseButton::Left {
//             let elements = self.camera.get_ui_elements_at(self.input.mouse_position.x_screen as usize, self.input.mouse_position.y_screen as usize);
//             if (!LevelEditor::check_click_on_menu(self.input.mouse_position.x_screen, self.input.mouse_position.y_screen)){
//                 self.last_query = Some(self.query_stuff_at(self.input.mouse_position.x_world.floor() as usize, self.input.mouse_position.y_world.floor() as usize));
//                 self.cur_editing = None;
//             }
//             for element_name in elements.iter() {
//                 if element_name.contains("level_editor_query_button_"){
//                     let index = element_name.replace("level_editor_query_button_", "").parse::<usize>().unwrap();
//                     self.last_query = Some(
//                         QueryResult {
//                             query_type: QueryType::FollowingObject,
//                             position: None,
//                             objects: vec![
//                                 self.last_query.clone().unwrap().objects[index].clone()
//                             ]
//                         }
//                     );
//                 }
//                 if element_name.contains("level_editor_query_edit_"){
//                     let thing_to_edit = element_name.replace("level_editor_query_edit_", "");
//                     match thing_to_edit.as_str() {
//                         "entity_x" => self.cur_editing = Some(EditableProperty::EntityX),
//                         "entity_y" => self.cur_editing = Some(EditableProperty::EntityY),
//                         "entity_sprite" => self.cur_editing = Some(EditableProperty::EntitySprite),
//                         "entity_archetype" => self.cur_editing = Some(EditableProperty::EntityArchetype),
//                         "terrain_x" => self.cur_editing = Some(EditableProperty::TerrainX),
//                         "terrain_y" => self.cur_editing = Some(EditableProperty::TerrainY),
//                         "terrain_w" => self.cur_editing = Some(EditableProperty::TerrainW),
//                         "terrain_h" => self.cur_editing = Some(EditableProperty::TerrainH),
//                         "terrain_archetype" => self.cur_editing = Some(EditableProperty::TerrainArchetype),
//                         _ => {}
//                     }
//                 }
//                 if element_name == "save_button"{
//                     match self.save_edits(){
//                         Ok(_) => {
//                             println!("Saved");
//                         },
//                         Err(e) => {
//                             println!("Error saving: {}", e);
//                         }
//                     }
//                 }
//             }
//         } else if mouse == MouseButton::Right{
//             if self.highlighted.is_some(){
//                 let terrain_id = self.highlighted.unwrap();
//                 let terrain = self.world.get_terrain(terrain_id).unwrap();
//                 let x = terrain.x;
//                 let y = terrain.y;
//                 let terrain_json = super::json_parsing::terrain_json {
//                     x: x/32,
//                     y: y/32,
//                     width: 1,
//                     height: 1,
//                     terrain_archetype: String::from("wall" )
//                 };
//                 self.parser.starting_level_json.terrain.push(terrain_json.clone());
//                 let new_ter_descriptor_id = self.parser.starting_level_json.terrain.len() - 1;
//                 let new_terrain = self.world.add_terrain(x, y);
//                 self.object_descriptor_hash.insert(new_terrain, ObjectJSONContainer::Terrain((terrain_json, new_ter_descriptor_id)));
//                 self.world.set_sprite(new_terrain, self.sprites.get_sprite_id("wall").unwrap());
//             }
//         }
//     }
//     pub fn highlight_square(&mut self){
//         if (!LevelEditor::check_click_on_menu(self.input.mouse_position.x_screen, self.input.mouse_position.y_screen)){
//             let sprite_id = self.sprites.get_sprite_id("highlight").unwrap();
//             let tx = (self.input.mouse_position.x_world as f32 / 32.0).floor() as usize * 32;
//             let ty = (self.input.mouse_position.y_world as f32 / 32.0).floor() as usize * 32;
//             let terrain = self.world.add_terrain(tx, ty);
//             self.world.set_sprite(terrain, sprite_id); 
//             if self.highlighted.is_some(){
//                 self.world.remove_terrain(self.highlighted.unwrap());
//             }
//             self.not_real_elements.insert(terrain, true);
//             self.highlighted = Some(terrain);
//         } else {
//             if self.highlighted.is_some(){
//                 self.world.remove_terrain(self.highlighted.unwrap());
//             }
//             self.highlighted = None;
//         }
//     }
//     pub fn add_level_editor_grid(&mut self, sprite_id: usize){
//         for x in 0..1000{
//             for y in 0..1000{
//                 self.grid.push(Terrain {
//                     element_id: 0,
//                     x: x * 32,
//                     y: y * 32
//                 });

//             }
//         }
//     }
//     pub fn update_ui(&mut self){
//         if self.query_at_text.is_none() || self.last_query.is_none(){
//             return;
//         }
//         let lq = self.last_query.as_ref().unwrap();
    
//         for ui in self.query_unique_ui_elements.clone(){
//             self.camera.remove_ui_element(ui);
//         }
//         for text in self.query_unique_text_elements.clone(){
//             self.camera.remove_text(text);
//         }
//         match lq.query_type{
//             QueryType::FollowingObject => {
//                 let queried_object = self.last_query.clone().unwrap().objects[0].clone();
//                 match queried_object.object.clone(){
//                     ObjectJSONContainer::Entity(entity) => {
//                         self.camera.get_text_mut(self.query_at_text.unwrap()).unwrap().text = format!("Following Entity");
//                     },
//                     ObjectJSONContainer::Terrain(terrain) => {
//                         self.camera.get_text_mut(self.query_at_text.unwrap()).unwrap().text = format!("Following Terrain");
//                     }
//                 }
//                 let (unique_ui, unique_text) = self.display_query_element(queried_object.object);
//                 self.query_unique_ui_elements.extend(unique_ui);
//                 self.query_unique_text_elements.extend(unique_text);
//             },
//             QueryType::Position => {
//                 self.camera.get_text_mut(self.query_at_text.unwrap()).unwrap().text = format!("Query at: {}, {}", lq.position.unwrap().0, lq.position.unwrap().1);
//                 let mut i = 0;
//                 for element in self.last_query.clone().unwrap().objects{
//                     self.query_unique_ui_elements.push(self.camera.add_ui_element(format!("level_editor_query_button_{}", i), UIElementDescriptor{
//                         x: 942.0 + 45.0 * i as f32,
//                         y: 90.0,
//                         width: 40.0,
//                         height: 15.0,
//                         z: 1.0,
//                         sprite: "level_editor_button_background".to_string(),
//                         visible: true
//                     }));
//                     let text: String = match element.object {
//                         ObjectJSONContainer::Entity(entity) => {
//                             format!("{}. Entity", i + 1)
//                         },
//                         ObjectJSONContainer::Terrain(terrain) => {
//                             format!("{}. Terrain", i + 1)
//                         }
//                     };
//                     self.query_unique_text_elements.push(self.camera.add_text(text, super::camera::Font::A, 962.0 + 45.0 * i as f32, 93.0, 40.0, 18.0, 18.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Center));
//                     i+= 1;
//                 }
//             }
//         }
//     }
//     pub fn display_query_element(&mut self, element: ObjectJSONContainer) -> (Vec<usize>, Vec<usize>){
//         let mut unique_ui = Vec::new();
//         let mut unique_text = Vec::new();

//         let mut edit_button = |name: &str, x: f32, y: f32, unique_ui: &mut Vec<usize>, unique_text: &mut Vec<usize>, camera: &mut Camera| {
//             unique_ui.push(
//                 camera.add_ui_element(format!("level_editor_query_edit_{}",name), UIElementDescriptor {
//                     x,
//                     y,
//                     width: 25.0,
//                     height: 9.0,
//                     sprite: "level_editor_button_background".to_string(),
//                     z: 1.0,
//                     visible: true
//                 })
//             );
//             unique_text.push(camera.add_text(String::from("Edit"), super::camera::Font::A, x + 11.5, y + 1.0, 25.0, 9.0, 13.0, [1.0, 1.0, 1.0, 1.0], HorizontalAlign::Center));
//         };
//         let mut potentially_editable_button_display = |label: &str, edit_name: &str, being_edited: bool, value_normal: String, x: f32, y: f32, font_size: f32, unique_ui: &mut Vec<usize>, unique_text: &mut Vec<usize>, camera: &mut Camera| {
//             if being_edited{
//                 unique_text.push(camera.add_text(format!("{}: {}",label,  self.typed),super::camera::Font::A, x, y, 200.0, 20.0, font_size, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//             } else {
//                 unique_text.push(camera.add_text(format!("{}: {}", label, value_normal), super::camera::Font::A, x, y, 200.0, 20.0, font_size, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 edit_button(edit_name, x + 1.0, y + font_size/2.0 + 2.0, unique_ui, unique_text, camera);
//             }
//         };

//         match element {
//             ObjectJSONContainer::Entity(ej) => {
//                 let entity = ej.0;
//                 unique_text.push(self.camera.add_text(format!("Entity:"),super::camera::Font::A, 945.0, 115.0, 50.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 potentially_editable_button_display(
//                     "x", "entity_x", self.cur_editing == Some(EditableProperty::EntityX), 
//                     entity.x.to_string(), 946.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);

//                 potentially_editable_button_display(
//                     "y", "entity_y", self.cur_editing == Some(EditableProperty::EntityY), 
//                     entity.y.to_string(), 1036.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);

//                 potentially_editable_button_display(
//                     "sprite", "entity_sprite", self.cur_editing == Some(EditableProperty::EntitySprite), 
//                     entity.sprite.to_string(), 946.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);

//                 potentially_editable_button_display(
//                     "Archetype", "entity_archetype", self.cur_editing == Some(EditableProperty::EntityArchetype), 
//                     entity.archetype.to_string(), 945.0, 195.0, 25.0, &mut unique_ui, &mut unique_text, &mut self.camera);

//                 let archetype = self.parser.get_entity_archetype_json(&entity.archetype).unwrap();
//                 unique_text.push(self.camera.add_text(format!("Monster Type: {}", archetype.monster_type), super::camera::Font::A, 945.0, 225.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Range: {}", archetype.range), super::camera::Font::A, 1036.0, 225.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Aggro Range: {}", archetype.aggro_range), super::camera::Font::A, 945.0, 240.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Movement Speed: {}", archetype.movement_speed), super::camera::Font::A, 1036.0, 240.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Attack Type: {}", archetype.attack_type), super::camera::Font::A, 945.0, 255.0, 100.0, 15.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Attack Pattern: {}", archetype.attack_pattern), super::camera::Font::A, 945.0, 270.0, 290.0, 30.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 unique_text.push(self.camera.add_text(format!("Basic Tags: {:?}", archetype.basic_tags), super::camera::Font::A, 945.0, 285.0, 200.0, 30.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//             },
//             ObjectJSONContainer::Terrain(tj) => {
//                 let terrain = tj.0;
//                 unique_text.push(self.camera.add_text(format!("Terrain Block:"), super::camera::Font::A, 945.0, 115.0, 200.0, 25.0, 25.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 potentially_editable_button_display(
//                     "x", "terrain_x", self.cur_editing == Some(EditableProperty::TerrainX), 
//                     terrain.x.to_string(), 946.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);
//                 potentially_editable_button_display(
//                     "y", "terrain_y", self.cur_editing == Some(EditableProperty::TerrainY), 
//                     terrain.y.to_string(), 1036.0, 140.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);
//                 potentially_editable_button_display(
//                     "w", "terrain_w", self.cur_editing == Some(EditableProperty::TerrainW), 
//                     terrain.width.to_string(), 946.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);
//                 potentially_editable_button_display(
//                     "h", "terrain_h", self.cur_editing == Some(EditableProperty::TerrainH), 
//                     terrain.height.to_string(), 1036.0, 165.0, 20.0, &mut unique_ui, &mut unique_text, &mut self.camera);
//                 let descriptor = self.parsed_data.get_terrain_archetype(&terrain.terrain_archetype).expect(format!("Could not find terrain archetype: {}", terrain.terrain_archetype).as_str()).clone();
//                 potentially_editable_button_display(
//                     "Archetype", "terrain_archetype", self.cur_editing == Some(EditableProperty::TerrainArchetype), 
//                     terrain.terrain_archetype.to_string(), 946.0, 190.0, 25.0, &mut unique_ui, &mut unique_text, &mut self.camera);
//                 unique_text.push(self.camera.add_text(format!("Type: {:?}", descriptor.r#type), super::camera::Font::A, 946.0, 220.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 if (descriptor.random_chances.is_some()){
//                     unique_text.push(self.camera.add_text(format!("Random Chances: {:?}", descriptor.random_chances.unwrap_or(Vec::new())), super::camera::Font::A, 946.0, 235.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                     unique_text.push(self.camera.add_text(format!("Sprites: {:?}", descriptor.sprites), super::camera::Font::A, 946.0, 250.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                     unique_text.push(self.camera.add_text(format!("Basic Tags: {:?}", descriptor.basic_tags), super::camera::Font::A, 946.0, 265.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 }else{
//                     unique_text.push(self.camera.add_text(format!("Sprites: {:?}", descriptor.sprites), super::camera::Font::A, 946.0, 235.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                     unique_text.push(self.camera.add_text(format!("Basic Tags: {:?}", descriptor.basic_tags), super::camera::Font::A, 946.0, 250.0, 200.0, 20.0, 20.0, [1.0,1.0,1.0,1.0], HorizontalAlign::Left));
//                 }
//             }
//         }
//         return (unique_ui, unique_text);
//     }
//     pub fn process_mouse_position(&mut self, x: f64, y: f64){
//         self.input.mouse_position.x_screen = x as f32/self.renderer.config.width as f32 * self.camera.viewpoint_width as f32;
//         self.input.mouse_position.y_screen = y as f32 /self.renderer.config.height as f32 * self.camera.viewpoint_height as f32;
//         self.input.mouse_position.x_world = self.input.mouse_position.x_screen + self.camera.camera_x;
//         self.input.mouse_position.y_world = self.input.mouse_position.y_screen + self.camera.camera_y;
//     }
//     pub fn process_mouse_input(&mut self, state: ElementState, button: MouseButton){
//         if state == ElementState::Pressed && button == MouseButton::Left{
//             self.input.mouse_left = true;
//         } else if state == ElementState::Released && button == MouseButton::Left{
//             self.input.mouse_left = false;
//         } else if state == ElementState::Pressed && button == MouseButton::Right{
//             self.input.mouse_right = true;
//         } else if state == ElementState::Released && button == MouseButton::Right{
//             self.input.mouse_right = false;
//         }
//         if state == ElementState::Pressed{
//             self.on_click(button);
//         } 
//     }
//     pub fn update(&mut self){
//         self.key_update();
//         self.highlight_square();
//         self.update_ui();
//     }
//     pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
//         self.renderer.render(self.camera.level_editor_render(&mut self.world, &self.sprites, &self.grid, self.renderer.config.width as f32, self.renderer.config.height as f32))
//     }
//     pub fn resize(&mut self, size: PhysicalSize<u32>){
//         self.renderer.resize(size);
//     }
//     pub fn window(&self) -> &Window{
//         self.renderer.window()
//     }
// }

// impl World {
//     pub fn set_level_editor(&mut self){
//         self.level_editor = true;
//         self.player.borrow_mut().movement_speed = 5.0;
//     }
// }

// impl Camera{
//     pub fn set_level_editor(&mut self){
//         self.level_editor = true;
//     }
//     pub fn level_editor_render(&mut self, world: &mut World, sprites: &SpriteContainer, grid: &Vec<Terrain>, screen_width: f32, screen_height: f32) -> RenderDataFull{
//         let mut render_data = RenderDataFull::new();
//         let mut terrain_data: RenderData = RenderData::new();
//         let mut entity_data: RenderData = RenderData::new();
//         let mut index_offset: u32 = 0;
//         let player = world.player.borrow(); 


//         let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
//         let camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize) + 1;

//         let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
//         let camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize) + 1; 

//         let mut chunks_loaded = Vec::new();
//         for x in camera_left_chunk_x..camera_right_chunk_x{
//             for y in camera_top_chunk_y..camera_bot_chunk_y{
                
//                 let chunk_id = world.get_chunk_from_chunk_xy(x,y);
                
//                 if chunk_id.is_none(){
//                     continue;
//                 }
//                 let chunk = &world.chunks.borrow()[chunk_id.unwrap()];

//                 chunks_loaded.push(chunk_id.unwrap());
//                 for terrain_id in chunk.terrain_ids.iter(){
//                     let potentially_sprite_id = world.get_sprite(*terrain_id);
//                     if potentially_sprite_id.is_none(){
//                         continue;
//                     }
//                     let sprite_id = potentially_sprite_id.unwrap();
//                     let sprite = &world.sprites.get_sprite(sprite_id).unwrap();

                    
//                     let vertex_offset_x = -1 * self.camera_x as i32;
//                     let vertex_offset_y = -1 * self.camera_y as i32;
//                     let terrain = world.get_terrain(*terrain_id).unwrap();
//                     let draw_data = sprite.draw_data(terrain.x as f32, terrain.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
//                     index_offset += 4;
//                     terrain_data.vertex.extend(draw_data.vertex);
//                     terrain_data.index.extend(draw_data.index);
//                 }

//                 for entity_id in chunk.entities_ids.iter(){
//                     let potentially_sprite_id = world.get_sprite(*entity_id);
//                     if potentially_sprite_id.is_none(){
//                         continue;
//                     }
//                     let sprite_id = potentially_sprite_id.unwrap();
//                     let sprite = &world.sprites.get_sprite(sprite_id).unwrap();
                    
//                     let vertex_offset_x = -1 * self.camera_x as i32;
//                     let vertex_offset_y = -1 * self.camera_y as i32;

//                     let position_component = world.entity_position_components.get(entity_id).expect("All entities with sprites should have a position component").borrow();

//                     let draw_data = sprite.draw_data(position_component.x, position_component.y, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
//                     index_offset += 4;
//                     entity_data.vertex.extend(draw_data.vertex);
//                     entity_data.index.extend(draw_data.index);
//                 }
//             }
//         }
//         render_data.vertex.extend(terrain_data.vertex);
//         render_data.vertex.extend(entity_data.vertex);
//         render_data.index.extend(terrain_data.index);
//         render_data.index.extend(entity_data.index);

//         for (id, element) in self.ui_elements.iter(){
//             if !element.visible{
//                 continue;
//             }
//             let index_offset = render_data.vertex.len() as u16;
//             let sprite = sprites.get_sprite_by_name(&element.sprite.sprite).expect(format!("Could not find sprite: {}", element.sprite.sprite).as_str());
            
            
//             let draw_data = sprite.draw_data(element.sprite.x, element.sprite.y, element.sprite.width as usize, element.sprite.height as usize, self.viewpoint_width, self.viewpoint_height, index_offset as u32, 0, 0);
//             render_data.vertex.extend(draw_data.vertex);
//             render_data.index.extend(draw_data.index);
//         }
//         let grid_sprite_id = sprites.get_sprite_id("grid");
//         if grid_sprite_id.is_none(){
//             return render_data;
//         }
//         let grid_sprite = &world.sprites.get_sprite(grid_sprite_id.unwrap()).unwrap();
//         for grid_cell in grid.iter(){
//             if(grid_cell.x > self.camera_x as usize + self.viewpoint_width || grid_cell.x + 32 < self.camera_x as usize || grid_cell.y > self.camera_y as usize + self.viewpoint_height || grid_cell.y + 32 < self.camera_y as usize){
//                 continue;
//             }
//             let vertex_offset_x = -1 * self.camera_x as i32;
//             let vertex_offset_y = -1 * self.camera_y as i32;
//             let draw_data = grid_sprite.draw_data(grid_cell.x as f32, grid_cell.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
//             render_data.vertex.extend(draw_data.vertex);
//             render_data.index.extend(draw_data.index);
//         }
//         let (sections_0, sections_1, sections_2, sections_3) =self.get_sections(screen_width, screen_height).unwrap();
//         let mut sections = vec![];
//         sections.extend(sections_0);
//         sections.extend(sections_1);
//         sections.extend(sections_2);
//         sections.extend(sections_3); 
//         render_data.sections_a_t = sections;
        
//         render_data
//     }
// }

// pub async fn run(world: World, sprites: SpriteContainer,parser: JSON_parser, hash: HashMap<usize, ObjectJSONContainer>, camera: Camera, sprites_json_to_load: Vec<String>) {
//     let event_loop = EventLoop::new().unwrap();
//     let title = "Level Editor";
//     let window = WindowBuilder::new().with_title(title).with_inner_size(winit::dpi::LogicalSize::new(1152, 720)).build(&event_loop).unwrap();
//     let mut renderer = Renderer::new(&window, &sprites_json_to_load.clone()).await;
//     let mut level_editor = LevelEditor::new(world, renderer, camera, sprites, parser, hash);
//     level_editor.init();

//     event_loop.run(move |event, control_flow| match event {
        
//         Event::WindowEvent {
//             event,
//             window_id,
//         } if window_id == level_editor.window().id() =>{
//             match event {
//                 WindowEvent::KeyboardInput {  event,.. } => { 
//                     level_editor.process_key_input(event.clone());
//                 },
//                 WindowEvent::CloseRequested => {
//                     level_editor.save_edits();
//                     control_flow.exit();
//                 },
//                 WindowEvent::Resized(physical_size) => {
//                     level_editor.resize(physical_size);
//                 },
//                 WindowEvent::CursorMoved {position, ..} => {
//                     level_editor.process_mouse_position(position.x, position.y);
//                 },
//                 WindowEvent::MouseInput { state, button, .. } => {
//                     level_editor.process_mouse_input(state, button);
//                 },
//                 WindowEvent::RedrawRequested => {
//                     level_editor.window().request_redraw();
//                     level_editor.update();
//                     match level_editor.render() {
//                         Ok(_) => {}
//                         Err(e) => {
//                             eprintln!("Error rendering: {:?}", e);
//                         }
//                     }
                    
//                 }
//                 _ => {}
//             }   
//         }
//         _ => {}
//     }).unwrap();
// }

// pub fn level_editor_generate_world_from_json_parsed_data(data: &ParsedData) -> (World, SpriteContainer, HashMap<usize, ObjectJSONContainer>) {

    
//     let starting_level_descriptor = data.starting_level_descriptor.clone();
//     let player_descriptor = starting_level_descriptor.player;
//     let mut world = World::new(Player::new(player_descriptor.x, player_descriptor.y, player_descriptor.health, player_descriptor.max_health, player_descriptor.movement_speed, data.get_texture_id(&player_descriptor.sprite)), sprites);
//     let sprites = SpriteContainer::generate_from_json_parsed_data(data, &mut world);
//     let mut hash = HashMap::new();
//     let mut i = 0;
//     for archetype in data.entity_archetypes.iter(){
//         world.add_entity_archetype(archetype.0.clone(), archetype.1.clone());
//     }
//     for entity_descriptor in starting_level_descriptor.entities.iter(){
//         let entity = world.create_entity_with_archetype(entity_descriptor.x, entity_descriptor.y, entity_descriptor.archetype.clone());
//         world.set_sprite(entity, sprites.get_sprite(&entity_descriptor.sprite).expect(format!("Could not find sprite: {}", entity_descriptor.sprite).as_str()));
//         hash.insert(entity, ObjectJSONContainer::Entity((entity_descriptor.clone(), i.clone())));
//         i += 1;
//     }
//     i = 0;
//     for terrain_json in starting_level_descriptor.terrain.iter(){
//         let start_x = terrain_json.x;
//         let start_y = terrain_json.y;
//         let width = terrain_json.width;
//         let height = terrain_json.height;
//         let archetype = data.get_terrain_archetype(terrain_json.terrain_archetype.clone().as_str()).expect(format!("Could not find terrain archetype: {}", terrain_json.terrain_archetype).as_str());
//         let tags = archetype.basic_tags.clone();
//         match archetype.r#type.as_str() {
//             "basic" => {
//                 for x in start_x..start_x + width{
//                     for y in start_y..start_y + height{
//                         let terrain = world.add_terrain(x * 32, y * 32);
//                         world.set_sprite(terrain, sprites.get_sprite(&archetype.sprites[0]).expect(format!("Could not find sprite: {}", archetype.sprites[0]).as_str()));
//                         hash.insert(terrain, ObjectJSONContainer::Terrain((terrain_json.clone(), i.clone())));
//                         match_terrain_tags(&tags, terrain, &mut world);
//                     }
//                 }
//             },
//             "randomness" => {
//                 println!("Randomness {:?}", archetype);
//                 let random_chances = archetype.random_chances.clone().expect("Randomness archetype must have random chances");
//                 let mut random_chances_adjusted = Vec::new();
//                 let mut sum_so_far = 0.0;
//                 for chance in random_chances{
//                     random_chances_adjusted.push(chance + sum_so_far);
//                     sum_so_far += chance;
//                 }
//                 for x in start_x..start_x + width{
//                     for y in start_y..start_y + height{
//                         let terrain = world.add_terrain(x * 32, y * 32);
//                         let random_number = rand::random::<f32>();
//                         for (index, chance) in random_chances_adjusted.iter().enumerate(){
//                             if random_number < *chance{
//                                 world.set_sprite(terrain, sprites.get_sprite(&archetype.sprites[index]).expect(format!("Could not find sprite: {}", archetype.sprites[index]).as_str()));
//                                 break;
//                             }
//                         }
//                         hash.insert(terrain, ObjectJSONContainer::Terrain((terrain_json.clone(), i.clone())));
//                         match_terrain_tags(&tags, terrain, &mut world);
//                     }
//                 }
//             },
//             _ => {
//                 panic!("Unknown terrain type: {}", archetype.r#type);
//             }
//         }
//         i += 1;


//     }
//     (world, sprites, hash)
// }