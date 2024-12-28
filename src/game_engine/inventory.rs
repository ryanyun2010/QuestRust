use std::{borrow::Borrow, cell::RefCell};

use super::{item::Item, world::{World}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemType {
    MeleeWeapon,
    RangedWeapon,
    MagicWeapon,
    Shield,
    HelmetArmor,
    ChestplateArmor,
    LeggingsArmor,
    BootsArmor,
    //I'll add accesories, but I have not implemented them yet.
    //These are basically the same as trinket and baubles for now. Might 
    BaubleRing, //Two of these by default
    BaubleCrown,
    BaubleNecklace,
    BaubleBelt,
    BaubleBack,
    BaubleBody,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemContainerPointer {
    element_id: RefCell<Option<usize>>,
    container_type: Option<Vec<ItemType>>,
}
impl ItemContainerPointer {
    pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
        Self {
            container_type,
            element_id: RefCell::new(None)
        }
    }
    pub fn is_init(&self) -> bool {
        self.element_id.borrow().is_some()
    }
}
//We just store a single item in each container.
//Containers, not to be confused with chests (if we implement them), or items (which are single, dynamic, inventory slots).
#[derive(Clone, Debug)]
pub struct ItemContainer {
    // element_id: usize,
    container_type: Option<Vec<ItemType>>,
    contained_item: Option<Item>,
}
impl ItemContainer {
    pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
        Self {
            container_type,
            contained_item: None
        }
    }
    pub fn tansfer_item(mut self, mut other: ItemContainer) -> ItemContainer {
        other.contained_item = self.contained_item;
        self.contained_item = None;
        println!("Self: {:?}\nOther: {:?}", self.contained_item, other.contained_item);
        other
    }
    pub fn drop(mut self) {
        //Spawn entity shit, will be used for resizing inventory and closing GUIs.
    }
}
#[derive(Clone, Debug)]
pub enum Hotbar {
    Normal([ItemContainerPointer; 6]),
    Extended([[ItemContainerPointer; 3]; 3]),
}
impl World {
    pub fn item_container_init(&self, mut container: &ItemContainerPointer) -> usize {
        self.item_containers.borrow_mut().insert(self.element_id, ItemContainer::new(container.container_type.clone()));
        *container.element_id.borrow_mut() = Some(self.element_id);
        self.element_id
    }
    pub fn player_init(&mut self) {
        for i in 0..6 {
            for j in 0..6 {
                self.element_id=self.item_container_init(&self.player.borrow_mut().inventory[i][j])+1;
            }
        }
    }
}