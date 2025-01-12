#[derive(Clone, Debug)]
pub struct EntityAttackBox {
    pub archetype: String,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub time_charged: f32,
}

#[derive(Clone, Debug)]
pub struct EntityAttackDescriptor{
    pub damage: f32,
    pub reach: usize,
    pub width: usize,
    pub time_to_charge: usize,
    pub sprite: String
}
