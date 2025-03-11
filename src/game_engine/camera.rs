use std::collections::BTreeMap;
use std::f32::consts::PI;

use crate::error::PError;
use crate::{perror, ptry, punwrap};
use crate::world::World;
use crate::rendering_engine::abstractions::{RenderData, RenderDataFull, Sprite, SpriteContainer, TextSprite, UIEFull};
use crate::game_engine::ui::UIElement;
use crate::game_engine::player_attacks::PlayerAttackType;
use compact_str::CompactString;
use itertools::izip;
use wgpu_text::glyph_brush::{HorizontalAlign, Section as TextSection};
use rustc_hash::FxHashMap;

use super::entity_components::{DamageableComponent, PositionComponent};
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
    ui_element_names: FxHashMap<CompactString, usize>,
    pub ui_elements: FxHashMap<usize, UIElement>,
    ui_element_id: usize,
    pub level_editor: bool,
    text: BTreeMap<usize, TextSprite>,
    world_text: BTreeMap<usize, TextSprite>,
    world_text_id: usize,
    world_text_font_lookup: FxHashMap<usize, Font>,
    text_font_lookup: FxHashMap<usize, Font>,
    text_id: usize,
    test: f32,
    temp_uie: Vec<TextSprite>,
    temp_uie2: Vec<TextSprite>
}

impl Camera{
    pub fn new(viewpoint_width: usize, viewpoint_height: usize) -> Self{
        Self{
            viewpoint_width,
            viewpoint_height,
            camera_x: 20.0,
            camera_y: 40.0,
            ui_elements: FxHashMap::default(),
            ui_element_names: FxHashMap::default(),
            ui_element_id: 0,
            level_editor: false,
            text: BTreeMap::new(),
            world_text: BTreeMap::new(),
            world_text_font_lookup: FxHashMap::default(),
            text_font_lookup: FxHashMap::default(),
            text_id: 0,
            world_text_id: 0,
            test: 0.0,
            temp_uie: Vec::new(),
            temp_uie2: Vec::new()
        }
    }
    pub fn update_ui(&mut self, world: &mut World) -> Result<(), PError>{
        let player = world.player.borrow().clone();
        let health_bar = punwrap!(self.get_ui_element_mut_by_name(CompactString::from("health_bar_inside")), "Could not find health bar inside ui element");
        let health_bar_width = f32::max(0.0, (player.health / player.max_health as f32) * 250.0);
        health_bar.sprite.width = health_bar_width;
        Ok(())
    }
    pub fn get_ui_element_mut_by_name(&mut self, name: CompactString) -> Option<&mut UIElement> {
        self.get_ui_element_id_from_name(name).and_then(
            |x| self.get_ui_element_mut(x)
        )
    }
    pub fn remove_ui_element(&mut self, element: usize) -> Result<(), PError>{
        let mut name_to_remove: Option<CompactString> = None;
        for (name, id) in self.ui_element_names.iter(){
            if *id == element{
                name_to_remove = Some(name.clone());
                break;
            }
        }
        if let Some(name_to_remove) = name_to_remove {
            if self.ui_element_names.remove(&name_to_remove).is_none() {
                return Err(perror!("Element name {} associated with id {} was not found", name_to_remove, element));
            };
            if self.ui_elements.remove(&element).is_none(){
                return Err(perror!(NotFound, "There was no element to remove with id {}", element));
            };
        } else { 
            return Err(perror!(NotFound, "no element with id {}", element));
        }

        Ok(())
    }
    pub fn add_ui_element(&mut self, name: CompactString,  element_descriptor: crate::game_engine::ui::UIElementDescriptor) -> usize{
        let element = UIElement::new(name.clone(), element_descriptor);
        self.ui_element_names.insert(name, self.ui_element_id);
        self.ui_elements.insert(self.ui_element_id, element);
        self.ui_element_id += 1;
        self.ui_element_id - 1
    }
    pub fn get_ui_elements_at(&self, x: usize, y: usize) -> Vec<CompactString>{
        let mut elements = Vec::new();
        for (.., element) in self.ui_elements.iter(){
            if x >= element.sprite.x as usize && x <= (element.sprite.x + element.sprite.width) as usize && y >= element.sprite.y as usize && y <= (element.sprite.y + element.sprite.height) as usize{
                elements.push(element.name.clone());
            }
        }
        elements
    }
    pub fn get_ui_element_id_from_name(&self, element: CompactString) -> Option<usize>{
        self.ui_element_names.get(&element).copied()
    }
    pub fn get_ui_element(&self, element: usize) -> Option<&UIElement>{
        self.ui_elements.get(&element)
    }
    pub fn get_ui_element_mut(&mut self, element: usize) -> Option<&mut crate::game_engine::ui::UIElement>{
        self.ui_elements.get_mut(&element)
    }
    pub fn update_camera_position(&mut self, player_x: f32, player_y: f32){
        self.camera_x = player_x - (self.viewpoint_width as f32/ 2.0);
        self.camera_y = player_y - (self.viewpoint_height as f32/ 2.0);
    }
    pub fn render_entity(&self, sprite: &Sprite, position_component: &PositionComponent, entity_index_offset: u32) -> RenderData {
        let vertex_offset_x = (-1.0 * self.camera_x).floor() as i32;
        let vertex_offset_y = (-1.0 * self.camera_y).floor() as i32;
        
        sprite.draw_data(position_component.x, position_component.y, 32, 32, self.viewpoint_width, self.viewpoint_height, entity_index_offset, vertex_offset_x, vertex_offset_y)
    }

