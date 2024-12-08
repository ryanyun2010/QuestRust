use std::os::macos::raw::stat;
use crate::game_engine::entities::AttackType;
use crate::stat::GearStat;
use super::{loot::Rarity, world::{Sprite, Terrain}};
#[derive(Clone, Debug)]
pub struct Item {
    item_type_id: usize,
    component_list: Vec<ItemComponent>,
}
#[derive(Clone, Debug)]
pub enum ItemComponent {
    Stack(usize),
    Weapon(WeaponComponent)
}
#[derive(Clone, Debug)]
pub enum ItemTags {
    Sprite(Sprite),
    Lore(String),
    Weapon(WeaponTag),
    Place(PlaceTerrain),
    Use(UseItem),
    MaxDurability(usize),
    UseRange(f64),
}
#[derive(Clone, Debug)]
pub enum UseItem {
    Eat(Food)
}
#[derive(Clone, Debug)]
pub struct Food {
    eating_time: f64,
    effects: Option<Vec<StatusEffect>>
}
#[derive(Clone, Debug)]
pub struct StatusEffect {
    duration: Option<f64>,
    start: usize, //What type is time measured in?
    effects: StatusEffectType
}
impl StatusEffect {
    pub fn new(duration: Option<f64>, effects:StatusEffectType, start: usize) -> Self {
        Self {
            duration: duration,
            effects: effects,
            start: start
            //Change this to start tick, but I'm not sure of the tick system
        }
    }
    pub fn tick(&mut self) {
        match self.duration {
            Some(time) => {
                self.duration = Some(time-1.0);
            },
            None => {}
        };
    }
}
#[derive(Clone, Debug)]
pub enum StatusEffectType {
    Slowness,
    Weakness,
    Poison,
    Plague
    //We'll do this much later.
}
#[derive(Clone, Debug)]
pub enum PlaceTerrain {
    Terrain(Terrain),
    Bucket(Terrain),
    Special(PlaceSpecial)
}
#[derive(Clone, Debug)]
pub enum PlaceSpecial {
    Bed,
}
#[derive(Clone, Debug)]
pub enum WeaponTag {
    Melee(MeleeWeaponTag),
    Ranged(RangedWeaponTag),
    Magic(MagicWeaponTag)
}
#[derive(Clone, Debug)]
pub struct MeleeWeaponTag {
    /*
    HOW LEVEL SCALING WORKS:
    quality is out of 100.
    Rarity is out of the Rarity Enumeration:
    Common is 1 to 50, Weight: 50
    Rare is 51 to 80, Weight: 30
    Epic is 81 to 90, Weight: 10
    Mythical is 91 to 97, Weight: 7
    Legendary is 98 to 99, Weight: 2
    SUPREME is 100, Weight: 1
    To upgrade a weapon, one must combine two of equal quality. You will then receive an item of one greater quality, with a random quality in that range. To garuntee a SUPREME item, one must have 32 Commons. Weapon is prioritized over quality.
    base_stat is the stat at quality 1, and base_stat+stat_variation is the maximum stat.
    */
    damage: GearStat,
    attack_speed: GearStat,
    swing_range: GearStat,
}
#[derive(Clone, Debug)]
pub struct RangedWeaponTag {
    
}
#[derive(Clone, Debug)]
pub struct MagicWeaponTag {
}
#[derive(Clone, Debug)]
pub enum WeaponComponentType {
    Melee(MeleeWeaponComponent),
    Ranged(RangedWeaponComponent),
    Magic(MagicWeaponComponent),
}
#[derive(Clone, Debug)]
pub struct WeaponComponent {
    weapon_type: WeaponComponentType,
    quality: u64,
    rarity: Rarity,
}
#[derive(Clone, Debug)]
pub struct MeleeWeaponComponent {

}
#[derive(Clone, Debug)]
pub struct RangedWeaponComponent {
    
}
#[derive(Clone, Debug)]
pub struct MagicWeaponComponent {

}