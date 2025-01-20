use std::collections::HashMap;

use anyhow::anyhow;
use wgpu::rwh::XcbDisplayHandle;

use crate::{game_engine::item::Item, rendering_engine::abstractions::{Sprite, SpriteContainer}};

use super::{camera::{self, Camera}, game::MousePosition, ui::{UIESprite, UIElementDescriptor}};

#[derive(Debug, Clone)]
pub struct ItemOnMouse{
    pub item_id: usize,
    pub slot_belonging: usize
}
#[derive(Debug)]
pub struct Inventory {
    items: HashMap<usize, Item>,
    hotbar: Vec<Slot>,
    cur_hotbar_slot: usize,
    slots: Vec<Slot>,
    item_id: usize,
    display_text: Option<usize>,
    show_inventory: bool,
    item_on_mouse: Option<ItemOnMouse>,
    mouse_position: MousePosition
}
#[derive(Debug, Clone)]
struct Slot {
    item: Option<usize>,
    main_image: Option<UIESprite>,
    item_image: Option<UIESprite>,
    x: usize,
    y: usize
}

impl Slot {
    pub fn new(x: usize, y: usize) -> Self {

        Self {
            item: None,
            main_image: Some(UIESprite{
                x: x as f32,
                y: y as f32,
                z: 5.0,
                width: 48.0,
                height: 48.0,
                sprite: "hslot".to_string()
            }),
            item_image: None,
            x: x,
            y: y
        }
    }
    pub fn get_ui(&self) -> Vec<UIESprite>{
        let mut sprites = Vec::new();
        sprites.extend(self.main_image.clone());
        sprites.extend(self.item_image.clone());
        sprites
    }
    pub fn set_item(&mut self, item: usize, items: &HashMap<usize, Item>) -> Result<(), anyhow::Error>{
        let i = items.get(&item);
        if i.is_none() {
            return Err(anyhow!("couldn't find item with id {}", item))
        }
        self.item = Some(item);
        self.item_image = Some(
            UIESprite {
                x: self.x as f32 + 8.0,
                y: self.y as f32 + 8.0,
                z: 5.2,
                width: 32.0,
                height: 32.0,
                sprite: i.unwrap().sprite.clone()
            }
        );
        Ok(())
    }
    pub fn remove_item(&mut self) {
        self.item = None;
        self.item_image = None;
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
            slots: Vec::new(),
            show_inventory: false,
            item_on_mouse: None,
            mouse_position: MousePosition::default()
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
    pub fn init_ui(&mut self, sprites: &SpriteContainer) {
        self.hotbar.push(Slot::new(20, 652));
        self.hotbar.push(Slot::new(78, 652));
        self.hotbar.push(Slot::new(136, 652));
        self.hotbar.push(Slot::new(194, 652));
        self.hotbar.push(Slot::new(252, 652));
        self.slots.push(Slot::new(520, 200));
        self.slots.push(Slot::new(578, 200));
        self.slots.push(Slot::new(636, 200));
        self.slots.push(Slot::new(694, 200));
        self.slots.push(Slot::new(752, 200));
        println!("{:?}", self.set_slot_item(0, 0));
        println!("{:?}", self.set_slot_item(3, 1));
        println!("{:?}", self.slots[0]);
    }
    pub fn show_inventory(&mut self, camera: &mut Camera){
        self.show_inventory = true;
    }
    pub fn hide_inventory(&mut self, camera: &mut Camera){
        self.show_inventory = false;
        if self.item_on_mouse.is_some(){
            let iom = self.item_on_mouse.as_ref().unwrap();
            self.set_slot_item(iom.slot_belonging, iom.item_id);
            self.item_on_mouse = None;
        }
    }
    pub fn set_hotbar_slot_item(&mut self, slot: usize, item_id: usize) {
        let slot_potentially = self.hotbar.get_mut(slot);
        if slot_potentially.is_some() {
            slot_potentially.unwrap().set_item(item_id, &self.items);
        }
    }
    pub fn set_slot_item(&mut self, slot: usize, item_id: usize) -> Result<(), anyhow::Error> {
        let slot_potentially = self.slots.get_mut(slot);
        if slot_potentially.is_some() {
            slot_potentially.unwrap().set_item(item_id, &self.items)?;
        }
        Ok(())
    }
    pub fn render_ui(&mut self, sprites: &SpriteContainer) -> Vec<UIESprite> {
        let mut ui = Vec::new();
        if self.show_inventory {
            ui.push(UIESprite {
                z: 0.0,
                x: 326.0,
                y: 186.5,
                width: 500.0,
                height: 347.0,
                sprite: String::from("inventory")
            });
            ui.push(UIESprite {
                z: -1.0,
                x: 0.0,
                y: 0.0,
                width: 1152.0,
                height: 720.0,
                sprite: String::from("inventory_background")
            });

            for slot in self.slots.iter() {
                ui.extend(slot.get_ui());
            }

            if self.item_on_mouse.is_some() {
                let iom = self.item_on_mouse.as_ref().unwrap();
                let item = self.get_item(&iom.item_id);
                if item.is_some(){
                    ui.push(UIESprite {
                        x: self.mouse_position.x_screen - 12.0,
                        y: self.mouse_position.y_screen - 12.0,
                        z: 999.0,
                        width: 24.0,
                        height: 24.0,
                        sprite: item.unwrap().sprite.clone()
                    })
                }
            }
        }
        else {
            for slot in self.hotbar.iter() {
                ui.extend(slot.get_ui());
            }
            ui.push(
                UIESprite{
                    x: 20.0 + 58.0 * self.cur_hotbar_slot as f32,
                    y: 652.0,
                    z: 5.1,
                    width: 48.0,
                    height: 48.0,
                    sprite: String::from("slot_highlight")
                }
            )
        }
        ui
        



        // camera.get_ui_element_mut_by_name(String::from("inventory_background")).unwrap().visible = self.show_inventory;
        // camera.get_ui_element_mut_by_name(String::from("inventory_back_shade")).unwrap().visible = self.show_inventory; 
        // camera.get_ui_element_mut_by_name(String::from("hhslot")).unwrap().x  = self.cur_hotbar_slot as f32 * 58 as f32 + 20.0;
        // camera.get_ui_element_mut_by_name(String::from("hhslot")).unwrap().visible  = self.show_inventory;
        // let cur_item = self.get_cur_held_item();
        // if cur_item.is_some(){
        //     let mut text = format!(
        //         "{}\n----------------------------------------\n\n{}\n\n", cur_item.unwrap().name, cur_item.unwrap().lore
        //     );
        //     let stats = &cur_item.unwrap().stats;
        //     for stat in stats.into_iter() {
        //         if stat.1.is_some(){
        //             text.push_str(
        //                 format!("{}: {} \n", stat.0, stat.1.unwrap()).as_str()
        //             );
        //         }
        //     }
        //     if self.display_text.is_none(){
        //         self.display_text = Some(camera.add_text(text, super::camera::Font::A, 40.0, 390.0, 130.0, 250.0, 20.0, [1.0, 1.0, 1.0, 1.0], wgpu_text::glyph_brush::HorizontalAlign::Left));
        //     }else{
        //         camera.get_text_mut(self.display_text.unwrap()).unwrap().text = text;
        //     }
        // }

        // if self.item_on_mouse.is_some() {
        //     let item = self.item_on_mouse.as_ref().unwrap();
        //     let item_ui = camera.get_ui_element_mut_by_name(String::from("item_held")).unwrap();
        //     item_ui.visible = true;
        //     item_ui.sprite_id = sprites.get_sprite_id(self.get_item(&item.item_id).unwrap().sprite.as_str()).unwrap();
        //     item_ui.x = self.mouse_position.x_screen - 12.0;
        //     item_ui.y = self.mouse_position.y_screen - 12.0;
        // }else{
        //     let item_ui = camera.get_ui_element_mut_by_name(String::from("item_held")).unwrap();
        //     item_ui.visible = false;
        // }
        // if self.show_inventory {
        //     for slot in self.hotbar.iter() {
        //         camera.get_ui_element_mut(slot.main_image_ui_id).visible = false;
        //         camera.get_ui_element_mut(slot.item_image_ui_id).visible = false;
        //     }
        //     for slot in self.slots.iter() {
        //         camera.get_ui_element_mut(slot.main_image_ui_id).visible = true;
        //         if slot.item.is_some() {
        //             let item = self.get_item(&slot.item.unwrap()).expect("Slot refers to a non-existent item?");
        //             let i = camera.get_ui_element_mut(slot.item_image_ui_id);
        //             i.sprite_id = sprites.get_sprite_id(&item.sprite).expect("Item refers to a non-existent sprite?");
        //             i.visible = true;
        //         }else {
        //             let i = camera.get_ui_element_mut(slot.item_image_ui_id);
        //             i.visible = false;
        //         }
        //     }
        // }else{
        //     for slot in self.slots.iter() {
        //         camera.get_ui_element_mut(slot.main_image_ui_id).visible = false;
        //         camera.get_ui_element_mut(slot.item_image_ui_id).visible = false;
        //     }
        //     for slot in self.hotbar.iter() {
        //         camera.get_ui_element_mut(slot.main_image_ui_id).visible = true;
        //         if slot.item.is_some() {
        //             let item = self.get_item(&slot.item.unwrap()).expect("Slot refers to a non-existent item?");
        //             let i = camera.get_ui_element_mut(slot.item_image_ui_id);
        //             i.sprite_id = sprites.get_sprite_id(&item.sprite).expect("Item refers to a non-existent sprite?");
        //             i.visible = true;
        //         }else {
        //             let i = camera.get_ui_element_mut(slot.item_image_ui_id);
        //             i.visible = false;
        //         }
        //     }
        // }
        
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

    pub fn on_key_down(&mut self, key: &String) {

    }
    pub fn on_mouse_click(&mut self, position: MousePosition, left: bool, right: bool) {
        let mut slot_clicked = None;
        let mut i = 0;
        for slot in self.slots.iter_mut() {
            if slot.x < position.x_screen as usize && slot.x + 48 > position.x_screen as usize && slot.y < position.y_screen as usize && slot.y + 48 > position.y_screen as usize {
                slot_clicked = Some(slot);
                break;
            }
            i += 1;
        }
        if slot_clicked.is_some() {
            let slot = slot_clicked.unwrap();
            if slot.item.is_some() {
                if self.item_on_mouse.is_none() {
                    self.item_on_mouse = Some(
                        ItemOnMouse {
                            slot_belonging: i,
                            item_id: slot.item.unwrap()
                        }
                    );
                    slot.remove_item();
                }else if self.item_on_mouse.is_some() {
                    let item_clone = self.item_on_mouse.as_ref().unwrap().clone();
                    self.item_on_mouse = Some(
                        ItemOnMouse {
                            slot_belonging: item_clone.slot_belonging,
                            item_id: slot.item.unwrap()
                        }
                    ); 
                    slot.set_item(item_clone.item_id, &self.items);
                }
            }else if self.item_on_mouse.is_some(){
                slot.set_item(self.item_on_mouse.as_ref().unwrap().item_id, &self.items);
                self.item_on_mouse = None;
            }
        }
    }
    pub fn process_input(&mut self, keys: &HashMap<String, bool>){

    }
    pub fn process_mouse_input(&mut self, position: MousePosition, left: bool, right: bool){
        self.mouse_position = position;
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