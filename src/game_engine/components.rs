use compact_str::CompactString;

use super::entity_components::{AggroComponent, CollisionBox, DamageableComponent, EntityAttackComponent, PathfindingComponent, PositionComponent};
use std::cell::RefCell;
#[macro_export]
macro_rules! setup_components{
    ($( $vec_name:ident => $component_type: ty),*) => {
        pub struct ComponentContainer {
            $(pub $vec_name: Vec<Option<RefCell<$component_type>>>,)*
            pub cur_id: usize,
        } 
        impl Default for ComponentContainer {
            fn default() -> Self {
                Self::new()
            }
        }
        impl ComponentContainer {
            pub fn new() -> Self {
                Self {
                    $( $vec_name: vec![], )*
                    cur_id: 0,
                }
            }
            pub fn add_entity(&mut self) -> usize {
                $( self.$vec_name.push(None); )*
                self.cur_id += 1;
                self.cur_id - 1
            }
            pub fn remove(&mut self, index: usize) {
                $(
                    self.$vec_name[index] = None;
                )*
            }
        }
    }
}

pub struct SpriteComponent {
    pub sprite: usize,
}

pub struct CollisionComponent {
    pub collision_box: CollisionBox,
    pub respects_collision: bool,
}

impl Default for CollisionComponent {
    fn default() -> Self {
        Self {
            collision_box: CollisionBox::default(),
            respects_collision: false,
        }
    }
}

pub struct LootComponent {
    pub loot_tables: Vec<CompactString>
}

pub struct AnimationComponent {
    pub animation_frame: usize // 0-119
}

setup_components!{
    aggro_components => AggroComponent,
    damageable_components => DamageableComponent,
    attack_components => EntityAttackComponent,
    pathfinding_components => PathfindingComponent,
    position_components => PositionComponent,
    sprite_components => SpriteComponent,
    collision_components => CollisionComponent,
    loot_components => LootComponent,
    animation_components => AnimationComponent
}



