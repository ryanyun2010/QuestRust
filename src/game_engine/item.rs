use crate::game_engine::entities::AttackType;
#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    lore: String,
    is_weapon: bool,
    weapon_type: AttackType,
    damage: usize, 
}

