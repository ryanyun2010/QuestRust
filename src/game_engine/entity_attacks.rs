#[derive(Clone, Debug, Copy)]
pub struct EntityAttackBox {
    pub damage: usize,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub reach: usize,
    pub width: usize,
    pub time_to_charge: usize,
    pub time_charged: f32,
    pub sprite_id: usize
}
