use compact_str::CompactString;

use super::{entities::AttackType, json_parsing::{FireDescriptor, PoisonDescriptor}};

#[derive(Clone, Debug)]
pub struct EntityAttackBox {
    pub archetype: CompactString,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub time_charged: f32,
}

#[derive(Clone, Debug)]
pub struct EntityAttackDescriptor{
    pub r#type: AttackType,
    pub damage: f32,
    pub reach: usize,
    pub width: usize,
    pub time_to_charge: usize,
    pub max_start_dist_from_entity: Option<usize>,
    pub sprite: CompactString,
    pub fire: Option<FireDescriptor>,
    pub poison: Option<PoisonDescriptor>
}
