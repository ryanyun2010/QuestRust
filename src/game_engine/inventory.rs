use super::{item::Item, world::{self, World}};

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
    element_id: Option<usize>,
    container_type: Option<Vec<ItemType>>,
}
impl ItemContainerPointer {
    pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
        Self {
            container_type,
            element_id: None
        }
    }
    pub fn is_init(&self) -> bool {
        self.element_id.is_some()
    }
    pub fn init(&mut self, world: &mut World) -> usize {
        world.element_id+=1;
        world.item_containers.borrow_mut().insert(world.element_id-1, ItemContainer::new(self.container_type.clone()));
        self.element_id = Some(world.element_id-1);
        world.element_id-1
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
    Normal([ItemContainer; 6]),
    Extended([[ItemContainer; 3]; 3]),
}