// #[derive(Clone, Debug)]
// pub struct GearStat {
//     base: Stat,
//     variation: Stat,
// }
// impl GearStat {
//     pub fn new(base: Stat, variation: Stat) -> Self {
//         Self {
//             base,
//             variation,
//         }
//     }
//     pub fn get_max(&self) -> Stat {
//         let max = StatList::extract_from_stat_vec(vec![self.base,  self.variation]).extract_to_stat(self.base).unwrap();
//         return max;
//     }
//     pub fn get_min(&self) -> Stat {
//         let min = StatList::extract_from_stat_vec(vec![self.base,  self.variation]).extract_to_stat(self.base).unwrap();
//         return min;
//     }
   
// }
// //Damage is the same as Health basically.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum Stat {
//     //Armor
//     Health(i32),
//     Defense(i32),
//     Toughness(i32),
//     Vitality(i32),
//     Luck(i32),
//     //Weapons
//     Damage(i32), //Global
//     CritLuck(f32), //Melee, Ranged
//     CritDamage(i32), //Melee, Ranged
//     SwingRange(f32),//Melee
//     Accuracy(i32), //Ranged, Magic, (Degrees of width of cone).
//     Mana(i32), //Magic
//     ManaRegen(i32), //Magic
//     CooldownRegen(i32), //Magic
//     Sweep(i32), //Melee, (Degrees of width of cone)
//     LoadSpeed(i32), //Ranged, (In ticks).
//     Range(f32), //Ranged
//     AbilityDamage(i32), //Magic
// }
// #[derive(Clone, Debug)]
// pub struct StatList {
//     health: Option<i32>,
//     defense: Option<i32>,
//     toughness: Option<i32>,
//     vitality: Option<i32>,
//     luck: Option<i32>,
//     damage: Option<i32>,
//     crit_luck: Option<f32>,
//     crit_damage: Option<i32>,
//     swing_range: Option<f32>,
//     accuracy: Option<i32>,
//     mana: Option<i32>,
//     mana_regen: Option<i32>,
//     cooldown_regen: Option<i32>,
//     sweep: Option<i32>,
//     load_speed: Option<i32>,
//     range: Option<f32>,
//     ability_damage: Option<i32>,
// }
// #[macro_export]
// macro_rules! extract_stat_vec_to {
//     ($output:ident, $name:expr) => {
//         extract_stat_vec_to_under! [
//             $output, $name;
//             health, i32, Health,
//             defense, i32, Defense,
//             toughness, i32, Toughness,
//             vitality, i32, Vitality,
//             luck, i32, Luck,
//             damage, i32, Damage,
//             crit_luck, f32, CritLuck,
//             crit_damage, i32, CritDamage,
//             swing_range, f32, SwingRange,
//             accuracy, i32, Accuracy,
//             mana, i32, Mana,
//             mana_regen, i32, ManaRegen,
//             cooldown_regen, i32, CooldownRegen,
//             sweep, i32, Sweep,
//             load_speed, i32, LoadSpeed,
//             range, f32, Range,
//             ability_damage, i32, AbilityDamage
//         ];
//     }
// }
// #[macro_export]
// macro_rules! extract_stat_vec_to_under {
//     ($output:ident, $name:expr; $( $stat_expr_list: ident, $stat_ty_list: ty, $stat_enum_list: ident),*) => {
//         $(
//             let mut $stat_expr_list: Option<$stat_ty_list> = None;
//         )*
//         for i in 0..$name.len() {
//             match $name[i] {
//                 $(
//                 Stat::$stat_enum_list(adding) => {if $stat_expr_list.is_none(){$stat_expr_list=Some(adding)}else{$stat_expr_list=Some(adding+$stat_expr_list.unwrap())}},
//                 )*
//             }
//         }
//         $output = StatList::new(
//             $(
//                 $stat_expr_list,
//             )*
//         )
//     }
// }
// #[macro_export]
// macro_rules! extract_to_stat_vec {
//     ($output:ident, $name:expr) => {
//         extract_to_stat_vec_under! [
//             $output, $name;
//             health, i32, Health,
//             defense, i32, Defense,
//             toughness, i32, Toughness,
//             vitality, i32, Vitality,
//             luck, i32, Luck,
//             damage, i32, Damage,
//             crit_luck, f32, CritLuck,
//             crit_damage, i32, CritDamage,
//             swing_range, f32, SwingRange,
//             accuracy, i32, Accuracy,
//             mana, i32, Mana,
//             mana_regen, i32, ManaRegen,
//             cooldown_regen, i32, CooldownRegen,
//             sweep, i32, Sweep,
//             load_speed, i32, LoadSpeed,
//             range, f32, Range,
//             ability_damage, i32, AbilityDamage
//         ];
//     }
// }
// #[macro_export]
// macro_rules! extract_to_stat_vec_under {
//     ($output:ident, $name:expr; $( $stat_expr_list: ident, $stat_ty_list: ty, $stat_enum_list: ident),*) => {
//         $(
//             if $name.$stat_expr_list.is_some() {$output.push(Stat::$stat_enum_list($name.$stat_expr_list.unwrap()))}
//         )*
//     }
// }
// #[macro_export]
// macro_rules! extract_stat_vec_to_stat_macro {
//     ($output:expr, $name:expr, $stat_type:expr) => {
//         extract_stat_vec_to_stat_under![
//             $output, $name, $stat_type;
//             health, i32, Health,
//             defense, i32, Defense,
//             toughness, i32, Toughness,
//             vitality, i32, Vitality,
//             luck, i32, Luck,
//             damage, i32, Damage,
//             crit_luck, f32, CritLuck,
//             crit_damage, i32, CritDamage,
//             swing_range, f32, SwingRange,
//             accuracy, i32, Accuracy,
//             mana, i32, Mana,
//             mana_regen, i32, ManaRegen,
//             cooldown_regen, i32, CooldownRegen,
//             sweep, i32, Sweep,
//             load_speed, i32, LoadSpeed,
//             range, f32, Range,
//             ability_damage, i32, AbilityDamage
//         ];
//     }
// }
// #[macro_export]
// macro_rules! extract_stat_vec_to_stat_under {
//     ($output:expr, $name:expr, $stat_type:expr; $( $stat_expr_list: ident, $stat_ty_list: ty, $stat_enum_list: ident),*) => {
//         let mut matching1: Stat;
//         let mut matching2: Stat;

