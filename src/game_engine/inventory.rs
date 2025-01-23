use std::collections::HashMap;


use crate::{error::PError, game_engine::item::Item, perror, ptry, rendering_engine::abstractions::{TextSprite, UIEFull}};

use super::{game::MousePosition, ui::UIESprite};

#[derive(Debug, Clone)]
pub struct ItemOnMouse{
    pub item_id: usize,
    pub slot_belonging: usize
}
#[derive(Debug)]
pub struct Inventory {
    items: HashMap<usize, Item>,
    hotbar: Vec<usize>, // slot id of hotbar slots
    cur_hotbar_slot: usize,
    pub slots: Vec<Slot>,
    item_id: usize,
    pub show_inventory: bool,
    item_on_mouse: Option<ItemOnMouse>,
    mouse_position: MousePosition
}
#[derive(Debug, Clone)]
pub struct Slot {
    pub item: Option<usize>,
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
            x,
            y,
        }
    }
    pub fn get_ui(&self) -> Vec<UIESprite>{
        let mut sprites = Vec::new();
        sprites.extend(self.main_image.clone());
        sprites.extend(self.item_image.clone());
        sprites
    }
    pub fn alter_position(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
        if self.item_image.is_some(){
            let ii = self.item_image.as_mut().unwrap();
            ii.x = self.x as f32 + 8.0;
            ii.y = self.y as f32 + 8.0;
        }
        if self.main_image.is_some(){
            let mi = self.main_image.as_mut().unwrap();
            mi.x = self.x as f32;
            mi.y = self.y as f32;
        }
    }
    pub fn set_item(&mut self, item: usize, items: &HashMap<usize, Item>) -> Result<(), PError>{
        let i = items.get(&item);
        if i.is_none() {
            return Err(perror!(NotFound, "no item with id {}", item));
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

impl Default for Inventory{
    fn default() -> Self {
        Self {
            hotbar: Vec::new(),
            cur_hotbar_slot: 0,
            items: HashMap::new(),
            item_id: 0,
            slots: Vec::new(),
            show_inventory: false,
            item_on_mouse: None,
            mouse_position: MousePosition::default(),
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) -> usize {
        self.items.insert(
            self.item_id, item
        );
        self.item_id += 1;
        self.item_id - 1
    }
    pub fn set_hotbar_slot(&mut self, slot: usize) {
        self.cur_hotbar_slot = slot;
    }
    pub fn init_ui(&mut self) {
        self.add_hotbar_slot(Slot::new(20, 652));
        self.add_hotbar_slot(Slot::new(78, 652));
        self.add_hotbar_slot(Slot::new(136, 652));
        self.add_hotbar_slot(Slot::new(194, 652));
        self.add_hotbar_slot(Slot::new(252, 652));
        self.add_slot(Slot::new(520, 200));
        self.add_slot(Slot::new(578, 200));
        self.add_slot(Slot::new(636, 200));
        self.add_slot(Slot::new(694, 200));
        self.add_slot(Slot::new(752, 200));
    }
    pub fn add_hotbar_slot(&mut self, slot: Slot) {
        self.hotbar.push(self.slots.len());
        self.slots.push(slot);
    }
    pub fn add_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
    }
    pub fn get_hotbar_slot(&self, slot: usize) -> Option<&Slot>{
        self.hotbar.get(slot).and_then(
            |x| self.slots.get(*x)
        )
    }
    pub fn get_hotbar_slot_mut(&mut self, slot: usize) -> Option<&mut Slot>{
        self.hotbar.get(slot).and_then(
            |x| self.slots.get_mut(*x)
        )
    }
    pub fn show_inventory(&mut self){
        self.show_inventory = true;
        for i in 0..self.hotbar.len(){
            let slot = self.get_hotbar_slot_mut(i).unwrap();
            slot.alter_position(520 + i * 58, 380);
        }
    }
    pub fn hide_inventory(&mut self) -> Result<(), anyhow::Error>{
        self.show_inventory = false;
        if self.item_on_mouse.is_some(){
            let iom = self.item_on_mouse.as_ref().unwrap();
            self.set_slot_item(iom.slot_belonging, iom.item_id)?;
            self.item_on_mouse = None;
        }
        for i in 0..self.hotbar.len(){
            let slot = self.get_hotbar_slot_mut(i).unwrap();
            slot.alter_position(20 + i * 58, 652);
        }
        Ok(())
    }
    pub fn set_hotbar_slot_item(&mut self, slot: usize, item_id: usize) -> Result<(), PError> {
        let slot_potentially = self.hotbar.get(slot).and_then(
            |x| self.slots.get_mut(*x)
        );
        if slot_potentially.is_some() {
            ptry!(slot_potentially.unwrap().set_item(item_id, &self.items), "While setting hotbar slot {} to item {}", slot, item_id);
        }
        Ok(())
    }
    pub fn get_slot(&self, slot: &usize) -> Option<&Slot> {
        self.slots.get(*slot)
    }
    pub fn add_to_slot(&mut self, item: Item) -> Result<(), PError> {
        let it = self.add_item(item);
        for slot in self.slots.iter_mut() {
            if slot.item.is_none() {
                ptry!(slot.set_item(it, &self.items));
                return Ok(());
            }
        }
        Err(perror!(NoSpace, "No space for item"))
    }
    pub fn set_slot_item(&mut self, slot: usize, item_id: usize) -> Result<(), anyhow::Error> {
        let slot_potentially = self.slots.get_mut(slot);
        if slot_potentially.is_some() {
            slot_potentially.unwrap().set_item(item_id, &self.items)?;
        }
        Ok(())
    }
    pub fn render_ui(&mut self) -> UIEFull {
        let mut ui = Vec::new();
        let mut text = Vec::new(); 
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
                if (slot.x as f32) < self.mouse_position.x_screen && (slot.x as f32 + 48.0) > self.mouse_position.x_screen && (slot.y as f32) < self.mouse_position.y_screen && (slot.y as f32 + 48.0) > self.mouse_position.y_screen{
                    if let Some(i) = slot.item{
                        let item = self.get_item(&i).unwrap();
                        let mut t = format!(
                            "{}\n----------------------------------------\n\n{}\n\n", item.name, item.lore
                        );
                        let stats = &item.stats;
                        for stat in stats.into_iter() {
                            if stat.1.is_some(){
                                t.push_str(
                                    format!("{}: {} \n", stat.0, stat.1.unwrap()).as_str()
                                );
                            }
                        }
                        ui.push(
                            UIESprite {
                                x: self.mouse_position.x_screen + 20.0,
                                y: self.mouse_position.y_screen - 165.0, 
                                z: 5.6,
                                width: 220.0,
                                height: 320.0,
                                sprite: String::from("level_editor_menu_background")
                            }
                        );
                        text.push(
                            TextSprite {
                                text: t,
                                font_size: 20.0,
                                x: self.mouse_position.x_screen + 30.0,
                                y: self.mouse_position.y_screen - 150.0,
                                w: 200.0,
                                h: 300.0,
                                color: [1.0,1.0,1.0,1.0],
                                align: wgpu_text::glyph_brush::HorizontalAlign::Left
                            }
                        )
                    }
                }
            }

            if self.item_on_mouse.is_some() {
                let iom = self.item_on_mouse.as_ref().unwrap();
                let item = self.get_item(&iom.item_id);
                if let Some(item) = item{
                    ui.push(UIESprite {
                        x: self.mouse_position.x_screen - 12.0,
                        y: self.mouse_position.y_screen - 12.0,
                        z: 999.0,
                        width: 24.0,
                        height: 24.0,
                        sprite: item.sprite.clone()
                    })
                }
            }
        }
        else {
            for slot in self.hotbar.iter() {
                ui.extend(self.get_slot(slot).unwrap().get_ui());
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
        UIEFull {
            sprites: ui,
            text,
        }
        
        
    }
    pub fn get_cur_held_item(&self) -> Option<&Item> {
        self.get_hotbar_slot(self.cur_hotbar_slot)
            .and_then(|slot| slot.item)
            .and_then(|item| self.get_item(&item))
    }
    pub fn get_item(&self, id: &usize) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn on_key_down(&mut self, key: &str) {

    }
    pub fn on_mouse_click(&mut self, position: MousePosition, left: bool, right: bool) {
        if left {
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
                        match slot.set_item(item_clone.item_id, &self.items) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error setting item: {:?}", e)
                            }
                        }
                    }
                }else if self.item_on_mouse.is_some(){
                    match slot.set_item(self.item_on_mouse.as_ref().unwrap().item_id, &self.items){
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error setting item: {:?}", e)
                        }
                    }
                    self.item_on_mouse = None;
                }
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