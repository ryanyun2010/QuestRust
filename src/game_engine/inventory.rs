use super::item::Item;

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
//We just store a single item in each container.
//Containers, not to be confused with chests (if we implement them), or falling items, are single, dynamic, inventory slots.
#[derive(Clone, Debug)]
pub struct ItemContainer {
    element_id: usize,
    container_type: Option<Vec<ItemType>>,
    contained_item: Option<Item>,
}
impl ItemContainer {
    
}