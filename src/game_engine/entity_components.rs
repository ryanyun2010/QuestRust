use super::world::World;
use super::pathfinding::EntityDirectionOptions;
use super::entities::EntityAttack;
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

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
impl EntityAttackComponent{
    pub fn default() -> Self{
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
impl PathfindingComponent{
    pub fn default() -> Self{
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
            max_health: max_health,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct CollisionBox{
    pub x_offset: f32,
    pub y_offset: f32,
    pub w: f32,
    pub h: f32,
}

macro_rules! setup_components {
    ( $( $name:ident => $component:ty ),* ) => {
        #[derive(Debug, Clone)]
        pub struct EntityComponentHolder{
            $(
                pub $name: RefCell<Option<$component>>,
            )*
        }
        impl EntityComponentHolder{
            pub fn new() -> Self{
                Self{
                    $(
                        $name: RefCell::new(None),
                    )*
                }
            }
        }
    };
}

setup_components!(
    position => PositionComponent,
    attack => EntityAttackComponent, 
    pathfinding => PathfindingComponent,
    health => HealthComponent, 
    collision_box => CollisionBox);
    