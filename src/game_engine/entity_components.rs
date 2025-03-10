use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::pathfinding::EntityDirectionOptions;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionComponent{
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug , PartialEq)]
pub struct EntityAttackComponent{
    pub cur_attack: usize,
    pub cur_attack_cooldown: f32,
    pub entity_attack_pattern: CompactString,
    pub attack_range: usize
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct PathfindingComponent{
    pub cur_direction: EntityDirectionOptions,
    pub aggroed_to_player: bool,
    pub movement_speed: usize,
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct DamageableComponent{
    pub health: f32,
    pub max_health: usize,
    pub damage_box: CollisionBox,
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
    pub aggro_range: usize,
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct CollisionBox{
    pub x_offset: f32,
    pub y_offset: f32,
    pub w: f32,
    pub h: f32,
}
impl Default for CollisionBox{
    fn default() -> Self{
        Self{
            x_offset: 0.0,
            y_offset: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }
}

