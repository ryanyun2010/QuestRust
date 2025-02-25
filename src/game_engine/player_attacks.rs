use super::stat::StatList;


#[derive(Clone, Debug)]
pub struct PlayerAttack{
    pub stats: StatList,
    pub attack_type: PlayerAttackType,
    pub sprite: String,
    pub width_to_length_ratio: f32,
    pub time_alive: f32,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub dealt_damage: bool,

}
impl PlayerAttack{
    pub fn new(stats: StatList, attack_type: PlayerAttackType,  sprite: String, width_to_length_ratio: f32, x: f32, y: f32, angle: f32) -> Self{
        Self{
            stats,
            sprite,
            attack_type,
            width_to_length_ratio,
            time_alive: 0.0,
            x,
            y,
            angle,
            dealt_damage: false
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerAttackType {
    Melee,
    Ranged,
    Magic,
    MeleeAbility,
    RangedAbility,
    MagicAbility
}

