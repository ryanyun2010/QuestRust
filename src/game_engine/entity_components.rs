use serde::{Deserialize, Serialize};

use super::pathfinding::EntityDirectionOptions;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct PositionComponent{
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct EntityAttackComponent{
    pub cur_attack: usize,
    pub cur_attack_cooldown: f32,
}
impl Default for EntityAttackComponent{
    fn default() -> Self{
        Self{
            cur_attack: 0,
            cur_attack_cooldown: 0.0,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct PathfindingComponent{
    pub cur_direction: EntityDirectionOptions,
    pub aggroed_to_player: bool,
}
impl Default for PathfindingComponent{
    fn default() -> Self{
        Self{
            cur_direction: EntityDirectionOptions::None,
            aggroed_to_player: false,
        }
    }
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct HealthComponent{
    pub health: f32,
    pub max_health: usize,
}
impl HealthComponent{
    pub fn new(max_health: usize) -> Self{
        Self{
            health: max_health as f32,
            max_health,
        }
    }
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct AggroComponent{
    pub aggroed: bool,
}
impl Default for AggroComponent{
    fn default() -> Self{
        Self{
            aggroed: false
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct CollisionBox{
    pub x_offset: f32,
    pub y_offset: f32,
    pub w: f32,
    pub h: f32,
}