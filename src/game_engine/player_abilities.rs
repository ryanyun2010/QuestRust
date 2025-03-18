use compact_str::CompactString;

use crate::create_stat_list;
use crate::game_engine::game::MousePosition;
use crate::stat::StatC;
use crate::world::World;
use std::f32::consts::PI;
use crate::PError;
use crate::perror;
use crate::ptry;
use crate::punwrap;
use crate::game_engine::player::PlayerState;

use super::item::ItemType;
use super::player::PlayerDir;
use super::stat::StatList;


pub struct PlayerAbilityActionDescriptor {
    pub on_start: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub while_charging: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub on_ending_start: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub while_ending: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub on_end: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>
}



pub struct PlayerAbilityDescriptor {
    pub name: CompactString,
    pub description: String,
    pub base_stats: StatList, // NOTE: COOLDOWN STAT IN BASE_STATS SHOULD BE IGNORED 
    pub flat_added_damage_effectiveness: f32, // 1.0 for flat is 100% effective
    pub cooldown: f32,
    pub time_to_charge: f32, 
    pub end_time: f32,
    pub actions: PlayerAbilityActionDescriptor,
    pub usable_with: UsableWith 
}


pub struct UsableWith {
    pub item_types: Vec<ItemType>,
    pub usable_with_nothing: bool
}



impl PlayerAbilityDescriptor {
    pub fn create_player_ability(&self, descriptor_id_of_this_descriptor: usize) -> PlayerAbility {
        PlayerAbility {
            stats: create_stat_list!(
                damage => StatC {
                    flat: 0.0,
                    percent: 0.0
                }
            ),
            adjusted_time_to_charge: self.time_to_charge,
            adjusted_cooldown: self.cooldown,
            end_time_left: self.end_time,
            cooldown_time_left: self.cooldown,
            time_to_charge_left: self.time_to_charge,
            descriptor_id: descriptor_id_of_this_descriptor,
            end_without_end_action: false,
            on_start_state: None,
            on_end_start_state: None
        }
    }
    pub fn setup_player_ability(&self, ability: &mut PlayerAbility, stats: &StatList) {
        let mut s = self.base_stats.clone();
        
        s.to_sum_with(stats);
        
        let base_damage = self.base_stats.damage.map(|x| x.flat).unwrap_or(0.0);
        let added_damage = stats.damage.map(|x| x.flat).unwrap_or(0.0);
        let percent_damage = s.damage.map(|x| x.percent).unwrap_or(0.0);
        s.damage = Some(
            StatC
            {
                flat: base_damage + added_damage * self.flat_added_damage_effectiveness,
                percent: percent_damage, 
            }
        );

        ability.adjusted_cooldown = self.cooldown / (s.cooldown_regen.map(|x| x.get_value()).unwrap_or(0.0) + 1.0);
        ability.adjusted_time_to_charge = self.time_to_charge / (s.charge_time_reduction.map(|x| x.get_value()).unwrap_or(0.0) + 1.0);
        ability.end_time_left = self.end_time;
        ability.time_to_charge_left = ability.adjusted_time_to_charge;
        ability.stats = s;
        ability.on_start_state = None;
        ability.on_end_start_state = None;
    } 
}
#[derive(Debug)]
pub struct PlayerAbility {
    pub stats: StatList, // NOTE: COOLDOWN STAT IN HERE SHOULD BE IGNORED
    pub adjusted_time_to_charge: f32, 
    pub adjusted_cooldown: f32, 
    pub end_time_left: f32,
    pub cooldown_time_left: f32,
    pub time_to_charge_left: f32,
    pub descriptor_id: usize,
    pub end_without_end_action: bool,
    pub on_start_state: Option<AbilityStateInformation>,
    pub on_end_start_state: Option<AbilityStateInformation>
}

#[derive(Debug, Clone)]
pub struct AbilityStateInformation {
    pub ability_key_held: bool,
    pub mouse_position: MousePosition,
    pub player_position: (f32, f32),
    pub player_direction: PlayerDir,
}

pub enum PlayerAbilityDescriptorName {
    Cyclone,
    Dash
}


