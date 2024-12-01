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
pub struct EntityTags {
    aggressive: bool,
    monster_type: MonsterType,
    follows_player: bool,
    range: usize,
    aggro_range: usize,
    attack_type: AttackType,
    attacks: EntityAttackPattern,
    movement_speed: usize,
    is_item: bool,
    contained_item: Option<Item>,
    drops: Option<Loot>
}

impl EntityTags{
    pub fn new(aggressive: bool, monster_type: MonsterType, follows_player: bool, range: usize, aggro_range: usize, attack_type: AttackType, attacks: EntityAttackPattern, movement_speed: usize, is_item: bool, drops: Option<Loot>,  contained_item: Option<Item>) -> Self{
        Self{
            aggressive: aggressive,
            monster_type: monster_type,
            follows_player: follows_player,
            range: range,
            aggro_range: aggro_range,
            attack_type: attack_type,
            attacks: attacks,
            movement_speed: movement_speed,
            is_item: is_item,
            drops: drops,
            contained_item: contained_item
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntityAttackPattern {
    attacks: Vec<EntityAttack>,
    // TODO
}

#[derive(Copy, Clone, Debug)]
pub struct EntityAttack{
    // TODO
}

#[derive(Clone, Debug)]
pub struct Item {
    name: String
}