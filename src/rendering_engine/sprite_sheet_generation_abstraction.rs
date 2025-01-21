use std::{collections::HashMap, env};

use crate::game_engine::json_parsing::sprite_sheet_json;

use super::abstractions::SpriteSheet;

pub struct SpriteSheetSheet {
    pub sheets: Vec<SpriteSheet>,
    pub path: String
}
const COMBINE_PATH: &str = "src/rendering_engine/img/COMBINED_AUTO_GENERATED.png";
impl SpriteSheetSheet{
    pub fn create_from_json(sheets: &Vec<sprite_sheet_json>, texture_id: i32) -> Self {
        let args: Vec<String> = env::args().collect();
        if args.contains(&String::from("combine")) {
            let mut sheet_paths = Vec::new();
            for sheet in sheets.iter() {
                sheet_paths.push(sheet.path.clone());
            }
            combine_images(sheet_paths, COMBINE_PATH).expect("Couldn't combine images, is one of the sprite/sprite_sheet paths wrong?");
        }
        let mut sprite_sheets = Vec::new();
        let mut x_offset = 0;
        let mut sprite_lookup = HashMap::new();
        let mut real_height = 0;
        for sheet in sheets.iter() {
            real_height = real_height.max(sheet.height);
        }
        let mut total_width = 0;
        for sheet in sheets.iter() {
            total_width += sheet.width;
        }
        for sheet in sheets.iter() {
            sprite_sheets.push(SpriteSheet{
                texture_id: texture_id,
                total_width: total_width,
                width: sheet.width,
                x_offset: x_offset,
                height: real_height,
                sprite_width: sheet.sprite_width,
                sprite_height: sheet.sprite_height,
                sprite_padding: sheet.sprite_padding
            });
            x_offset += sheet.width;
            for sprite in sheet.sprites.iter() {
                sprite_lookup.insert(sprite.name.clone(), sprite_sheets.len() - 1);
            }
        }


        Self {
            sheets: sprite_sheets,
            path: COMBINE_PATH.to_string()
        }
    }
    

}

use image::{GenericImageView, RgbaImage};

pub fn combine_images(image_paths: Vec<String>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_width = 0;
    let mut max_height = 0;
    let mut images = Vec::new();

    for path in image_paths {
        let img = image::open(&path)?;
        let (width, height) = img.dimensions();
        total_width += width;
        max_height = max_height.max(height);
        images.push(img);
    }
    let mut combined_img = RgbaImage::new(total_width, max_height);

    let mut x_offset = 0;
    for img in images {
        let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                combined_img.put_pixel(x + x_offset, y, pixel);
            }
        }

        x_offset += width;
    }

    combined_img.save(output_path)?;

    Ok(())
}