    pub fn render_health_bar(&self, entity_position_component: &PositionComponent, health_component: &DamageableComponent, extra_index_offset: u32, sprites: &SpriteContainer) -> Result<RenderData, PError> {
        let vertex_offset_x = (-1.0 * self.camera_x).floor() as i32;
        let vertex_offset_y = (-1.0 * self.camera_y).floor() as i32;

        let mut draw_data_other = RenderData::new();
        let potentially_health_bar_back_id = sprites.get_sprite_id("health_bar_back");
        if potentially_health_bar_back_id.is_none() {
            return Err(perror!(MissingExpectedGlobalSprite, "There was no health bar back sprite"));
        }
        let entity_health_bar_sprite = punwrap!(sprites.get_sprite(potentially_health_bar_back_id.unwrap()), Invalid, "health bar back sprite was found with id {} but there is no sprite with id {}?", potentially_health_bar_back_id.unwrap(), potentially_health_bar_back_id.unwrap());
        let health_bar_draw_data = entity_health_bar_sprite.draw_data(entity_position_component.x - 4.0, entity_position_component.y - 15.0, 40, 12, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
        draw_data_other.vertex.extend(health_bar_draw_data.vertex);
        draw_data_other.index.extend(health_bar_draw_data.index);
        let potentially_health_bar_id = sprites.get_sprite_id("health");
        if potentially_health_bar_id.is_none() {
            return Err(perror!(MissingExpectedGlobalSprite, "There was no health bar inside sprite"));
        }
        let entity_health_sprite = punwrap!(sprites.get_sprite(potentially_health_bar_id.unwrap()), Invalid, "health bar inside sprite was found with id {} but there is no sprite with id {}?", potentially_health_bar_back_id.unwrap(), potentially_health_bar_back_id.unwrap());
        let health_bar_inner_draw_data = entity_health_sprite.draw_data(entity_position_component.x - 3.0, entity_position_component.y - 14.0, (38.0 * health_component.health/health_component.max_health as f32).floor() as usize, 10, self.viewpoint_width, self.viewpoint_height, extra_index_offset + draw_data_other.vertex.len() as u32, vertex_offset_x, vertex_offset_y);
        draw_data_other.vertex.extend(health_bar_inner_draw_data.vertex);
        draw_data_other.index.extend(health_bar_inner_draw_data.index);
        Ok(draw_data_other)
    }
    pub fn render(&mut self, world: &mut World, uie: UIEFull, screen_width: f32, screen_height: f32) -> Result<RenderDataFull, PError>{
        let mut render_data = RenderDataFull::new();
        let mut terrain_data: RenderData = RenderData::new();
        let mut entity_data: RenderData = RenderData::new();
        let mut extra_data: RenderData = RenderData::new();
        let mut terrain_index_offset: u32 = 0;

        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
        let camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize);

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
        let camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize); 