pub fn get_ability_descriptor(name: PlayerAbilityDescriptorName) -> PlayerAbilityDescriptor {
    match name {
        PlayerAbilityDescriptorName::Cyclone => {
            PlayerAbilityDescriptor {
                base_stats: create_stat_list!(
                    damage => StatC {
                        flat: 2.0,
                        percent: 30.0,
                    },  
                    damage => StatC { flat: 2.8, percent: 0.0},
                    width => StatC { flat: 40.0, percent: 0.0},
                    reach => StatC { flat: 40.0, percent: 0.0}
                ),
                flat_added_damage_effectiveness: 0.09,
                name: CompactString::from("Cyclone"),
                description: String::from("A cyclone of wind that knocks back enemies"),
                cooldown: 1000.0,
                time_to_charge: 100000.0,
                end_time: 0.0,
                actions: CYCLONE_ACTIONS,
                usable_with: UsableWith {
                    item_types: vec![
                        ItemType::MeleeWeapon
                    ],
                    usable_with_nothing: false,
                }
            }
        },
        PlayerAbilityDescriptorName::Dash => {
            PlayerAbilityDescriptor {
                base_stats: create_stat_list!(
                                damage => StatC {
                                    flat: 0.0,
                                    percent: 0.0,
                                }
                            ),
                flat_added_damage_effectiveness: 0.0,
                name: CompactString::from("Dash"),
                description: String::from("Dash in the direction you are moving"),
                cooldown: 50.0,
                time_to_charge: 2.0,
                end_time: 9.0,
                actions: super::player_abilities::DASH,
                usable_with: UsableWith {
                    item_types: vec![
                        ItemType::MeleeWeapon,
                        ItemType::RangedWeapon,
                        ItemType::MagicWeapon
                    ],
                    usable_with_nothing: true,
                }
            }
        }
    }

}


