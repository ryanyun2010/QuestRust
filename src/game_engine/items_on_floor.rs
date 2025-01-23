use crate::rendering_engine::abstractions::{TextSprite, UIEFull};

use super::{item::Item, ui::UIESprite};

#[derive(Clone, Debug)]
pub struct ItemOnFloor {
    pub x: f32,
    pub y: f32,
    pub item: Item
}

impl ItemOnFloor {
    // pub fn show_description(&self) -> UIEFull{
    //     let mut sprites: Vec<UIESprite> = Vec::new();
    //     let mut text = Vec::new();
    //     let mut t = format!(
    //         "{}\n----------------------------------------\n\n{}\n\n", self.item.name, self.item.lore
    //     );
    //     let stats = &self.item.stats;
    //     for stat in stats.into_iter() {
    //         if stat.1.is_some(){
    //             t.push_str(
    //                 format!("{}: {} \n", stat.0, stat.1.unwrap()).as_str()
    //             );
    //         }
    //     }
    //     sprites.push(
    //         UIESprite {
    //             x: self.x + 20.0,
    //             y: self.y - 82.5, 
    //             z: 5.6,
    //             width: 110.0,
    //             height: 160.0,
    //             sprite: String::from("level_editor_menu_background")
    //         }
    //     );
    //     text.push(
    //         TextSprite {
    //             text: t,
    //             font_size: 14.0,
    //             x: self.x + 30.0,
    //             y: self.y - 75.0,
    //             w: 100.0,
    //             h: 140.0,
    //             color: [1.0,1.0,1.0,1.0],
    //             align: wgpu_text::glyph_brush::HorizontalAlign::Left
    //         }
    //     );

    //     UIEFull {
    //         sprites,
    //         text
    //     }
    // }
}