        let mut chunks_loaded = Vec::new();
        let mut entities_to_render = Vec::new();
        for x in camera_left_chunk_x..=camera_right_chunk_x{
            for y in camera_top_chunk_y..=camera_bot_chunk_y{
                
                let chunk_id = world.get_chunk_from_chunk_xy(x,y);
                
                if chunk_id.is_none(){
                    continue;
                }
                let chunk_id = chunk_id.unwrap();

                let chunk = &world.chunks.borrow()[chunk_id];

                chunks_loaded.push(chunk_id);
                for terrain_id in chunk.terrain_ids.iter(){
                    let sprite_id = match world.get_terrain_sprite(*terrain_id) {
                        Some(id) => id,
                        None => continue
                    };
                    let sprite = punwrap!(world.sprites.get_sprite(sprite_id), Expected, "Sprite in sprite_lookup for terrain with id {} is a non-existent sprite", terrain_id);

                    let vertex_offset_x = -self.camera_x as i32;
                    let vertex_offset_y = -self.camera_y as i32;

                    let terrain = punwrap!(world.get_terrain(*terrain_id), Invalid, "chunk with id {} contains terrain with id {} but that terrain does not exist", chunk_id, terrain_id);
                    let draw_data = sprite.draw_data(terrain.x as f32, terrain.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, terrain_index_offset, vertex_offset_x, vertex_offset_y);
                    terrain_index_offset += 4;
                    terrain_data.vertex.extend(draw_data.vertex);
                    terrain_data.index.extend(draw_data.index);
                }

                entities_to_render.extend(chunk.entities_ids.clone());
            }
        }


