use crate::camera;
use crate::world::World;
use crate::world::RenderData;

pub struct Camera{
    pub viewpoint_width: usize,
    pub viewpoint_height: usize,
    pub camera_x: f32, // top left corner of the camera in world/element coordinates
    pub camera_y: f32,
}


impl Camera{
    pub fn new(viewpoint_width:usize, viewpoint_height:usize) -> Self{
        Self{
            viewpoint_width: viewpoint_width,
            viewpoint_height: viewpoint_height,
            camera_x: 0.0,
            camera_y: 0.0,
        }
    }
    pub fn render(&mut self, world: &mut World) -> RenderData{

        let player = world.player.borrow().clone();
        if player.x > (self.viewpoint_width / 2) as f32{
            self.camera_x = player.x - (self.viewpoint_width / 2) as f32;
        }else{
            self.camera_x = 0.0;
        }   
        if player.y > (self.viewpoint_height / 2) as f32{
            self.camera_y = player.y - (self.viewpoint_height / 2) as f32;
        }else{
            self.camera_y = 0.0;
        }
        let mut render_data = RenderData::new();
        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x.floor() as usize);
        let mut camera_right_chunk_x = World::coord_to_chunk_coord((self.camera_x + self.viewpoint_width as f32).floor() as usize);

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y.floor() as usize);
        let mut camera_bot_chunk_y = World::coord_to_chunk_coord((self.camera_y + self.viewpoint_height as f32).floor() as usize); 
       
        if camera_right_chunk_x - camera_left_chunk_x < 1{
            camera_right_chunk_x += 1;
        }
        if camera_bot_chunk_y - camera_top_chunk_y < 1{
            camera_bot_chunk_y += 1;
        }

        let mut chunks_loaded = Vec::new();
        
        for x in camera_left_chunk_x..camera_right_chunk_x{
            for y in camera_top_chunk_y..camera_bot_chunk_y{
                let chunk_id = world.get_chunk_from_xy(x,y);
                if chunk_id.is_none(){
                    continue;
                }
                let chunk = world.get_chunk_from_id(chunk_id.unwrap()).unwrap();
                chunks_loaded.push(chunk_id.unwrap());
                for terrain_id in chunk.terrain_ids.iter(){
                    let potentially_sprite_id = world.get_sprite(*terrain_id);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &world.sprites[sprite_id];

                    let index_offset = render_data.vertex.len() as u16;
                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;
                    let terrain = world.get_terrain(*terrain_id).unwrap();
                    let draw_data = sprite.draw_data(terrain.x as f32, terrain.y as f32, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
                    
                    render_data.vertex.extend(draw_data.vertex);
                    render_data.index.extend(draw_data.index);
                }

                for entity_id in chunk.entities_ids.iter(){
                    let potentially_sprite_id = world.get_sprite(*entity_id);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &world.sprites[sprite_id];

                    let index_offset = render_data.vertex.len() as u16;
                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;

                    let entity = world.get_entity(*entity_id).unwrap();

                    let draw_data = sprite.draw_data(entity.x, entity.y, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
                    
                    render_data.vertex.extend(draw_data.vertex);
                    render_data.index.extend(draw_data.index);
                }
            }
        }

        let player_draw_data = player.draw_data(self.viewpoint_width, self.viewpoint_height, render_data.vertex.len() as u16, -1 * self.camera_x as i32, -1 * self.camera_y as i32);
        render_data.vertex.extend(player_draw_data.vertex);
        render_data.index.extend(player_draw_data.index);
        world.set_loaded_chunks(chunks_loaded);
        render_data
    }
}