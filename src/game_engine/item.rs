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
<<<<<<< HEAD
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
=======
>>>>>>> 421f822 (16 stats now)
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
        let max: Stat = StatList::extract_from_list(vec![base,  variation]).extract_to_stat(base).unwrap();
        Self {
            base,
            variation,
            max
        }
    }
}
//Damage is the same as Health basically.
#[derive(Clone, Copy, Debug)]
pub enum Stat {
    //Armor
    Health(i32),
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
    ManaRegen(i32),
    CooldownRegen(i32), //Magic
    Sweep(i32), //Melee, (Degrees of width of cone)
    LoadSpeed(i32), //Ranged, (In ticks).
    Range(f32), //Ranged
    AbilityDamage(i32), //Magic
}
#[derive(Clone, Debug)]
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
    cooldown_regen: Option<i32>,
    sweep: Option<i32>,
    load_speed: Option<i32>,
    range: Option<f32>,
    ability_damage: Option<i32>,
}
#[macro_export]
macro_rules! extract_stat_to_under {
    ($output:ident, $name:expr; $( $stat_expr_list: ident, $stat_ty_list: ty, $stat_enum_list: ident),*) => {
        $(
            let mut $stat_expr_list: Option<$stat_ty_list> = None;
        )*
        for i in 0..$name.len() {
            match $name[i] {
                $(
                Stat::$stat_enum_list(adding) => {if $stat_expr_list.is_none(){$stat_expr_list=Some(adding)}else{$stat_expr_list=Some(adding+$stat_expr_list.unwrap())}},
                )*
            }
        }
        $output = StatList::new(
            $(
                $stat_expr_list,
            )*
        )
    }
}
#[macro_export]
macro_rules! extract_stat_to {
    ($output:ident, $name:expr) => {
        extract_stat_to_under! [
            $output, $name;
            health, i32, Health,
            defense, i32, Defense,
            toughness, i32, Toughness,
            vitality, i32, Vitality,
            luck, i32, Luck,
            damage, i32, Damage,
            crit_luck, f32, CritLuck,
            crit_damage, i32, CritDamage,
            swing_range, f32, SwingRange,
            accuracy, i32, Accuracy,
            mana, i32, Mana,
            mana_regen, i32, ManaRegen,
            cooldown_regen, i32, CooldownRegen,
            sweep, i32, Sweep,
            load_speed, i32, LoadSpeed,
            range, f32, Range,
            ability_damage, i32, AbilityDamage
        ];

    }
}
#[macro_export]
macro_rules! extract_to_stat {
    ($name:expr, $enum:expr) => {
        if $name.is_none() {return None}
        else {return Some($enum($name.unwrap()))}}
}
impl StatList {
    pub fn new(
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
        cooldown_regen: Option<i32>,
        sweep: Option<i32>,
        load_speed: Option<i32>,
        range: Option<f32>,
        ability_damage: Option<i32>,
    ) -> Self {
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
            mana_regen,
            cooldown_regen,
            sweep,
            load_speed,
            range,
            ability_damage,
        }
    }
    pub fn extract_from_list(stat_list: Vec<Stat>) -> Self {
        let out: StatList;
        extract_stat_to!(out, stat_list);
        out
    }
    pub fn extract_to_stat(&self, stat: Stat) -> Option<Stat>{
        match stat {
            Stat::Health(_) => extract_to_stat!(self.health, Stat::Health),
            Stat::Defense(_) => {extract_to_stat!(self.defense, Stat::Defense)},
            Stat::Toughness(_) => {extract_to_stat!(self.toughness, Stat::Toughness)},
            Stat::Vitality(_) => {extract_to_stat!(self.vitality, Stat::Vitality)},
            Stat::Luck(_) => {extract_to_stat!(self.luck, Stat::Luck)},
            Stat::Damage(_) => {extract_to_stat!(self.damage, Stat::Damage)},
            Stat::CritLuck(_) => {extract_to_stat!(self.crit_luck, Stat::CritLuck)},
            Stat::CritDamage(_) => {extract_to_stat!(self.crit_damage, Stat::CritDamage)},
            Stat::SwingRange(_) => {extract_to_stat!(self.swing_range, Stat::SwingRange)},
            Stat::Accuracy(_) => {extract_to_stat!(self.accuracy, Stat::Accuracy)},
            Stat::Mana(_) => {extract_to_stat!(self.mana, Stat::Mana)},
            Stat::ManaRegen(_) => {extract_to_stat!(self.mana_regen, Stat::ManaRegen)},
            Stat::CooldownRegen(_) => {extract_to_stat!(self.cooldown_regen, Stat::CooldownRegen)},
            Stat::Sweep(_) => {extract_to_stat!(self.sweep, Stat::Sweep)},
            Stat::LoadSpeed(_) => {extract_to_stat!(self.load_speed, Stat::LoadSpeed)},
            Stat::Range(_) => {extract_to_stat!(self.range, Stat::Range)},
            Stat::AbilityDamage(_) => {extract_to_stat!(self.ability_damage, Stat::AbilityDamage)},
        }
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