        entities_to_render.sort();
        if !entities_to_render.is_empty() {
            // main rendering
            let mut entity_to_render_index = 0;
            for (i, (sprite_component, position_component)) in izip!( 
                world.components.sprite_components.iter(),
                world.components.position_components.iter()
            ).enumerate().filter_map(
            |(i, (x, y))| 
                if entity_to_render_index == entities_to_render.len() {None} 
                else if entities_to_render[entity_to_render_index] == i && x.is_some() && y.is_some() {entity_to_render_index += 1; Some((i,(x.as_ref().unwrap().borrow(), y.as_ref().unwrap().borrow()))) }
                else { None }
            ) {
                let sprite = punwrap!(world.sprites.get_sprite(sprite_component.sprite), Expected, "Sprite in sprite_component for entity with id {} is a non-existent sprite", i);

                let dd = self.render_entity(sprite, &position_component,entity_data.vertex.len() as u32);
                entity_data.vertex.extend(dd.vertex);
                entity_data.index.extend(dd.index);

            }
            // health bars
            entity_to_render_index = 0;

            for (i, (position_component, damageable_component)) in izip!(
                world.components.position_components.iter(),
                world.components.damageable_components.iter()
            ).enumerate().filter_map(
                |(i, (x, y))| 
                if entity_to_render_index == entities_to_render.len() {None} 
                else if entities_to_render[entity_to_render_index] == i && x.is_some() && y.is_some() {entity_to_render_index += 1; Some((i,(x.as_ref().unwrap().borrow(), y.as_ref().unwrap().borrow()))) }
                else { None }
            ) {
                
                let dd = ptry!(self.render_health_bar(&position_component, &damageable_component, extra_data.vertex.len() as u32, &world.sprites), "while rendering health bar for entity with id {}", i);
                extra_data.vertex.extend(dd.vertex);
                extra_data.index.extend(dd.index);
            }


            render_data.vertex.extend(terrain_data.vertex);
            render_data.index.extend(terrain_data.index);

            self.test += 1.0;
            let mut entity_attack_draw_data = RenderData::new();
            for attack in world.entity_attacks.borrow().iter() {
                let descriptor = punwrap!(world.get_attack_descriptor(attack), Expected, "Could not find attack descriptor for attack: {:?}", attack);
                let sprite = punwrap!(world.sprites.get_sprite_by_name(&descriptor.sprite), Expected, "Attack descriptor for attack: {:?}, refers to a non-existent sprite: {}", attack, descriptor.sprite);
                let percent = attack.time_charged/descriptor.time_to_charge as f32;
                for _ in 0..(percent * 100.0).floor() as usize {
                    let dd = sprite.draw_data_rotated(attack.rotation * 180.0/PI, attack.x, attack.y, descriptor.reach, descriptor.width, self.viewpoint_width, self.viewpoint_height, entity_attack_draw_data.vertex.len() as u32, -self.camera_x.floor() as i32, -self.camera_y.floor() as i32);
                    entity_attack_draw_data.vertex.extend(dd.vertex);
                    entity_attack_draw_data.index.extend(dd.index);
                }
            }
            entity_attack_draw_data.offset(render_data.vertex.len() as u32);
            render_data.vertex.extend(entity_attack_draw_data.vertex);
            render_data.index.extend(entity_attack_draw_data.index);
            entity_data.offset(render_data.vertex.len() as u32);
            render_data.vertex.extend(entity_data.vertex);
            render_data.index.extend(entity_data.index);
        }else {

            render_data.vertex.extend(terrain_data.vertex);
            render_data.index.extend(terrain_data.index);
        }


        if let Some(e) = world.cur_exit {
            let x = e[0] * 32 - 7;
            let y = e[1] * 32 - 7;

            let sprite = world.sprites.get_sprite_by_name("health").unwrap();
            let dd = sprite.draw_data(x as f32, y as f32, 46, 46, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u32, -self.camera_x as i32, -self.camera_y as i32);
            render_data.vertex.extend(dd.vertex);
            render_data.index.extend(dd.index);
        }


        let player_draw_data = ptry!(world.player.borrow().draw_data(world, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u32, -self.camera_x as i32, -self.camera_y as i32));
    
        render_data.vertex.extend(player_draw_data.vertex);
        render_data.index.extend(player_draw_data.index);

        extra_data.offset(render_data.vertex.len() as u32);    
        render_data.vertex.extend(extra_data.vertex);
        render_data.index.extend(extra_data.index);

        let mut item_on_floor_render_data = RenderData::new();
        self.temp_uie2.clear();
        for item_on_floor in world.items_on_floor.borrow().iter() {
            let sprite_id = punwrap!(world.sprites.get_sprite_id(&item_on_floor.item.sprite), Expected, "Item on floor with refers to a non-existent sprite {}", item_on_floor.item.sprite);
            let sprite = punwrap!(world.sprites.get_sprite(sprite_id), Expected, "Item on floor with refers to a non-existent sprite {}", item_on_floor.item.sprite);
            let draw_data = sprite.draw_data(item_on_floor.x, item_on_floor.y, 24, 24, self.viewpoint_width, self.viewpoint_height, item_on_floor_render_data.vertex.len() as u32, -self.camera_x.floor() as i32, -self.camera_y.floor() as i32);
            item_on_floor_render_data.vertex.extend(draw_data.vertex);
            item_on_floor_render_data.index.extend(draw_data.index);

            let display = item_on_floor.display();
            for text in display.text{
                self.temp_uie2.push(text);
            }
            for ui in display.sprites {
                let sprite_id = punwrap!(world.sprites.get_sprite_id(&ui.sprite), Expected, "item on floor display refers to a non-existent sprite {}", ui.sprite);
                let sprite = punwrap!(world.sprites.get_sprite(sprite_id), Expected, "item on floor display refers to a non-existent sprite {}", ui.sprite);
                let draw_data = sprite.draw_data(ui.x, ui.y, ui.width as usize, ui.height as usize, self.viewpoint_width, self.viewpoint_height, item_on_floor_render_data.vertex.len() as u32, -self.camera_x.floor() as i32, -self.camera_y.floor() as i32);
                item_on_floor_render_data.vertex.extend(draw_data.vertex);
                item_on_floor_render_data.index.extend(draw_data.index);
            }
        }
       
