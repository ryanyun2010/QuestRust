use crate::world::World;
use crate::world::RenderData;

pub struct Camera{
    pub viewpoint_width: usize,
    pub viewpoint_height: usize,
    pub camera_x: usize, // top left corner of the camera in world/element coordinates
    pub camera_y: usize,
}


impl Camera{
    pub fn new(viewpoint_width:usize, viewpoint_height:usize) -> Self{
        Self{
            viewpoint_width: viewpoint_width,
            viewpoint_height: viewpoint_height,
            camera_x: 0,
            camera_y: 0,
        }
    }
    pub fn render(&mut self, world: &World) -> RenderData{
        if world.player.x > self.viewpoint_width / 2{
            self.camera_x = world.player.x - self.viewpoint_width / 2;
        }else{
            self.camera_x = 0;
        }   
        println!("{:?} {:?}", world.player.y, self.viewpoint_height / 2);

        if world.player.y > self.viewpoint_height / 2{
            self.camera_y = world.player.y - self.viewpoint_height / 2;
        }else{
            self.camera_y = 0;
        }
        let mut render_data = RenderData::new();
        let camera_left_chunk_x = World::coord_to_chunk_coord(self.camera_x);
        let camera_right_chunk_x = World::coord_to_chunk_coord(self.camera_x + self.viewpoint_width);

        let camera_top_chunk_y = World::coord_to_chunk_coord(self.camera_y);
        let camera_bot_chunk_y = World::coord_to_chunk_coord(self.camera_y + self.viewpoint_height); 

        for x in camera_left_chunk_x..camera_right_chunk_x{
            for y in camera_top_chunk_y..camera_bot_chunk_y{
                let chunk_id = world.get_chunk_from_xy(x,y);
                if chunk_id.is_none(){
                    continue;
                }
                let chunk = &world.chunks[chunk_id.unwrap()];
                for terrain_id in chunk.terrain_ids.iter(){
                    let potentially_sprite_id = world.get_sprite(chunk.terrain[*terrain_id].element_id);
                    if potentially_sprite_id.is_none(){
                        continue;
                    }
                    let sprite_id = potentially_sprite_id.unwrap();
                    let sprite = &world.sprites[sprite_id];

                    let index_offset = render_data.vertex.len() as u16;
                    let vertex_offset_x = -1 * self.camera_x as i32;
                    let vertex_offset_y = -1 * self.camera_y as i32;

                    let draw_data = sprite.draw_data(chunk.terrain[*terrain_id].x, chunk.terrain[*terrain_id].y, 32, 32, self.viewpoint_width, self.viewpoint_height, index_offset, vertex_offset_x, vertex_offset_y);
                    
                    render_data.vertex.extend(draw_data.vertex);
                    render_data.index.extend(draw_data.index);
                }
            }
        }
        render_data
    }
}