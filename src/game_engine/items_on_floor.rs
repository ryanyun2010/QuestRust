
use compact_str::CompactString;

use crate::rendering_engine::abstractions::{TextSprite, UIEFull};

use super::{item::Item, ui::UIESprite};

#[derive(Clone, Debug)]
pub struct ItemOnFloor {
    pub x: f32,
    pub y: f32,
    pub item: Item
}

impl ItemOnFloor {
    pub fn display(&self) -> UIEFull{
        let mut sprites: Vec<UIESprite> = Vec::new();
        let mut text = Vec::new();
        if self.item.name.clone().chars().count() > 12 {
        sprites.push(
            UIESprite {
                x: self.x + 20.0,
                y: self.y - 15.5, 
                z: 5.6,
                width: 150.0,
                height: 20.0,
                sprite: CompactString::from("level_editor_menu_background")
            }
        );
        text.push(
            TextSprite {
                text: self.item.name.clone().to_string(),
                font_size: 23.0,
                x: self.x + 85.0,
                y: self.y - 12.0,
                w: 145.0,
                h: 16.0,
                color: [1.0,1.0,1.0,1.0],
                align: wgpu_text::glyph_brush::HorizontalAlign::Center
            }
        );
        }else {
        sprites.push(
            UIESprite {
                x: self.x + 20.0,
                y: self.y - 15.5, 
                z: 5.6,
                width: 70.0,
                height: 20.0,
                sprite: CompactString::from("level_editor_menu_background")
            }
        );
        text.push(
            TextSprite {
                text: self.item.name.clone().to_string(),
                font_size: 23.0,
                x: self.x + 55.0,
                y: self.y - 12.0,
                w: 65.0,
                h: 16.0,
                color: [1.0,1.0,1.0,1.0],
                align: wgpu_text::glyph_brush::HorizontalAlign::Center
            }
        );


        } 

        UIEFull {
            sprites,
            text
        }
    }
}
