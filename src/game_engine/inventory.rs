use crate::game_engine::item::Item;

use super::camera::Camera;
#[derive(Debug)]
pub struct Inventory {
    pub hotbar: Vec<Item>,
    pub cur_hotbar_slot: usize,
}

impl Inventory{
    pub fn new() -> Self {
        Self {
            hotbar: Vec::new(),
            cur_hotbar_slot: 0,
        }
    }
    pub fn set_hotbar_slot(&mut self, slot: usize) {
        self.cur_hotbar_slot = slot;
    }
    pub fn update_ui(&self, camera: &mut Camera) {
        camera.get_ui_element_mut_by_name(String::from("hhslot")).unwrap().x  = self.cur_hotbar_slot as f32 * 58 as f32 + 20.0;
    }
    pub fn get_cur_held_item(&self) -> Option<&Item> {
        return self.hotbar.get(self.cur_hotbar_slot);
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