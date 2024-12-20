use std::collections::HashMap;
use std::hash::Hash;

use crate::camera;
use crate::world::World;
use crate::world::RenderData;

pub struct Camera{
    pub viewpoint_width: usize,
    pub viewpoint_height: usize,
    pub camera_x: f32, // top left corner of the camera in world/element coordinates
    pub camera_y: f32,
    ui_elements: Vec<crate::game_engine::ui::UIElement>, // vec element i should be element with id i
    ui_element_names: HashMap<String, usize>, // map names to ids
    ui_element_id: usize,
    velocity: [isize; 2]
}

impl Camera{
    pub fn new(viewpoint_width:usize, viewpoint_height:usize) -> Self{
        Self{
            viewpoint_width: viewpoint_width,
            viewpoint_height: viewpoint_height,
            camera_x: 20.0,
            camera_y: 40.0,
            ui_elements: Vec::new(),
            ui_element_names: HashMap::new(),
            ui_element_id: 0,
            velocity: [0,0]
        }
    }
    pub fn update_ui(&mut self, world: &mut World){
        let player = world.player.borrow().clone();
        let health_bar = self.get_ui_element_mut(self.get_element_by_name(String::from("health_bar_inside")).unwrap());
        let health_bar_width = f32::max(0.0, (player.health as f32 / player.max_health as f32) * 250.0);
        health_bar.width = health_bar_width;
    }
    pub fn add_ui_element(&mut self, name: String,  element: crate::game_engine::ui::UIElement) -> usize{
        self.ui_elements.insert(self.ui_element_id, element);
        self.ui_element_names.insert(name, self.ui_element_id);
        self.ui_element_id += 1;
        self.ui_element_id - 1
    }
    pub fn get_element_by_name(&self, name: String) -> Option<usize>{
        self.ui_element_names.get(&name).copied()
    }
    pub fn get_ui_element_mut(&mut self, id: usize) -> &mut crate::game_engine::ui::UIElement{
        &mut self.ui_elements[id]
    }
    pub fn update_camera_position(&mut self, world: &World){
        let player = world.player.borrow().clone();
        let mut direction = [player.x - (self.viewpoint_width / 2) as f32 - self.camera_x, player.y - (self.viewpoint_height / 2) as f32 - self.camera_y];
        if self.camera_x < 4.0 && direction[0] < 0.0{
            direction[0] = 0.0;
        }
        if self.camera_y < 4.0 && direction[1] < 0.0{
            direction[1] = 0.0;
        }

        
        let magnitude = (direction[0].powi(2) + direction[1].powi(2)).sqrt();

        
        if magnitude < 6.0{
            return;
        }
        self.camera_x += (direction[0]/magnitude * 3.0).round();
        self.camera_y += (direction[1]/magnitude * 3.0).round();

        if self.camera_x < 0.0{
            self.camera_x = 0.0;
        }
        if self.camera_y < 0.0{
            self.camera_y = 0.0;
        }
    }
    pub fn render(&mut self, world: &mut World) -> RenderData{
        let mut render_data = RenderData::new();
        let mut terrain_data: RenderData = RenderData::new();
        let mut entity_data: RenderData = RenderData::new();
        let mut index_offset: u16 = 0;
        let player = world.player.borrow().clone(); 


        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
        let mut camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize) + 1;

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
        let mut camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize) + 1; 

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

        let player_draw_data = player.draw_data(self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
        render_data.vertex.extend(player_draw_data.vertex);
        render_data.index.extend(player_draw_data.index);
        world.set_loaded_chunks(chunks_loaded);
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