use std::collections::{BTreeMap, HashMap};

use crate::world::World;
use crate::rendering_engine::abstractions::{RenderData, RenderDataFull, TextSprite};
use crate::game_engine::ui::UIElement;
use wgpu_text::glyph_brush::{HorizontalAlign, Section as TextSection};

use super::player_attacks::PlayerAttackDescriptor;
#[derive(Debug, Clone)]
pub struct Camera{
    pub viewpoint_width: usize,
    pub viewpoint_height: usize,
    pub camera_x: f32, // top left corner of the camera in world/element coordinates
    pub camera_y: f32,
    pub ui_element_names: HashMap<String, usize>,
    pub ui_elements: BTreeMap<usize, UIElement>,
    pub ui_element_id: usize,
    pub level_editor: bool,
    pub text: BTreeMap<usize, TextSprite>,
    pub text_id: usize,
}

impl Camera{
    pub fn new(viewpoint_width:usize, viewpoint_height:usize) -> Self{
        Self{
            viewpoint_width: viewpoint_width,
            viewpoint_height: viewpoint_height,
            camera_x: 20.0,
            camera_y: 40.0,
            ui_elements: BTreeMap::new(),
            ui_element_names: HashMap::new(),
            ui_element_id: 0,
            level_editor: false,
            text: BTreeMap::new(),
            text_id: 0,
        }
    }
    pub fn update_ui(&mut self, world: &mut World){
        let player = world.player.borrow().clone();
        let health_bar = self.get_ui_element_mut(self.get_ui_element_id_from_name(String::from("health_bar_inside")).unwrap());
        let health_bar_width = f32::max(0.0, (player.health as f32 / player.max_health as f32) * 250.0);
        health_bar.width = health_bar_width;
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
            if x >= element.x as usize && x <= (element.x + element.width) as usize && y >= element.y as usize && y <= (element.y + element.height) as usize{
                elements.push(element.name.clone());
            }
        }
        return elements;
    }
    pub fn get_ui_element_id_from_name(&self, element: String) -> Option<usize>{
        self.ui_element_names.get(&element).copied()
    }
    pub fn get_ui_element(&self, element: usize) -> Option<UIElement>{
        self.ui_elements.get(&element).cloned()
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
        
        let vertex_offset_x = -1 * self.camera_x as i32;
        let vertex_offset_y = -1 * self.camera_y as i32;
        

        let entity_position_component = world.entity_position_components.get(&entity_id).expect("All entities with sprites should have a position component").borrow().clone();

        let draw_data_main = sprite.draw_data(entity_position_component.x, entity_position_component.y, 32, 32, self.viewpoint_width, self.viewpoint_height, entity_index_offset, vertex_offset_x, vertex_offset_y);
        let mut draw_data_other = RenderData::new();

        let potentially_health_component = world.entity_health_components.get(&entity_id);
        if potentially_health_component.is_some(){
            let health_component = potentially_health_component.unwrap().borrow();
            let potentially_health_bar_back_id = world.sprites.get_sprite_id("health_bar_back");
            if potentially_health_bar_back_id.is_none() {
                // println!("WARNING: No Health Bar Back Sprite");
                return (draw_data_main, draw_data_other);
            }
            let entity_health_bar_sprite = world.sprites.get_sprite(potentially_health_bar_back_id.unwrap()).expect("Could not find health bar back sprite");
            let health_bar_draw_data = entity_health_bar_sprite.draw_data(entity_position_component.x - 4.0, entity_position_component.y - 15.0, 40, 12, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u16, vertex_offset_x, vertex_offset_y);
            draw_data_other.vertex.extend(health_bar_draw_data.vertex);
            draw_data_other.index.extend(health_bar_draw_data.index);
            let potentially_health_bar_id = world.sprites.get_sprite_id("health");
            if potentially_health_bar_id.is_none() {
                // println!("WARNING: No Health Bar Sprite");
                return (draw_data_main, draw_data_other);
            }
            let entity_health_sprite = world.sprites.get_sprite(potentially_health_bar_id.unwrap()).expect("Could not find health bar sprite");
            let health_bar_inner_draw_data = entity_health_sprite.draw_data(entity_position_component.x - 3.0, entity_position_component.y - 14.0, (38.0 * health_component.health/health_component.max_health as f32).floor() as usize, 10, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u16, vertex_offset_x, vertex_offset_y);
            draw_data_other.vertex.extend(health_bar_inner_draw_data.vertex);
            draw_data_other.index.extend(health_bar_inner_draw_data.index);
        }
        (draw_data_main, draw_data_other)
    }
    pub fn render(&mut self, world: &mut World) -> RenderDataFull{
        let mut render_data = RenderDataFull::new();
        let mut terrain_data: RenderData = RenderData::new();
        let mut entity_data: RenderData = RenderData::new();
        let mut extra_data: RenderData = RenderData::new();
        let mut terrain_index_offset: u16 = 0;
        let mut entity_index_offset: u16 = 0;
        let mut extra_index_offset: u16 = 0;
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

                    
                    let (draw_data, other_draw_data) = self.render_entity(world, *entity_id, entity_index_offset, extra_index_offset);
                    entity_data.vertex.extend(draw_data.vertex);
                    entity_data.index.extend(draw_data.index);
                    extra_data.vertex.extend(other_draw_data.vertex);
                    extra_data.index.extend(other_draw_data.index);
                    entity_index_offset += 4;
                    extra_index_offset += extra_data.vertex.len() as u16;
        

                }
            }
        }
        render_data.vertex.extend(terrain_data.vertex);
        render_data.index.extend(terrain_data.index);

        let mut entity_attack_draw_data = RenderData::new();
        for attack in world.entity_attacks.borrow().iter() {
            let sprite = world.sprites.get_sprite(attack.sprite_id).expect("Could not find attack sprite");
            let percent = attack.time_charged/attack.time_to_charge as f32;
            for i in 0..(percent * 100.0).floor() as usize {
                let dd = sprite.draw_data(attack.x, attack.y, attack.reach, attack.width, self.viewpoint_width, self.viewpoint_height, entity_attack_draw_data.vertex.len() as u16, -1 * self.camera_x.floor() as i32, -1 * self.camera_y.floor() as i32).rotated(attack.rotation * 180.0/std::f32::consts::PI);
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

        let player_draw_data = player.draw_data(world, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
    
        render_data.vertex.extend(player_draw_data.vertex);
        render_data.index.extend(player_draw_data.index);

        extra_data.offset(render_data.vertex.len() as u16);    
        render_data.vertex.extend(extra_data.vertex);
        render_data.index.extend(extra_data.index);

       
        let mut melee = false;

        let mut player_effect_draw_data = RenderData::new();
        for effect in world.player_attacks.borrow().iter(){
            let effect_archetype = world.player_archetype_descriptor_lookup.get(&effect.archetype).expect(format!("Could not find effect archetype {}", effect.archetype).as_str());
            let mut sprite = None;
            let mut width = None;
            let mut height = None;
            match effect_archetype {
                PlayerAttackDescriptor::Projectile(projectile_descriptor) => {
                    width = Some(projectile_descriptor.size);
                    height = Some(projectile_descriptor.size);
                    let sprite_id = world.sprites.get_sprite_id(projectile_descriptor.sprite.as_str()).expect(format!("Could not find projectile sprite {}", projectile_descriptor.sprite).as_str());
                    sprite = Some(world.sprites.get_sprite(sprite_id).expect(format!("Could not find projectile sprite {}", projectile_descriptor.sprite.as_str()).as_str()).clone());
                }
                PlayerAttackDescriptor::Melee(melee_descriptor) => {
                    melee = true;
                    height = Some(melee_descriptor.width);
                    width = Some(melee_descriptor.reach);
                    
                    let sprite_id = world.sprites.get_sprite_id(melee_descriptor.sprite.as_str()).expect(format!("Could not find melee sprite {}", melee_descriptor.sprite).as_str());
                    sprite = Some(world.sprites.get_sprite(sprite_id).expect(format!("Could not find melee attack sprite {}", melee_descriptor.sprite.as_str()).as_str()).clone());

                }
            }
            if sprite.is_none(){
                continue;
            }
            if width.is_none() || height.is_none(){
                continue;
            }
            if melee {
                let angle = -1.0 * (f32::atan2(effect.direction[1],  effect.direction[0]));
                let draw_data = sprite.unwrap().draw_data(effect.x, effect.y, width.unwrap().floor() as usize, height.unwrap().floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32).rotated(angle * 180.0/std::f32::consts::PI);
                player_effect_draw_data.vertex.extend(draw_data.vertex);
                player_effect_draw_data.index.extend(draw_data.index);
                continue;
            }
            let draw_data = sprite.unwrap().draw_data(effect.x, effect.y, width.unwrap().floor() as usize, height.unwrap().floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
            player_effect_draw_data.vertex.extend(draw_data.vertex);
            player_effect_draw_data.index.extend(draw_data.index);
        }
        player_effect_draw_data.offset(render_data.vertex.len() as u16);
        render_data.vertex.extend(player_effect_draw_data.vertex);
        render_data.index.extend(player_effect_draw_data.index);

        world.set_loaded_chunks(chunks_loaded);
        for (.., element) in self.ui_elements.iter(){
            if !element.visible{
                continue;
            }
            let element_sprite = world.sprites.sprites[element.sprite_id];
            let draw_data = element_sprite.draw_data(element.x, element.y, element.width.floor() as usize, element.height.floor() as usize, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, 0, 0);
            render_data.vertex.extend(draw_data.vertex);
            render_data.index.extend(draw_data.index);
        }
        render_data.sections = self.get_sections(self.viewpoint_width as f32, self.viewpoint_height as f32);
        render_data
    }
    pub fn add_text(&mut self, text: String, x: f32, y: f32, w: f32, h: f32, font_size: f32, color: [f32; 4], align: HorizontalAlign) -> usize{
        self.text.insert(self.text_id,TextSprite::new(text, font_size, x, y, w, h, color, align));
        self.text_id += 1;
        self.text_id - 1
    }
    pub fn remove_text(&mut self, id: usize){
        self.text.remove(&id);
    }
    pub fn get_sections(&self, screen_width: f32, screen_height: f32) -> Vec<TextSection>{
        let mut sections = Vec::new();
        for (id, text) in self.text.iter(){
            sections.push(text.get_section(&self, screen_width, screen_height).clone());
        }
        sections
    }
}
