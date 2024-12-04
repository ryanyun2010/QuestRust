use crate::game_engine::entities::AttackType;
use super::world::Terrain;

#[derive(Clone, Debug)]
pub enum Item {
    Lore(String),
    Weapon(Weapon),
    Stacking(usize),
    Place(PlaceTerrain),
    Use(UseItem),
    MaxDurability(usize),
    UseRange(f64),
}
#[derive(Clone, Debug)]
pub enum UseItem {
    Mining(ChoppingTool),
    Eat(Food)
}
#[derive(Clone, Debug)]
pub struct Food {
    hunger: usize,
    eating_time: f64,
    effects: Option<Vec<StatusEffect>>
}
#[derive(Clone, Debug)]
pub struct StatusEffect {
    duration: Option<f64>,
    effects: StatusEffectType,
    expired: bool
}
impl StatusEffect {
    pub fn new(duration: Option<f64>, effects:StatusEffectType) -> Self {
        Self {
            duration: duration,
            effects: effects,
            expired: false,
        }
    }
    pub fn tick(&mut self) {
        match self.duration {
            Some(time) => {
                self.duration = Some(time-1.0);
                if time-1.0 <= 0.0 {
                    self.expired = true
                }
            },
            None => {}
        };
    }
    pub fn kill(&mut self) {
        self.expired = true;
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
pub struct ChoppingTool {
    chopping_speed: f64,
    chopping_power: u8,
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
pub enum Weapon {
    Melee(MeleeWeapon),
    Ranged(RangedWeapon),
    Magic(MagicWeapon)
}
#[derive(Clone, Debug)]
pub struct MeleeWeapon {

}
#[derive(Clone, Debug)]
pub struct RangedWeapon {
    
}
#[derive(Clone, Debug)]
pub struct MagicWeapon {
    
}