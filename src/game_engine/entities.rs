use crate::loot::Loot;
use crate::game_engine::item::Item;
use std::cell::RefCell;

use super::world::Player;
#[derive(Copy, Clone, Debug)]
pub enum AttackType {
    Melee,
    Range,
    Magic
}


#[derive(Copy, Clone, Debug)]
pub enum MonsterType {
    Undead,
    Uruk,
    Parasite,
    Beast,
    Demon,
    Dragon,
    Item,
    Ambient,
    Structure
}
#[derive(Clone, Debug)]
pub enum EntityTags {
    Aggressive,
    MonsterType(MonsterType),
    FollowsPlayer,
    Range(usize),
    AggroRange(usize),
    AttackType(AttackType),
    Attacks(EntityAttackPattern),
    MovementSpeed(f32),
    Item(Item),
    Drops(Loot),
    BaseHealth(usize),
}

impl EntityTags{
    // pub fn new(aggressive: bool, monster_type: MonsterType, follows_player: bool, range: usize, aggro_range: usize, attack_type: AttackType, attacks: EntityAttackPattern, movement_speed: usize, is_item: bool, drops: Option<Loot>,  contained_item: Option<Item>, max_health: usize) -> Self{
    //     Self{
    //         aggressive: aggressive,
    //         monster_type: monster_type,
    //         follows_player: follows_player,
    //         range: range,
    //         aggro_range: aggro_range,
    //         attack_type: attack_type,
    //         attacks: attacks,
    //         movement_speed: movement_speed,
    //         is_item: is_item,
    //         drops: drops,
    //         contained_item: contained_item,
    //         max_health: max_health
    //     }
    // }

}

#[derive(Clone, Debug)]
pub struct EntityAttackPattern {
    pub attacks: Vec<EntityAttack>,
    pub attack_cooldowns: Vec<f32>,  
}
impl EntityAttackPattern{
    pub fn new(attacks: Vec<EntityAttack>, attack_cooldowns: Vec<f32>) -> Self{
        Self{
            attacks: attacks,
            attack_cooldowns: attack_cooldowns,
        }
    }
    // pub fn in_range_to_attack(&mut self) -> Option<EntityAttack>{
    //     if self.cur_attack_cooldown <= 0.0{
    //         self.cur_attack += 1;
    //         if self.cur_attack >= self.attacks.len(){
    //             self.cur_attack = 0;
    //         }
    //         self.cur_attack_cooldown = self.attack_cooldowns[self.cur_attack];
    //         return Some(self.attacks[self.cur_attack]);
    //     }
    //     self.cur_attack_cooldown -= 1.0/60.0;
    //     None
    // }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityAttack{
    damage: usize
}

impl EntityAttack{
    pub fn new(damage: usize) -> Self{
        Self{
            damage: damage
        }
    }

    pub fn attack(&self) -> i32{
        self.damage as i32
    }
}