        item_on_floor_render_data.offset(render_data.vertex.len() as u32);
        render_data.vertex.extend(item_on_floor_render_data.vertex);
        render_data.index.extend(item_on_floor_render_data.index);
        for text in self.temp_uie2.iter() {
            let dd = text.get_section(self, screen_width, screen_height, -self.camera_x, -self.camera_y);
            render_data.sections_a_b.push(dd);
        }

       
        let mut melee = false;

        let mut player_effect_draw_data = RenderData::new();
        for effect in world.player_attacks.borrow().iter(){
            let mut sprite = None;
            let mut width = None;
            let mut height = None;
            match effect.attack_type {
                PlayerAttackType::Melee | PlayerAttackType::MeleeAbility => {

                    melee = true;
                    height = effect.stats.width;
                    width = effect.stats.reach;
                    
                    let sprite_id = punwrap!(world.sprites.get_sprite_id(effect.sprite.as_str()), Expected, "player melee attack {:?} refers to a non-existent sprite {}", effect, effect.sprite.as_str());
                    sprite = Some(punwrap!(world.sprites.get_sprite(sprite_id), Expected, "player melee attack {:?} refers to a non-existent sprite {}", effect, effect.sprite.as_str()));
                }
                PlayerAttackType::Ranged | PlayerAttackType::RangedAbility => {
                    width = effect.stats.size;
                    height = effect.stats.size;
                    let sprite_id = punwrap!(world.sprites.get_sprite_id(effect.sprite.as_str()), Expected, "player ranged attack {:?} refers to a non-existent sprite {}", effect, effect.sprite.as_str());
                    sprite = Some(punwrap!(world.sprites.get_sprite(sprite_id), Expected, "player ranged attack {:?} refers to a non-existent sprite {}", effect, effect.sprite.as_str()));
                }
                _ => {}
            }
            if sprite.is_none(){
                return Err(perror!("Player attack {:?} has no sprite", effect));
            }
            if width.is_none() || height.is_none(){
                return Err(perror!("Player attack {:?} has no width or no height?", effect));
            }
            let sprite = sprite.unwrap();
            let width = width.unwrap().get_value();
            let height = height.unwrap().get_value();
            if melee {
                let draw_data = sprite.draw_data_rotated(effect.angle, effect.x, effect.y, width.floor() as usize, height.floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u32, -self.camera_x as i32, -self.camera_y as i32);
                player_effect_draw_data.vertex.extend(draw_data.vertex);
                player_effect_draw_data.index.extend(draw_data.index);
                continue;
            } else{
                let draw_data = sprite.draw_data_rotated(effect.angle + 90.0, effect.x, effect.y, width.floor() as usize, height.floor() as usize, self.viewpoint_width, self.viewpoint_height, player_effect_draw_data.vertex.len() as u32, -self.camera_x as i32, -self.camera_y as i32);
                player_effect_draw_data.vertex.extend(draw_data.vertex);
                player_effect_draw_data.index.extend(draw_data.index);
                continue;
            }
        }
        player_effect_draw_data.offset(render_data.vertex.len() as u32);
        render_data.vertex.extend(player_effect_draw_data.vertex);
        render_data.index.extend(player_effect_draw_data.index);
        render_data.index_behind_text = render_data.index.len() as u32;

