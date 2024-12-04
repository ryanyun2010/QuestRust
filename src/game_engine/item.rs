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
    Mining(MiningTool),
}
#[derive(Clone, Debug)]
pub struct MiningTool {
    can_mine: MiningToolType,
    mining_speed: f64,
}
#[derive(Clone, Debug)]
pub enum MiningToolType {
    Pickaxe,
    Axe,
    Shovel,
    Paxel
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