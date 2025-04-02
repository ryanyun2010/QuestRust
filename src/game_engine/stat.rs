
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Copy)]
pub struct StatC {
    pub flat: f32,
    pub percent: f32,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)] 
pub struct GearStatC {
    pub flat: Option<GearStat>,
    pub percent: Option<GearStat> 
}

impl std::ops::Add for StatC {
    type Output = Self;
    fn add(self, other: StatC) -> Self {
        StatC {
            flat: self.flat + other.flat,
            percent: self.percent + other.percent,
        }
    }
}
impl std::ops::Sub for StatC {
    type Output = Self;
    fn sub(self, other: StatC) -> Self {
        StatC {
            flat: self.flat - other.flat,
            percent: self.percent - other.percent
        }
    }
}
impl StatC {
    pub fn get_value(&self) -> f32 {
        self.flat * (self.percent/100.0 +1.0)
    }
}


impl GearStatC {
    pub fn get_variation(&self) -> StatC {
        StatC {
            flat: self.flat.map(|x| x.get_variation()).unwrap_or(0.0),
            percent: self.percent.map(|x| x.get_variation()).unwrap_or(0.0)
        }
    }
}
macro_rules! create_stat_lists {
    ($($stat_name:ident => $def:expr),*) => {
        #[derive(Debug, Clone, Default)]
        pub struct StatList {
            $(pub $stat_name: Option<StatC>,)*
        }
        impl StatList {
            pub fn to_sum_with(&mut self, list: &StatList) {
                $(
                    if self.$stat_name.is_some() && list.$stat_name.is_some() {
                        self.$stat_name = Some(self.$stat_name.unwrap() + list.$stat_name.unwrap());
                    } else if (self.$stat_name.is_none() && list.$stat_name.is_some()) {
                        self.$stat_name = list.$stat_name;
                    }
                )*
            }
            pub fn base() -> Self {
                StatList {
                    $(
                        $stat_name: Some(
                            StatC {flat: $def, percent: 0.0}),
                    )*
                }
            }
        }
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct GearStatList {
            $(pub $stat_name: Option<GearStatC>,)*
        }
        impl GearStatList {
            pub fn get_variation(&self) -> StatList {
                let list = StatList {
                    $( $stat_name: self.$stat_name.map(|x| x.get_variation()), )*
                };
                list
            }
        }

        impl IntoIterator for StatList {
            type Item = (&'static str, Option<StatC>);
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                vec![
                    $( (stringify!($stat_name), self.$stat_name), )*
                ].into_iter()
            }
        }
        impl<'a> IntoIterator for &'a StatList {
            type Item = (&'static str, &'a Option<StatC>);
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                vec![
                    $( (stringify!($stat_name), &self.$stat_name), )*
                ].into_iter()
            }
        }
    };
}


pub const BASE_POISON_TICK_DELAY: f32 = 40.0;
pub const BASE_FIRE_TICK_DELAY: f32 = 65.0;

create_stat_lists!(
    health => 100.0,
    defense => 0.0, // 100.0 defense should mean half damage, -100.0 defense should mean double
                    // damage taken
    health_regen => 1.0,
    healing_effectiveness => 100.0, // 0% healing effectiveness = 0 healing
    damage => 0.0,
    lifesteal => 0.0, // 100.0 = 100% of damage is lifestealed
    attack_cooldown => 0.0,
    crit_chance => 3.0,
    crit_damage => 175.0,

    max_mana => 100.0,
    mana_regen => 6.0,
    mana_cost => 0.0, // like defense 
    cooldown_regen => 0.0,
    ability_damage => 0.0,
    charge_time_reduction => 0.0,
    width => 0.0,
    reach => 0.0,
    pierce => 1.0,
    speed => 0.0,
    lifetime => 100.0,
    size => 0.0,
    shots => 1.0,
    focus => 1.0,
    poison_damage => 0.0, 
    poison_duration => 120.0,
    poison_tick_speed => 1.0, 
    fire_damage => 0.0,
    fire_tick_speed => 1.0, 
    fire_duration => 120.0,
    loot => 100.0
);

// cooldown is number of frames, 60 fps, the display is adjusted. so for a 1s cooldown, do a cooldown of 60.

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! create_stat_list {
    ($($field:ident => $value:expr),* $(,)?) => {{
        let mut stats = crate::stat::StatList::default();
        $(
            stats.$field = Some($value);
        )*
        stats
    }};
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
        self.average + self.variation * 2.0 * rng.gen::<f32>() - self.variation
    }
}

pub fn crit_chance_roll(crit_chance: f32) -> bool {
    if crit_chance >= 500.0 {
        return true;
    }
    if rand::random::<f32>() <= (((1000.0/(1.0+std::f32::consts::E.powf(-0.021929*(crit_chance-100.0)))).floor())/1000.0) {
        return true;
    }
    false
}
//Returns f32: [0, 1)
pub fn percent_damage_blocked(defense: i32, toughness: i32, damage: i32) -> f32 {
    (defense as f32/(100.0+defense as f32))*2.0*(1.0-(1.0/(1.0+std::f32::consts::E.powf(-(damage as f32/((toughness as f32).powf(0.8)))))))
}
pub fn healing_with_vitality(incoming_healing: i32, vitality: i32) -> i32 {
    ((vitality as f32 + 100.0)/100.0).min((vitality as f32).powf(0.5)/(incoming_healing as f32).powf(0.5)).ceil() as i32
}
pub fn mana_regen_with_regen(incoming_mana: i32, mana_regen: i32) -> i32 {
    ((mana_regen as f32 + 100.0)/100.0).min((mana_regen as f32).powf(0.5)/(incoming_mana as f32).powf(0.5)).ceil() as i32
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
    for multiplier in multipliers {
        total_multipliers += multiplier;
    }
    if crit_chance_roll(crit_chance) {
        return (total_multipliers*(damage as f32)*(crit_damage as f32+100.0)/100.0).ceil() as i32
    }
    (total_multipliers*(damage as f32)).ceil() as i32
}
