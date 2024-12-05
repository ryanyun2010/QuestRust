use crate::game_engine::entities::AttackType;
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
//Damage is the same as Health basically.
#[derive(Clone, Debug)]

pub enum Stat {
<<<<<<< HEAD
    Damage(i32),
    Health(i32),
    SwingRange(f32),
    CritLuck(f32),
    CritDamage(i32),
=======
    //Armor
    Health(i32),
>>>>>>> 43547c3 (mana)
    Defense(i32),
    Toughness(i32),
    Vitality(i32),
    Luck(i32),
    //Weapons
    Damage(i32), //Global
    CritLuck(f32), //Melee, Ranged
    CritDamage(i32), //Melee, Ranged
    SwingRange(f32),//Melee
    Accuracy(i32), //Ranged, Magic, (Degrees of width of cone).
    Mana(i32),
    ManaRegen(i32)
}
pub fn crit_chance_roll(crit_chance: f32) -> bool {
    if crit_chance >= 500.0 {
        return true;
    }
    if rand::random::<f32>() <= ((((1000.0/(1.0+2.71828_f32.powf(-0.021929*(crit_chance-100.0)))) as f32).floor())/1000.0) {
        return true;
    }
    false
}
//Returns f32: [0, 1)
pub fn percent_damage_blocked(defense: i32, toughness: i32, damage: i32) -> f32 {
    (defense as f32/(100.0+defense as f32))*2.0*(1.0-(1.0/1.0+2.71828_f32.powf(-(damage as f32/((toughness as f32).powf(0.8))))))
}
pub fn healing_with_vitality(incoming_healing: i32, vitality: i32) -> i32 {
    (((vitality as f32 + 100.0)/100.0)).min((vitality as f32).powf(0.5)/(incoming_healing as f32).powf(0.5)).ceil() as i32
}
pub fn mana_regen_with_regen(incoming_mana: i32, mana_regen: i32) -> i32 {
    (((mana_regen as f32 + 100.0)/100.0)).min((mana_regen as f32).powf(0.5)/(incoming_mana as f32).powf(0.5)).ceil() as i32
}
//healing_tick_with_vitality is run on an entity when a healing tick is triggered.
//Healing ticks can be triggered once every 60 frames (1 second) or on ability procs.
//Mana ticks work the same way.
pub fn healing_tick_with_vitality(max_health: i32, current_health: i32, vitality: i32) -> i32 {
    healing_with_vitality((0.05*(max_health-current_health) as f32).ceil() as i32, vitality)
}
pub fn mana_regen_tick_with_regen(max_mana: i32, current_mana: i32, mana_regen: i32) -> i32 {
    mana_regen_with_regen((0.05*(max_mana-current_mana) as f32).ceil() as i32, mana_regen)
}
#[derive(Clone, Debug)]
pub struct GearStat {
    base: Stat,
    variation: Stat,
    max: Stat
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
    quality: u64,
    rarity: Rarity,

}
#[derive(Clone, Debug)]
pub struct RangedWeaponTag {
    
}
#[derive(Clone, Debug)]
pub struct MagicWeaponTag {
}
#[derive(Clone, Debug)]
pub enum WeaponComponent {
    Melee(MeleeWeaponComponent),
    Ranged(RangedWeaponComponent),
    Magic(MagicWeaponComponent)
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