use crate::{rendering_engine::abstractions::RenderData, rendering_engine::vertex::Vertex};
#[derive(Clone, Copy, Debug)]
pub struct UIElement{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture_id: i32,
    pub visible: bool,
}

impl UIElement{
    pub fn draw_data(&self, screen_w: usize, screen_h: usize, index_offset: u16) -> RenderData{
        let screen_to_render_ratio_x = 2.0 / screen_w as f32;
        let screen_to_render_ratio_y = 2.0 / screen_h as f32;

        let w = self.width * screen_to_render_ratio_x;
        let h = self.height * screen_to_render_ratio_y;

        let x = (self.x) * screen_to_render_ratio_x - 1.0;
        let y = -1.0 * ((self.y) * screen_to_render_ratio_y - 1.0) - h;

        let vertex = vec![
            Vertex { position: [x, y, 0.0], tex_coords: [0.0, 1.0], index: self.texture_id as i32 },
            Vertex { position: [x + w, y, 0.0], tex_coords: [1.0, 1.0], index: self.texture_id as i32 },
            Vertex { position: [x + w, y + h, 0.0], tex_coords: [1.0, 0.0], index: self.texture_id as i32 },
            Vertex { position: [x, y + h, 0.0], tex_coords: [0.0, 0.0], index: self.texture_id as i32 },
        ];

        let index = vec![0 + index_offset, 1 + index_offset, 2 + index_offset, 0 + index_offset, 2 + index_offset, 3 + index_offset];

        RenderData { vertex, index }
    }
}