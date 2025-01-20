use std::collections::{BTreeMap, HashMap};
use std::f32::consts::PI;

use crate::world::World;
use crate::rendering_engine::abstractions::{RenderData, RenderDataFull, TextSprite};
use crate::game_engine::ui::UIElement;
use wgpu_text::glyph_brush::{HorizontalAlign, Section as TextSection};

use super::entities::AttackType;
use super::ui::UIESprite;

#[derive(Debug, Clone)]
pub enum Font{
    A,
    B
}
#[derive(Debug, Clone)]
pub struct Camera{
    pub viewpoint_width: usize,
    pub viewpoint_height: usize,
    pub camera_x: f32, // top left corner of the camera in world/element coordinates
    pub camera_y: f32,
    pub ui_element_names: HashMap<String, usize>,
    pub ui_elements: HashMap<usize, UIElement>,
    pub ui_element_id: usize,
    pub level_editor: bool,
    pub text: BTreeMap<usize, TextSprite>,
    pub world_text: BTreeMap<usize, TextSprite>,
    pub world_text_id: usize,
    pub world_text_font_lookup: HashMap<usize, Font>,
    pub text_font_lookup: HashMap<usize, Font>,
    pub text_id: usize,
    pub test: f32,
}

impl Camera{
    pub fn new(viewpoint_width: usize, viewpoint_height: usize) -> Self{
        Self{
            viewpoint_width: viewpoint_width,
            viewpoint_height: viewpoint_height,
            camera_x: 20.0,
            camera_y: 40.0,
            ui_elements: HashMap::new(),
            ui_element_names: HashMap::new(),
            ui_element_id: 0,
            level_editor: false,
            text: BTreeMap::new(),
            world_text: BTreeMap::new(),
            world_text_font_lookup: HashMap::new(),
            text_font_lookup: HashMap::new(),
            text_id: 0,
            world_text_id: 0,
            test: 0.0
        }
    }
    pub fn update_ui(&mut self, world: &mut World){
        let player = world.player.borrow().clone();
        let health_bar = self.get_ui_element_mut_by_name(String::from("health_bar_inside")).unwrap();
        let health_bar_width = f32::max(0.0, (player.health as f32 / player.max_health as f32) * 250.0);
        health_bar.sprite.width = health_bar_width;
    }
    pub fn get_ui_element_mut_by_name(&mut self, name: String) -> Option<&mut UIElement> {
        let id_potential = self.get_ui_element_id_from_name(name);
        if id_potential.is_some() {
            return Some(self.get_ui_element_mut(id_potential.unwrap())); 
        }
        return None;
        
    }
    pub fn remove_ui_element(&mut self, element: usize){
        let mut name_to_remove = String::new();
        for (name, id) in self.ui_element_names.iter(){
            if *id == element{
                name_to_remove = name.clone();
                break;
            }
        }
        self.ui_element_names.remove(&name_to_remove);
        self.ui_elements.remove(&element);
    }
    pub fn add_ui_element(&mut self, name: String,  element_descriptor: crate::game_engine::ui::UIElementDescriptor) -> usize{
        let element = UIElement::new(name.clone(), element_descriptor);
        self.ui_element_names.insert(name, self.ui_element_id);
        self.ui_elements.insert(self.ui_element_id, element);
        self.ui_element_id += 1;
        self.ui_element_id - 1
    }
    pub fn get_ui_elements_at(&self, x: usize, y: usize) -> Vec<String>{
        let mut elements = Vec::new();
        for (.., element) in self.ui_elements.iter(){
            if x >= element.sprite.x as usize && x <= (element.sprite.x + element.sprite.width) as usize && y >= element.sprite.y as usize && y <= (element.sprite.y + element.sprite.height) as usize{
                elements.push(element.name.clone());
            }
        }
        return elements;
    }
    pub fn get_ui_element_id_from_name(&self, element: String) -> Option<usize>{
        self.ui_element_names.get(&element).copied()
    }
    pub fn get_ui_element(&self, element: usize) -> Option<&UIElement>{
        self.ui_elements.get(&element)
    }
    pub fn get_ui_element_mut(&mut self, element: usize) -> &mut crate::game_engine::ui::UIElement{
        self.ui_elements.get_mut(&element).unwrap()
    }
    pub fn update_camera_position(&mut self, player_x: f32, player_y: f32){
        self.camera_x = player_x - (self.viewpoint_width as f32/ 2.0);
        self.camera_y = player_y - (self.viewpoint_height as f32/ 2.0);
    }
    pub fn render_entity(&self, world: &World, entity_id: usize, entity_index_offset: u16, extra_index_offset: u16) -> (RenderData, RenderData) {
        let potentially_sprite_id = world.get_sprite(entity_id);
        if potentially_sprite_id.is_none(){
            return (RenderData::new(), RenderData::new());
        }
        let sprite_id = potentially_sprite_id.unwrap();
        let sprite = world.sprites.get_sprite(sprite_id).expect(format!("Could not find sprite {} while processing entity_id {}", sprite_id, entity_id).as_str());
        
        let vertex_offset_x = (-1.0 * self.camera_x).floor() as i32;
        let vertex_offset_y = (-1.0 * self.camera_y).floor() as i32;
        

        let entity_position_component = world.entity_position_components.get(&entity_id).expect("All entities with sprites should have a position component").borrow().clone();

        let draw_data_main = sprite.draw_data(entity_position_component.x, entity_position_component.y, 32, 32, self.viewpoint_width, self.viewpoint_height, entity_index_offset, vertex_offset_x, vertex_offset_y);
        let mut draw_data_other = RenderData::new();

        let potentially_health_component = world.entity_health_components.get(&entity_id);
        if potentially_health_component.is_some(){
            let health_component = potentially_health_component.unwrap().borrow();
            let potentially_health_bar_back_id = world.sprites.get_sprite_id("health_bar_back");
            if potentially_health_bar_back_id.is_none() {
                println!("WARNING: No Health Bar Back Sprite");
                return (draw_data_main, draw_data_other);
            }
            let entity_health_bar_sprite = world.sprites.get_sprite(potentially_health_bar_back_id.unwrap()).expect("Could not find health bar back sprite");
            let health_bar_draw_data = entity_health_bar_sprite.draw_data(entity_position_component.x - 4.0, entity_position_component.y - 15.0, 40, 12, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u16, vertex_offset_x, vertex_offset_y);
            draw_data_other.vertex.extend(health_bar_draw_data.vertex);
            draw_data_other.index.extend(health_bar_draw_data.index);
            let potentially_health_bar_id = world.sprites.get_sprite_id("health");
            if potentially_health_bar_id.is_none() {
                println!("WARNING: No Health Bar Sprite");
                return (draw_data_main, draw_data_other);
            }
            let entity_health_sprite = world.sprites.get_sprite(potentially_health_bar_id.unwrap()).expect("Could not find health bar sprite");
            let health_bar_inner_draw_data = entity_health_sprite.draw_data(entity_position_component.x - 3.0, entity_position_component.y - 14.0, (38.0 * health_component.health/health_component.max_health as f32).floor() as usize, 10, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u16, vertex_offset_x, vertex_offset_y);
            draw_data_other.vertex.extend(health_bar_inner_draw_data.vertex);
            draw_data_other.index.extend(health_bar_inner_draw_data.index);
        }
        (draw_data_main, draw_data_other)
    }
    pub fn render(&mut self, world: &mut World, uie: Vec<UIESprite>, screen_width: f32, screen_height: f32) -> RenderDataFull{
        let mut render_data = RenderDataFull::new();
        let mut terrain_data: RenderData = RenderData::new();
        let mut entity_data: RenderData = RenderData::new();
        let mut extra_data: RenderData = RenderData::new();
        let mut terrain_index_offset: u16 = 0;
        let mut entity_index_offset: u16 = 0;

        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
        let camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize);

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
        let camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize); 

        let mut chunks_loaded = Vec::new();
        for x in camera_left_chunk_x..=camera_right_chunk_x{
            for y in camera_top_chunk_y..=camera_bot_chunk_y{
                
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
                    let sprite = world.sprites.get_sprite(sprite_id).expect("Could not find sprite while processing terrain");

                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;

                    let terrain = world.get_terrain(*terrain_id).unwrap();
                    let draw_data = sprite.draw_data(terrain.x as f32, terrain.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, terrain_index_offset, vertex_offset_x, vertex_offset_y);
                    terrain_index_offset += 4;
                    terrain_data.vertex.extend(draw_data.vertex);
                    terrain_data.index.extend(draw_data.index);
                }

                for entity_id in chunk.entities_ids.iter(){

                    
                    let (draw_data, other_draw_data) = self.render_entity(world, *entity_id, entity_index_offset, extra_data.vertex.len() as u16);
                    entity_data.vertex.extend(draw_data.vertex);
                    entity_data.index.extend(draw_data.index);
                    extra_data.vertex.extend(other_draw_data.vertex);
                    extra_data.index.extend(other_draw_data.index);
                    entity_index_offset += 4;
        

                }
            }
        }
        render_data.vertex.extend(terrain_data.vertex);
        render_data.index.extend(terrain_data.index);
        self.test += 1.0;
        let mut entity_attack_draw_data = RenderData::new();
        for attack in world.entity_attacks.borrow().iter() {
            let descriptor = world.get_attack_descriptor(attack).expect("Could not find attack descriptor");
            let sprite = world.sprites.get_sprite_by_name(&descriptor.sprite).expect("Could not find attack sprite");
            let percent = attack.time_charged/descriptor.time_to_charge as f32;
            for i in 0..(percent * 100.0).floor() as usize {
                let dd = sprite.draw_data_rotated(attack.rotation * 180.0/PI, attack.x, attack.y, descriptor.reach, descriptor.width, self.viewpoint_width, self.viewpoint_height, entity_attack_draw_data.vertex.len() as u16, -1 * self.camera_x.floor() as i32, -1 * self.camera_y.floor() as i32);
                entity_attack_draw_data.vertex.extend(dd.vertex);
                entity_attack_draw_data.index.extend(dd.index);
            }
        }
        entity_attack_draw_data.offset(render_data.vertex.len() as u16);
        render_data.vertex.extend(entity_attack_draw_data.vertex);
        render_data.index.extend(entity_attack_draw_data.index);
        entity_data.offset(render_data.vertex.len() as u16);
        render_data.vertex.extend(entity_data.vertex);
        render_data.index.extend(entity_data.index);

        let player_draw_data = world.player.borrow().draw_data(world, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
    
        render_data.vertex.extend(player_draw_data.vertex);
        render_data.index.extend(player_draw_data.index);

        extra_data.offset(render_data.vertex.len() as u16);    
        render_data.vertex.extend(extra_data.vertex);
        render_data.index.extend(extra_data.index);

       
        let mut melee = false;

        let mut player_effect_draw_data = RenderData::new();
        for effect in world.player_attacks.borrow().iter(){
            let mut sprite = None;
            let mut width = None;
            let mut height = None;
            match effect.attack_type {
                AttackType::Melee => {

                    melee = true;
                    height = effect.stats.width;
                    width = effect.stats.reach;
                    
                    let sprite_id = world.sprites.get_sprite_id(effect.sprite.as_str()).expect(format!("Could not find melee sprite {}", effect.sprite).as_str());
                    sprite = Some(world.sprites.get_sprite(sprite_id).expect(format!("Could not find melee attack sprite {}", effect.sprite.as_str()).as_str()));
                }
                AttackType::Ranged => {
                    width = effect.stats.size;
                    height = effect.stats.size;
                    let sprite_id = world.sprites.get_sprite_id(effect.sprite.as_str()).expect(format!("Could not find projectile sprite {}", effect.sprite).as_str());
                    sprite = Some(world.sprites.get_sprite(sprite_id).expect(format!("Could not find projectile sprite {}", effect.sprite.as_str()).as_str()));
                }
                _ => {}
            }
            if sprite.is_none(){
                continue;
            }
            if width.is_none() || height.is_none(){
                continue;
            }
            if melee {
                let draw_data = sprite.unwrap().draw_data_rotated(effect.angle, effect.x, effect.y, width.unwrap().floor() as usize, height.unwrap().floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
                player_effect_draw_data.vertex.extend(draw_data.vertex);
                player_effect_draw_data.index.extend(draw_data.index);
                continue;
            } else{
                let draw_data = sprite.unwrap().draw_data_rotated(effect.angle + 90.0, effect.x, effect.y, width.unwrap().floor() as usize, height.unwrap().floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
                player_effect_draw_data.vertex.extend(draw_data.vertex);
                player_effect_draw_data.index.extend(draw_data.index);
                continue;
            }
        }
        player_effect_draw_data.offset(render_data.vertex.len() as u16);
        render_data.vertex.extend(player_effect_draw_data.vertex);
        render_data.index.extend(player_effect_draw_data.index);

        world.set_loaded_chunks(chunks_loaded);
        let mut sorted_ui_elements: Vec<&UIESprite> = self.ui_elements.values().filter_map(|x| if x.visible { Some(&x.sprite) } else { None }).collect();
        sorted_ui_elements.extend(&uie);
        sorted_ui_elements.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        for element in sorted_ui_elements.iter(){
            let element_sprite = world.sprites.get_sprite_by_name(&element.sprite).expect(format!("Could not find sprite with name {} for ui element", element.sprite).as_str());
            let draw_data = element_sprite.draw_data(element.x, element.y, element.width.floor() as usize, element.height.floor() as usize, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, 0, 0);
            render_data.vertex.extend(draw_data.vertex);
            render_data.index.extend(draw_data.index);
        }
        (render_data.sections_a, render_data.sections_b) = self.get_sections(screen_width, screen_height);
        render_data
    }
    pub fn add_text(&mut self, text: String, font: Font,  x: f32, y: f32, w: f32, h: f32, font_size: f32, color: [f32; 4], align: HorizontalAlign) -> usize{
        self.text.insert(self.text_id,TextSprite::new(text, font_size, x, y, w, h, color, align));
        self.text_font_lookup.insert(self.text_id, font);
        self.text_id += 1;
        self.text_id - 1
    }
    pub fn add_world_text(&mut self, text: String, font: Font, x: f32, y: f32, w: f32, h: f32, font_size: f32, color: [f32; 4], align: HorizontalAlign) -> usize{
        self.world_text.insert(self.world_text_id,TextSprite::new(text, font_size, x, y, w, h, color, align));
        self.world_text_font_lookup.insert(self.world_text_id, font);
        self.world_text_id += 1;
        self.world_text_id - 1
    }
    pub fn remove_world_text(&mut self, id: usize){
        self.world_text.remove(&id);
    }
    pub fn get_world_text_mut(&mut self, id: usize) -> Option<&mut TextSprite>{
        self.world_text.get_mut(&id)
    }
    pub fn remove_text(&mut self, id: usize){
        self.text.remove(&id);
    }
    pub fn get_text_mut(&mut self, id: usize) -> Option<&mut TextSprite>{
        self.text.get_mut(&id)
    }
    pub fn get_sections(&self, screen_width: f32, screen_height: f32) -> (Vec<TextSection>, Vec<TextSection>){
        let mut sections_a = Vec::new();
        let mut sections_b = Vec::new();
        for (id, text) in self.text.iter(){
            match self.text_font_lookup.get(id).expect(format!("Could not find font for text with id {}", id).as_str()){
                Font::A => {
                    sections_a.push(text.get_section(&self, screen_width, screen_height, 0.0, 0.0));
                },
                Font::B => {
                    sections_b.push(text.get_section(&self, screen_width, screen_height, 0.0, 0.0));
                }
            }
        }
        for (id, text) in self.world_text.iter(){
            match self.world_text_font_lookup.get(id).expect(format!("Could not find font for text with id {}", id).as_str()){
                Font::A => {
                    sections_a.push(text.get_section(&self, screen_width, screen_height, self.camera_x * -1.0, self.camera_y * -1.0));
                },
                Font::B => {
                    sections_b.push(text.get_section(&self, screen_width, screen_height, self.camera_x * -1.0, self.camera_y * -1.0));
                }
            }
        }
        (sections_a, sections_b)
    }
}
