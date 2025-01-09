
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerAttack{
    pub archetype: String,
    pub time_alive: f32,
    pub x: f32,
    pub y: f32,
    pub direction: [f32; 2],
    pub dealt_damage: bool
}
impl PlayerAttack{
    pub fn new(archetype: String, time_alive: f32, x: f32, y: f32, direction: [f32; 2]) -> Self{
        Self{
            archetype,
            time_alive,
            x,
            y,
            direction,
            dealt_damage: false
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum PlayerAttackDescriptor{
    Projectile(player_projectile_descriptor),
    Melee(melee_attack_descriptor)
}



#[derive(Debug, Clone, PartialEq)]
pub struct player_projectile_descriptor{
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub AOE: f32,
    pub size: f32,
    pub sprite: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct melee_attack_descriptor{
    pub damage: f32,
    pub width: f32,
    pub reach: f32,
    pub lifetime: f32,
    pub sprite: String
}