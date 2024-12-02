#[derive(Clone, Debug)]
enum AttackType {
    Melee,
    Range,
    Magic
}
#[derive(Clone, Debug)]
struct EntityTag {
    aggressive: bool,
    undead: bool,
    uruk: bool,
    parasite: bool,
    beast: bool,
    demon: bool,
    dragon: bool,
    follows_player: bool,
    range: usize,
    sight_range: usize,
    attack_type: AttackType,
    attacks: Vec<EntityAttack>,
    movement_speed: usize
}

#[derive(Clone, Debug)]
struct EntityAttack {
    // BLAH BLAH BLAH PEE PEE POO POO
}