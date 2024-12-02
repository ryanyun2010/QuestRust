#[derive(Copy, Clone, Debug)]
enum AttackType {
    Melee,
    Range,
    Magic
}


#[derive(Copy, Clone, Debug)]
enum MonsterType {
    Undead,
    Uruk,
    Parasite,
    Beast,
    Demon,
    Dragon
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
    movement_speed: usize
}

impl EntityTags{
    pub fn new(aggressive: bool, monster_type: MonsterType, follows_player: bool, range: usize, aggro_range: usize, attack_type: AttackType, attacks: EntityAttackPattern, movement_speed: usize) -> Self{
        Self{
            aggressive: aggressive,
            monster_type: monster_type,
            follows_player: follows_player,
            range: range,
            aggro_range: aggro_range,
            attack_type: attack_type,
            attacks: attacks,
            movement_speed: movement_speed
        }
    }
}

#[derive(Clone, Debug)]
struct EntityAttackPattern {
    attacks: Vec<EntityAttack>,
    // BLAH BLAH BLAH PEE PEE POO POO
}

#[derive(Copy, Clone, Debug)]
struct EntityAttack{
    // BLAH BLAH BLAH PEE PEE POO POO
}