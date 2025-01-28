// /*
// BASIC INFORMATION:
// Non-unique items such as materials will rely on tags except for current stack size, which is a component..
// Unique items such as weapons, armor, baubles, etc. will have tags to define their basic behaviour but will also have components for unique attributes.
// Items give stats to the player. Entities and Terrain can have items (specifically the Item Entity type) but they will not be able to use them.
// */
// use std::os::macos::raw::stat;
// use crate::game_engine::entities::AttackType;
// use super::{inventory::ItemType, loot::Rarity, stat::GearStat};
// use crate::rendering_engine::abstractions::Sprite;
// //Each item when stored will be exactly like this. The item_type_id will be used in a lookup to find the tags.
// //They also have a Vec of components that will be largely unique. It's stackability will not be judged by the components, but instead by the tags.
// #[derive(Clone, Debug)]
// pub struct Item {
//     item_type_id: usize, //Where tag lookup is stored
//     component_list: Vec<ItemComponent>, //Components are stored here
// }
// //---THIS IS WHERE TAGS START---
// #[derive(Clone, Debug)]
// pub enum ItemTags {
//     ItemType(ItemType),
//     Name(String),
//     Sprite(String), //No Shit.
//     Lore(String), //Description.
//     Weapon(WeaponTag), //Weapon Stuff.
//     // Place(PlaceTerrain), //I don't think we need this.
//     Use(UseItem), //For special uses like food and maybe things like permenant consumables. This is largely for edge cases.
//     UseRange(f32), //The range in which this item can be used. Mostly cosmetic, but for uprgrading things in your base, it can be useful.
// }
// #[derive(Clone, Debug)]
// pub enum UseItem {
//     Eat(Food)
// }
// //Food is basically our equivalent of potions.
// //Nobody likes brewing.
// #[derive(Clone, Debug)]
// pub struct Food {
//     eating_time: f64,
//     effects: Option<Vec<StatusEffect>>
// }
// //Might need Ryan's help for implementing the StatusEffect updating system. Only players can have these because yes.
// //If entities can have this, let's make a simpler version made specifically for entities. Entities can't really use items and have different stat systems so...
// #[derive(Clone, Debug)]
// pub struct StatusEffect {
//     duration: Option<f64>,
//     start: usize, //What type is time measured in?
//     effects: StatusEffectType
// }
// impl StatusEffect {
//     pub fn new(duration: Option<f64>, effects:StatusEffectType, start: usize) -> Self {
//         Self {
//             duration: duration,
//             effects: effects,
//             start: start
//             //Change this to start tick, but I'm not sure of the tick system
//         }
//     }
//     pub fn tick(&mut self) {
//         match self.duration {
//             Some(time) => {
//                 self.duration = Some(time-1.0);
//             },
//             None => {}
//         };
//     }
// }
// //What type is it?
// //Might not have effect levels.
// #[derive(Clone, Debug)]
// pub enum StatusEffectType {
//     Slowness,
//     Weakness,
//     Poison,
//     Plague
//     //We'll do this much later.
// }
// // // We probably won't place stuff.
// // #[derive(Clone, Debug)]
// // pub enum PlaceTerrain {
// //     Terrain(Terrain),
// //     Bucket(Terrain),
// //     Special(PlaceSpecial)
// // }
// // // Same with this
// // #[derive(Clone, Debug)]
// // pub enum PlaceSpecial {
// //     Bed,
// // }
// //Weapon info: Weapons are broken up into 3 parts.
// //Will make armor and baubles later.
// #[derive(Clone, Debug)]
// pub enum WeaponTag {
//     Melee(MeleeWeaponTag),
//     Ranged(RangedWeaponTag),
//     Magic(MagicWeaponTag)
// }
// /*
// HOW LEVEL SCALING WORKS:
// quality is out of 100.
// Rarity is out of the Rarity Enumeration:
// Common is 1 to 50, Weight: 50
// Rare is 51 to 80, Weight: 30
// Epic is 81 to 90, Weight: 10
// Mythical is 91 to 97, Weight: 7
// Legendary is 98 to 99, Weight: 2
// SUPREME is 100, Weight: 1
// To upgrade a weapon, one must combine two of equal quality. You will then receive an item of one greater quality, with a random quality in that range. To garuntee a SUPREME item, one must have 32 Commons. Weapon is prioritized over quality.
// base_stat is the stat at quality 1, and base_stat+stat_variation is the maximum stat.
// */
// #[derive(Clone, Debug)]
// pub struct MeleeWeaponTag {
//     damage: GearStat,
//     attack_speed: GearStat,
//     swing_range: GearStat,
//     crit_luck: GearStat,
//     crit_damage: GearStat,
//     sweep: GearStat
// }
// #[derive(Clone, Debug)]
// pub struct RangedWeaponTag {
//     damage: GearStat,
//     crit_luck: GearStat,
//     crit_damage: GearStat,
//     accuracy: GearStat,
//     load_speed: GearStat,
//     range: GearStat
// }
// #[derive(Clone, Debug)]
// pub struct MagicWeaponTag {
//     main_ability: MagicAbility,
//     secondary_ability: MagicAbility,
//     mana: GearStat,
//     mana_regen: GearStat,
//     cooldown_regen: GearStat
// }
// #[derive(Clone, Debug)]
// pub enum WeaponComponentType {
//     Melee(MeleeWeaponComponent),
//     Ranged(RangedWeaponComponent),
//     Magic(MagicWeaponComponent),
// }
// //---THIS IS WHERE COMPONENTS START---
// //These are the only two component we have yet.
// #[derive(Clone, Debug)]
// pub enum ItemComponent {
//     Stack(usize),
//     Weapon(WeaponComponent),
//     Renamed(String)
// }
// //Weapon shit.
// //Will make armor and baubles later.
// #[derive(Clone, Debug)]
// pub struct WeaponComponent {
//     weapon_type: WeaponComponentType,
//     quality: u64,
//     rarity: Rarity,
// }
// #[derive(Clone, Debug)]
// pub struct MeleeWeaponComponent {
//     damage: GearStat,
//     attack_speed: GearStat,
//     swing_range: GearStat,
//     crit_luck: GearStat,
//     crit_damage: GearStat,
//     sweep: GearStat
// }
// #[derive(Clone, Debug)]
// pub struct RangedWeaponComponent {
//     damage: GearStat,
//     crit_luck: GearStat,
//     crit_damage: GearStat,
//     accuracy: GearStat,
//     load_speed: GearStat,
//     range: GearStat
// }
// #[derive(Clone, Debug)]
// pub struct MagicWeaponComponent {
//     mana: GearStat,
//     mana_regen: GearStat,
//     cooldown_regen: GearStat
// }
// //MAGIC SHIT FUCK YEAH!!!
// #[derive(Clone, Debug)]
// pub struct MagicAbility {
//     element_id: usize,
//     mana_cost: i32,
//     ability_id: usize,
//     cooldown_ticks: i32,
//     current_cooldown_ticks: i32
// }
// impl MagicAbility {
//     pub fn new(
//         element_id: usize,
//         mana_cost: i32,
//         ability_id: usize,
//         cooldown_ticks: i32,
//         current_cooldown_ticks: i32
//     ) -> Self {
//         Self {
//             element_id,
//             mana_cost,
//             ability_id,
//             cooldown_ticks,
//             current_cooldown_ticks,
//         }
//     }
// }

use serde::{Deserialize, Serialize};

use super::stat::{GearStatList, StatList};


#[derive(Clone, Debug)]
pub struct Item {
    pub stats: StatList,
    pub lore: String,
    pub name: String,
    pub item_type: ItemType,
    pub width_to_length_ratio: Option<f32>,
    pub sprite: String,
    pub attack_sprite: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ItemArchetype {
    pub name: String,
    pub stats: GearStatList,
    pub lore: String,
    pub item_type: ItemType,
    pub width_to_length_ratio: Option<f32>,
    pub sprite: String,
    pub attack_sprite: Option<String>
}


macro_rules! setup_item_types {
    ($( $variant:ident, )*) => {
        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
        pub enum ItemType {
            $( $variant, )*
        }

        impl ItemType {
            pub fn all() -> Vec<ItemType> {
                vec![$( ItemType::$variant, )*]
            }
        }


    }
}
setup_item_types!{   
    MeleeWeapon,
    RangedWeapon,
    MagicWeapon,
    Shield,
    HelmetArmor,
    ChestplateArmor,    
    LeggingsArmor,
    BootsArmor,
    BaubleRing,
    BaubleCrown,
    BaubleNecklace,
    BaubleBelt,
    BaubleBack,
    BaubleBody,
}