//         match $stat_type {
//             $(
//                 Stat::$stat_enum_list(_) => {matching1 = Stat::$stat_enum_list(0 as $stat_ty_list);},
//             )*
//         }
//         'match_extract_stat_vec_to_stat_under: for i in 0..$name.len() {
//             match $name[i] {
//                 $(
//                     Stat::$stat_enum_list(_) => {
//                         matching2 = Stat::$stat_enum_list(0 as $stat_ty_list);
//                         if matching1 == matching2 {
//                             $output = Some($name[i]);
//                             break 'match_extract_stat_vec_to_stat_under
//                         }
//                     },
//                 )*
//             }
//         }
//     }
// }
// pub fn extract_stat_vec_to_stat(vector: Vec<Stat>, goal: Stat) -> Option<Stat> {
//     let mut output: Option<Stat> = None;
//     extract_stat_vec_to_stat_macro!(output, vector, goal);
//     output
// }
// impl StatList {
//     pub fn new(
//         health: Option<i32>,
//         defense: Option<i32>,
//         toughness: Option<i32>,
//         vitality: Option<i32>,
//         luck: Option<i32>,
//         damage: Option<i32>,
//         crit_luck: Option<f32>,
//         crit_damage: Option<i32>,
//         swing_range: Option<f32>,
//         accuracy: Option<i32>,
//         mana: Option<i32>,
//         mana_regen: Option<i32>,
//         cooldown_regen: Option<i32>,
//         sweep: Option<i32>,
//         load_speed: Option<i32>,
//         range: Option<f32>,
//         ability_damage: Option<i32>,
//     ) -> Self {
//         Self {
//             health,
//             defense,
//             toughness,
//             vitality,
//             luck,
//             damage,
//             crit_luck,
//             crit_damage,
//             swing_range,
//             accuracy,
//             mana,
//             mana_regen,
//             cooldown_regen,
//             sweep,
//             load_speed,
//             range,
//             ability_damage,
//         }
//     }
//     pub fn extract_from_stat_vec(stat_list: Vec<Stat>) -> Self {
//         let output: StatList;
//         extract_stat_vec_to!(output, stat_list);
//         output
//     }
//     pub fn extract_to_stat_vec(&self) -> Vec<Stat>{
//         let mut output: Vec<Stat> = Vec::new();
//         extract_to_stat_vec!(output, self);
//         output
//     }
//     pub fn extract_to_stat(&self, goal: Stat) -> Option<Stat>{
//         let mut output2: Vec<Stat> = Vec::new();
//         extract_to_stat_vec!(output2, self);
//         extract_stat_vec_to_stat( output2, goal)
//     }
// }

