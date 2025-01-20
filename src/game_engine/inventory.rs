use std::collections::HashMap;

use wgpu::rwh::XcbDisplayHandle;

use crate::{game_engine::item::Item, rendering_engine::abstractions::{Sprite, SpriteContainer}};

use super::{camera::{self, Camera}, ui::UIElementDescriptor};
#[derive(Debug)]
pub struct Inventory {
    pub items: HashMap<usize, Item>,
    pub hotbar: Vec<Slot>,
    pub cur_hotbar_slot: usize,
    pub slots: Vec<Slot>,
    item_id: usize,
    pub display_text: Option<usize>
}
#[derive(Debug, Clone)]
pub struct Slot {
    pub item: Option<usize>,
    pub main_image_ui_id: usize,
    pub item_image_ui_id: usize,
    pub x: usize,
    pub y: usize
}

impl Slot {
    pub fn create_with_ui(x: usize, y: usize, camera: &mut Camera, sprites: &SpriteContainer) -> Self {
        let slot_ui = camera.add_ui_element(format!("hslot:{}:{}",x,y), UIElementDescriptor {
                x: x as f32,
                y: y as f32,
                z: 1.0,
                width: 48.0,
                height: 48.0,
                sprite_id: sprites.get_sprite_id("hslot").expect("couldn't find hotbar sprite"),
                visible: true
            });
        let item_ui = camera.add_ui_element(format!("hslotitem:{}:{}",x,y), 
            UIElementDescriptor {
                x: x as f32 + 8.0,
                y: y as f32 + 8.0,
                z: 3.0,
                width: 32.0,
                height: 32.0,
                sprite_id: 0,
                visible: false
            }
        );
        Self {
            item: None,
            main_image_ui_id: slot_ui,
            item_image_ui_id: item_ui,
            x: x,
            y: y
        }
    }
}

impl Inventory{
    pub fn new() -> Self {
        Self {
            hotbar: Vec::new(),
            cur_hotbar_slot: 0,
            display_text: None,
            items: HashMap::new(),
            item_id: 0,
            slots: Vec::new()
        }
    }
    pub fn add_item(&mut self, item: Item) -> usize {
        self.items.insert(
            self.item_id, item
        );
        self.item_id += 1;
        return self.item_id - 1;
    }
    pub fn set_hotbar_slot(&mut self, slot: usize) {
        self.cur_hotbar_slot = slot;
    }
    pub fn init_ui(&mut self, camera: &mut Camera, sprites: &SpriteContainer) {
        self.hotbar.push(Slot::create_with_ui(20, 652, camera, sprites));
        self.hotbar.push(Slot::create_with_ui(78, 652, camera, sprites));
        self.hotbar.push(Slot::create_with_ui(136, 652, camera, sprites));
        self.hotbar.push(Slot::create_with_ui(194, 652, camera, sprites));
        self.hotbar.push(Slot::create_with_ui(252, 652, camera, sprites));
        camera.add_ui_element(String::from("hhslot"), UIElementDescriptor {
            x: 20.0,
            y: 652.0,
            z: 2.0,
            width: 48.0,
            height: 48.0,
            sprite_id: sprites.get_sprite_id("slot_highlight").expect("couldn't find hotbar highlight sprite"),
            visible: true
        });
    }
    pub fn set_hotbar_slot_item(&mut self, slot: usize, item_id: usize) {
        let slot_potentially = self.hotbar.get_mut(slot);
        if slot_potentially.is_some() {
            slot_potentially.unwrap().item = Some(item_id);
        }
    }
    pub fn update_ui(&mut self, camera: &mut Camera, sprites: &SpriteContainer) {
        camera.get_ui_element_mut_by_name(String::from("hhslot")).unwrap().x  = self.cur_hotbar_slot as f32 * 58 as f32 + 20.0;
        let cur_item = self.get_cur_held_item();
        if cur_item.is_some(){
            let mut text = format!(
                "{}\n----------------------------------------\n\n{}\n\n", cur_item.unwrap().name, cur_item.unwrap().lore
            );
            let stats = &cur_item.unwrap().stats;
            for stat in stats.into_iter() {
                if stat.1.is_some(){
                    text.push_str(
                        format!("{}: {} \n", stat.0, stat.1.unwrap()).as_str()
                    );
                }
            }
            if self.display_text.is_none(){
                self.display_text = Some(camera.add_text(text, super::camera::Font::A, 40.0, 390.0, 130.0, 250.0, 20.0, [1.0, 1.0, 1.0, 1.0], wgpu_text::glyph_brush::HorizontalAlign::Left));
            }else{
                camera.get_text_mut(self.display_text.unwrap()).unwrap().text = text;
            }
        }

        for slot in self.hotbar.iter() {
            if slot.item.is_some() {
                let item = self.get_item(&slot.item.unwrap()).expect("Slot refers to a non-existent item?");
                let i = camera.get_ui_element_mut(slot.item_image_ui_id);
                i.sprite_id = sprites.get_sprite_id(&item.sprite).expect("Item refers to a non-existent sprite?");
                i.visible = true;
            }else {
                let i = camera.get_ui_element_mut(slot.item_image_ui_id);
                i.visible = false;
            }
        }
        
    }
    pub fn get_cur_held_item(&self) -> Option<&Item> {
        self.hotbar
            .get(self.cur_hotbar_slot)
            .and_then(|slot| slot.item)
            .and_then(|item| self.get_item(&item))
    }
    pub fn get_item(&self, id: &usize) -> Option<&Item> {
        self.items.get(id)
    }
}




// use std::{borrow::Borrow, cell::RefCell};

// use super::{item::Item, world::{World}};

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ItemContainerPointer {
//     element_id: RefCell<Option<usize>>,
//     container_type: Option<Vec<ItemType>>,
// }
// impl ItemContainerPointer {
//     pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
//         Self {
//             container_type,
//             element_id: RefCell::new(None)
//         }
//     }
//     pub fn is_init(&self) -> bool {
//         self.element_id.borrow().is_some()
//     }
// }
// //We just store a single item in each container.
// //Containers, not to be confused with chests (if we implement them), or items (which are single, dynamic, inventory slots).
// #[derive(Clone, Debug)]
// pub struct ItemContainer {
//     // element_id: usize,
//     container_type: Option<Vec<ItemType>>,
//     contained_item: Option<Item>,
// }
// impl ItemContainer {
//     pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
//         Self {
//             container_type,
//             contained_item: None
//         }
//     }
//     pub fn tansfer_item(mut self, mut other: ItemContainer) -> ItemContainer {
//         other.contained_item = self.contained_item;
//         self.contained_item = None;
//         println!("Self: {:?}\nOther: {:?}", self.contained_item, other.contained_item);
//         other
//     }
//     pub fn drop(mut self) {
//         //Spawn entity shit, will be used for resizing inventory and closing GUIs.
//     }
// }
// #[derive(Clone, Debug)]
// pub enum Hotbar {
//     Normal([ItemContainerPointer; 6]),
//     Extended([[ItemContainerPointer; 3]; 3]),
// }
// impl World {
//     pub fn item_container_init(&self, mut container: &ItemContainerPointer) -> usize {
//         self.item_containers.borrow_mut().insert(self.element_id, ItemContainer::new(container.container_type.clone()));
//         *container.element_id.borrow_mut() = Some(self.element_id);
//         self.element_id
//     }
//     pub fn player_init(&mut self) {
//         for i in 0..6 {
//             for j in 0..6 {
//                 self.element_id=self.item_container_init(&self.player.borrow_mut().inventory[i][j])+1;
//             }
//         }
//     }
// }