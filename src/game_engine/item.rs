use std::os::macos::raw::stat;

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
#[derive(Clone, Copy, Debug)]
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
impl GearStat {
    pub fn new(base: Stat, variation: Stat) -> Self {
        let max: Stat = StatList::new(vec![base,  variation]).get_stat_from_enum_as_stat(base).unwrap();
        Self {
            base,
            variation,
            max
        }
    }
}
pub struct StatList {
    health: Option<i32>,
    defense: Option<i32>,
    toughness: Option<i32>,
    vitality: Option<i32>,
    luck: Option<i32>,
    damage: Option<i32>,
    crit_luck: Option<f32>,
    crit_damage: Option<i32>,
    swing_range: Option<f32>,
    accuracy: Option<i32>,
    mana: Option<i32>,
    mana_regen: Option<i32>,
}
impl StatList {
    pub fn new(stat_list: Vec<Stat>) -> Self {
        let mut health: Option<i32> = None;
        let mut defense: Option<i32> = None;
        let mut toughness: Option<i32> = None;
        let mut vitality: Option<i32> = None;
        let mut luck: Option<i32> = None;
        let mut damage: Option<i32> = None;
        let mut crit_luck: Option<f32> = None;
        let mut crit_damage: Option<i32> = None;
        let mut swing_range: Option<f32> = None;
        let mut accuracy: Option<i32> = None;
        let mut mana: Option<i32> = None;
        let mut mana_regen: Option<i32> = None;
        for i in 0..stat_list.len() {
            match stat_list[i] {
                Stat::Health(t_health) => {if health.is_some(){health=Some(t_health)}else{health=Some(t_health+health.unwrap())}},
                Stat::Defense(t_defense) => {if defense.is_some(){defense=Some(t_defense)}else{defense=Some(t_defense+defense.unwrap())}},
                Stat::Toughness(t_toughness) => {if toughness.is_some(){toughness=Some(t_toughness)}else{toughness=Some(t_toughness+toughness.unwrap())}},
                Stat::Vitality(t_vitality) => {if vitality.is_some(){vitality=Some(t_vitality)}else{vitality=Some(t_vitality+vitality.unwrap())}},
                Stat::Luck(t_luck) => {if luck.is_some(){luck=Some(t_luck)}else{luck=Some(t_luck+luck.unwrap())}},
                Stat::Damage(t_damage) => {if damage.is_some(){damage=Some(t_damage)}else{damage=Some(t_damage+damage.unwrap())}},
                Stat::CritLuck(t_crit_luck) => {if crit_luck.is_some(){crit_luck=Some(t_crit_luck)}else{crit_luck=Some(t_crit_luck+crit_luck.unwrap())}},
                Stat::CritDamage(t_crit_damage) => {if crit_damage.is_some(){crit_damage=Some(t_crit_damage)}else{crit_damage=Some(t_crit_damage+crit_damage.unwrap())}},
                Stat::SwingRange(t_swing_range) => {if swing_range.is_some(){swing_range=Some(t_swing_range)}else{swing_range=Some(t_swing_range+swing_range.unwrap())}},
                Stat::Accuracy(t_accuracy) => {if accuracy.is_some(){accuracy=Some(t_accuracy)}else{accuracy=Some(t_accuracy+accuracy.unwrap())}},
                Stat::Mana(t_mana) => {if mana.is_some(){mana=Some(t_mana)}else{mana=Some(t_mana+mana.unwrap())}},
                Stat::ManaRegen(t_mana_regen) => {if mana_regen.is_some(){mana_regen=Some(t_mana_regen)}else{mana_regen=Some(t_mana_regen+mana_regen.unwrap())}},
                _ => {}
            } 
        }
        Self {
            health,
            defense,
            toughness,
            vitality,
            luck,
            damage,
            crit_luck,
            crit_damage,
            swing_range,
            accuracy,
            mana,
            mana_regen
        }
    }
    pub fn get_stat_from_enum_as_stat(&self, stat: Stat) -> Option<Stat>{
        match stat {
            Stat::Health(_) => {if self.health.is_none() {return None} else {return Some(Stat::Health(self.health.unwrap()))}},
            Stat::Defense(_) => {if self.defense.is_none() {return None} else {return Some(Stat::Defense(self.defense.unwrap()))}},
            Stat::Toughness(_) => {if self.toughness.is_none() {return None} else {return Some(Stat::Toughness(self.toughness.unwrap()))}},
            Stat::Vitality(_) => {if self.vitality.is_none() {return None} else {return Some(Stat::Vitality(self.vitality.unwrap()))}},
            Stat::Luck(_) => {if self.luck.is_none() {return None} else {return Some(Stat::Luck(self.luck.unwrap()))}},
            Stat::Damage(_) => {if self.damage.is_none() {return None} else {return Some(Stat::Damage(self.damage.unwrap()))}},
            Stat::CritLuck(_) => {if self.crit_luck.is_none() {return None} else {return Some(Stat::CritLuck(self.crit_luck.unwrap()))}},
            Stat::CritDamage(_) => {if self.crit_damage.is_none() {return None} else {return Some(Stat::CritDamage(self.crit_damage.unwrap()))}},
            Stat::SwingRange(_) => {if self.swing_range.is_none() {return None} else {return Some(Stat::SwingRange(self.swing_range.unwrap()))}},
            Stat::Accuracy(_) => {if self.accuracy.is_none() {return None} else {return Some(Stat::Accuracy(self.accuracy.unwrap()))}},
            Stat::Mana(_) => {if self.mana.is_none() {return None} else {return Some(Stat::Mana(self.mana.unwrap()))}},
            Stat::ManaRegen(_) => {if self.mana_regen.is_none() {return None} else {return Some(Stat::ManaRegen(self.mana_regen.unwrap()))}},
            _ => {panic!("Nonexistent type you fucking idiot.");}
        }
        None
    }
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