        world.set_loaded_chunks(chunks_loaded);
        let mut sorted_ui_elements: Vec<&UIESprite> = self.ui_elements.values().filter_map(|x| if x.visible { Some(&x.sprite) } else { None }).collect();
        sorted_ui_elements.extend(&uie.sprites);
        sorted_ui_elements.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        for element in sorted_ui_elements.iter(){
            let element_sprite = punwrap!(world.sprites.get_sprite_by_name(&element.sprite), Expected, "UI: {:?} refers to a non-existent sprite {}", element, element.sprite);
            let draw_data = element_sprite.draw_data(element.x, element.y, element.width.floor() as usize, element.height.floor() as usize, self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u32, 0, 0);
            render_data.vertex.extend(draw_data.vertex);
            render_data.index.extend(draw_data.index);
        }
        let temp_uie_clone = uie.text.clone();
        self.temp_uie = temp_uie_clone; // THIS IS THE JANKIEST THING IVE EVER SEEN BUT ITS THE ONLY WAY IT WORKS FOR SOME REASON
        
        let (rat, rab, rbt, rbb) = ptry!(self.get_sections(screen_width, screen_height), "failed to get text sections");
        render_data.sections_a_t.extend(rat);
        render_data.sections_a_b.extend(rab);
        render_data.sections_b_t.extend(rbt);
        render_data.sections_b_b.extend(rbb);
        let mut f = Vec::new();
        for text in self.temp_uie.iter() {
            f.push(text.get_section(self, screen_width, screen_height, 0.0, 0.0).clone());
        }
        render_data.sections_a_t.extend(f);
        Ok(render_data)
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
    pub fn remove_world_text(&mut self, id: usize) -> Result<(), PError>{
        if self.world_text.remove(&id).is_none() {
            return Err(perror!(NotFound, "There was no world text to remove with id {}", id))
        }
        Ok(())
    }
    pub fn get_world_text_mut(&mut self, id: usize) -> Option<&mut TextSprite>{
        self.world_text.get_mut(&id)
    }
    pub fn remove_text(&mut self, id: usize) -> Result<(), PError>{
        if self.text.remove(&id).is_none() {
            return Err(perror!(NotFound, "There was no text to remove with id {}", id))
        }
        Ok(())
    }
    pub fn get_text_mut(&mut self, id: usize) -> Option<&mut TextSprite>{
        self.text.get_mut(&id)
    }
    #[allow(clippy::type_complexity)]
    pub fn get_sections(&self, screen_width: f32, screen_height: f32) -> Result<(Vec<TextSection>, Vec<TextSection>, Vec<TextSection>, Vec<TextSection>), PError>{
        let mut sections_a_t = Vec::new();
        let mut sections_a_b = Vec::new();
        let mut sections_b_t = Vec::new();
        let mut sections_b_b = Vec::new();
        for (id, text) in self.text.iter(){
            match punwrap!(self.text_font_lookup.get(id), "could not find a font for text with id {} with text {}", id, text.text){
                Font::A => {
                    sections_a_t.push(text.get_section(self, screen_width, screen_height, 0.0, 0.0).clone());
                },
                Font::B => {
                    sections_b_t.push(text.get_section(self, screen_width, screen_height, 0.0, 0.0).clone());
                }
            }
        }
        for (id, text) in self.world_text.iter(){
            match punwrap!(self.world_text_font_lookup.get(id), "could not find a font for world text with id {} with text {}", id, text.text){
                Font::A => {
                    sections_a_b.push(text.get_section(self, screen_width, screen_height, self.camera_x * -1.0, self.camera_y * -1.0).clone());
                },
                Font::B => {
                    sections_b_b.push(text.get_section(self, screen_width, screen_height, self.camera_x * -1.0, self.camera_y * -1.0).clone());
                }
            }
        }
        Ok((sections_a_t, sections_a_b, sections_b_t, sections_b_b))
    }
}