pub const CYCLONE_ACTIONS: PlayerAbilityActionDescriptor = PlayerAbilityActionDescriptor {
    on_start: |world, ability, state| {
        world.player.borrow_mut().player_state = PlayerState::ChargingAbility;
        world.cur_ability_charging = Some(ability);
        println!("CYCLONE CHARGING BEGAN");
        Ok(())
    },
    while_charging: |world, ability, state| {
        if !(state.ability_key_held) {
            let mut_ability_ref = punwrap!(world.inventory.get_ability_mut(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
            mut_ability_ref.time_to_charge_left = 0.0; 
        }
        let ability_ref = punwrap!(world.inventory.get_ability(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        let stats = ptry!(world.inventory.get_combined_stats());
        let pitem = world.inventory.get_cur_held_item();
        let player = world.player.borrow();
            if let Some(item) = pitem {
                if ability_ref.time_to_charge_left % 1.0 == 0.0 {
                    for i in 0..4 {
                        let angle = PI/5.0 * i as f32 + (ability_ref.adjusted_time_to_charge - ability_ref.time_to_charge_left) * 0.9 % (PI * 2.0);
                        let mut stats = ability_ref.stats.clone();
                        stats.lifetime = Some(StatC {flat: 3.0, percent: 0.0});
                        stats.width = stats.width.map(|x| 
                                StatC {
                                    flat: 40.0,
                                    percent: x.percent
                                }
                            );
                        stats.reach = stats.reach.map(|x| 
                                StatC {
                                    flat: 40.0,
                                    percent: x.percent
                                }
                            );
                        ptry!(world.add_player_attack_custom(
                            // TODO: FLAT DAMAGE EFFECTIVENESS STAT OR SMTH
                                &stats,
                                CompactString::from("melee_attack"),
                                1.0,
                                crate::game_engine::player_attacks::PlayerAttackType::MeleeAbility,
                                player.x + 16.0 + angle.cos() * 37.0,
                                player.y + 22.0 + angle.sin() * 37.0,
                                angle * 180.0/PI));
                    }
                }
            }


        println!("Charging cyclone");
        Ok(())
    },
    on_ending_start: |world, ability, state| {
        Ok(())
    },
    while_ending: |world, ability, state | {
        Ok(())
    },
    on_end: |world, ability, state| {
        let mut player_ref = world.player.borrow_mut();
        println!("END");
        if !(player_ref.player_state == PlayerState::EndingAbility) {
            return Err(perror!(Invalid, "Player State is {:?} at the end of ability charging, however it should be PlayerState::ChargingAbility", player_ref.player_state));
        }
        world.cur_ability_charging = None;
        player_ref.player_state = PlayerState::Idle;
        Ok(())
    }
};



pub const RANDOM_BIG_SHOT: PlayerAbilityActionDescriptor = PlayerAbilityActionDescriptor {
    on_start: |world, ability, state| {
        world.player.borrow_mut().player_state = PlayerState::ChargingAbility;
        world.cur_ability_charging = Some(ability);
        println!("BIG_SHOT CHARGING BEGAN");
        Ok(())
    },
    while_charging: |world, ability, state| {
        let mut_ability_ref = punwrap!(world.inventory.get_ability_mut(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        if !(state.ability_key_held) {
            mut_ability_ref.time_to_charge_left = 0.0; 
            mut_ability_ref.end_without_end_action = true;
        }
        Ok(())
    },
    on_ending_start: |world, ability, state| {
        Ok(())
    },
    while_ending: |world, ability, state | {
        Ok(())
    },
    on_end: |world, ability, state| {
        let mut player = world.player.borrow_mut();
        let ability_ref = punwrap!(world.inventory.get_ability(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        println!("END");
        if !(player.player_state == PlayerState::EndingAbility) {
            return Err(perror!(Invalid, "Player State is {:?} at the end of ability charging, however it should be PlayerState::ChargingAbility", player.player_state));
        }
        world.cur_ability_charging = None;

        let mouse_direction_unnormalized = [(state.mouse_position.x_world - player.x - 16.0), (state.mouse_position.y_world - player.y - 22.0)];
        let magnitude = f32::sqrt(mouse_direction_unnormalized[0].powf(2.0) + mouse_direction_unnormalized[1].powf(2.0));
        let mouse_direction_normalized = [
            mouse_direction_unnormalized[0] / magnitude,
            mouse_direction_unnormalized[1] / magnitude
        ];
        let main_angle = mouse_direction_normalized[1].atan2(mouse_direction_normalized[0]);
        for i in -2..=2 {
            let angle = PI/30.0 * i as f32 + main_angle;
            ptry!(world.add_player_attack_custom(
                    // TODO: FLAT DAMAGE EFFECTIVENESS STAT OR SMTH
                    &ability_ref.stats,
                    CompactString::from("spear"),
                    0.6,
                    crate::game_engine::player_attacks::PlayerAttackType::RangedAbility,
                    player.x + 16.0 + angle.cos() * 37.0,
                    player.y + 22.0 + angle.sin() * 37.0,
                    angle * 180.0/PI));
        }
        player.player_state = PlayerState::Idle;
        Ok(())
    }
};


pub const DASH: PlayerAbilityActionDescriptor = PlayerAbilityActionDescriptor {
    on_start: |world, ability, state| {
        world.player.borrow_mut().player_state = PlayerState::ChargingAbility;
        world.cur_ability_charging = Some(ability);
        Ok(())
    },
    while_charging: |world, ability, state| {
        Ok(())
    },
    on_ending_start: |world, ability, state| {
        let ability_ref = punwrap!(world.inventory.get_ability_mut(ability), Invalid, "on_ending_start was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        ability_ref.on_end_start_state = Some(state.clone());
        Ok(())
    },
    while_ending: |world, ability, state | {
        let ability_ref = punwrap!(world.inventory.get_ability(ability), Invalid, "while_ending was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        let on_ending_start_state = punwrap!(&ability_ref.on_end_start_state, Invalid, "no on end start state while calling while_ending");
        let direction_normalized_10 = match on_ending_start_state.player_direction {
            super::player::PlayerDir::Up => [0.0, -10.0],
            super::player::PlayerDir::Down => [0.0, 10.0],
            super::player::PlayerDir::Left => [-10.0, 0.0],
            super::player::PlayerDir::Right => [10.0, 0.0],
            super::player::PlayerDir::DownLeft => [-7.07106, 7.07106],
            super::player::PlayerDir::UpLeft => [-7.07107, -7.07107],
            super::player::PlayerDir::DownRight => [7.07107, 7.07107],
            super::player::PlayerDir::UpRight => [7.07107, -7.07107],

        };
        ptry!(world.attempt_move_player_ignore_damageable(&mut world.player.borrow_mut(), direction_normalized_10));
        Ok(())
    },
    on_end: |world, ability, state| {
        let mut player = world.player.borrow_mut();
        let ability_ref = punwrap!(world.inventory.get_ability(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        println!("END");
        if !(player.player_state == PlayerState::EndingAbility) {
            return Err(perror!(Invalid, "Player State is {:?} at the end of ability charging, however it should be PlayerState::ChargingAbility", player.player_state));
        }

        
        world.cur_ability_charging = None;
        player.player_state = PlayerState::Idle;
        Ok(())
    }
};