use rand::prelude::*;

use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct StatList {
    pub health: Option<f32>,
    pub defense: Option<f32>,
    pub toughness: Option<f32>,
    pub vitality: Option<f32>,
    pub luck: Option<f32>,
    pub damage: Option<f32>,
    pub crit_luck: Option<f32>,
    pub crit_damage: Option<f32>,
    pub reach: Option<f32>,
    pub accuracy: Option<f32>,
    pub mana: Option<f32>,
    pub mana_regen: Option<f32>,
    pub cooldown_regen: Option<f32>,
    pub width: Option<f32>,
    pub load_speed: Option<f32>,
    pub range: Option<f32>,
    pub lifetime: Option<f32>,
    pub speed: Option<f32>,
    pub ability_damage: Option<f32>,
    pub size: Option<f32>,
    pub AOE: Option<f32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearStatList {
    health: Option<GearStat>,
    defense: Option<GearStat>,
    toughness: Option<GearStat>,
    vitality: Option<GearStat>,
    luck: Option<GearStat>,
    damage: Option<GearStat>,
    crit_luck: Option<GearStat>,
    crit_damage: Option<GearStat>,
    reach: Option<GearStat>,
    accuracy: Option<GearStat>,
    mana: Option<GearStat>,
    mana_regen: Option<GearStat>,
    cooldown_regen: Option<GearStat>,
    width: Option<GearStat>,
    load_speed: Option<GearStat>,
    range: Option<GearStat>,
    lifetime: Option<GearStat>,
    speed: Option<GearStat>,
    ability_damage: Option<GearStat>,
    size: Option<GearStat>,
    AOE: Option<GearStat>,
}

impl GearStatList {
    pub fn get_variation(&self) -> StatList {
        let mut list = StatList {
            health: self.health.map(|x| x.get_variation()),
            defense: self.defense.map(|x| x.get_variation()),
            toughness: self.toughness.map(|x| x.get_variation()),
            vitality: self.vitality.map(|x| x.get_variation()),
            luck: self.luck.map(|x| x.get_variation()),
            damage: self.damage.map(|x| x.get_variation()),
            crit_luck: self.crit_luck.map(|x| x.get_variation()),
            crit_damage: self.crit_damage.map(|x| x.get_variation()),
            reach: self.reach.map(|x| x.get_variation()),
            accuracy: self.accuracy.map(|x| x.get_variation()),
            mana: self.mana.map(|x| x.get_variation()),
            mana_regen: self.mana_regen.map(|x| x.get_variation()),
            cooldown_regen: self.cooldown_regen.map(|x| x.get_variation()),
            width: self.width.map(|x| x.get_variation()),
            load_speed: self.load_speed.map(|x| x.get_variation()),
            range: self.range.map(|x| x.get_variation()),
            lifetime: self.lifetime.map(|x| x.get_variation()),
            speed: self.speed.map(|x| x.get_variation()),
            ability_damage: self.ability_damage.map(|x| x.get_variation()),
            size: self.size.map(|x| x.get_variation()),
            AOE: self.AOE.map(|x| x.get_variation()),
        };
        list
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GearStat {
    average: f32,
    variation: f32,
}
impl GearStat {
    pub fn get_max(&self) -> f32 {
        self.average + self.variation
    }
    pub fn get_min(&self) -> f32 {
        self.average - self.variation
    }
    pub fn get_variation(&self) -> f32 {
        let mut rng = rand::thread_rng();
        (self.average + self.variation * 2.0 * rng.gen::<f32>() - self.variation).round()
    }
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
pub fn calculate_scaling_damage(multipliers: Vec<f32>, damage: i32, crit_chance: f32, crit_damage: i32) -> i32 {
    //Multipliers are additive
    let mut total_multipliers: f32 = 0.0;
    for i in 0..multipliers.len() {
        total_multipliers += multipliers[i];
    }
    if crit_chance_roll(crit_chance) {
        return (total_multipliers*(damage as f32)*(crit_damage as f32+100.0)/100.0).ceil() as i32
    }
    return (total_multipliers*(damage as f32)).ceil() as i32
}