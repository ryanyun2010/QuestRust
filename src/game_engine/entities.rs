use crate::loot::Loot;
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
    attacks: Vec<EntityAttack>,
    // TODO
}
impl EntityAttackPattern{
    pub fn new() -> Self{
        Self{
            attacks: Vec::new()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityAttack{
    // TODO
}

#[derive(Clone, Debug)]
pub struct Item {
    name: String
}