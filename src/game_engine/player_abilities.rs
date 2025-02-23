use crate::world::World;
use std::f32::consts::PI;
use crate::PError;
use crate::perror;
use crate::ptry;
use crate::punwrap;
use crate::game_engine::player::PlayerState;


pub struct PlayerAbilityActionDescriptor {
    pub on_start: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub while_charging: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>,
    pub on_end: fn(world: &mut World, ability_id: usize, state: &AbilityStateInformation) -> Result<(), PError>
}



pub struct PlayerAbilityDescriptor {
    pub name: String,
    pub description: String,
    pub cooldown: f32,
    pub time_to_charge: f32, 
    pub actions: PlayerAbilityActionDescriptor,

}

#[derive(Debug)]
pub struct PlayerAbility {
    pub adjusted_time_to_charge: f32, // TODO: ACCOUNT FOR STATS WHEN CREATING THESE
    pub adjusted_cooldown: f32, // TODO: ACCOUNT FOR STATS WHEN CREATING THESE
    pub cooldown_time_left: f32,
    pub time_to_charge_left: f32,
    pub descriptor_id: usize
}


pub struct AbilityStateInformation {
    pub ability_key_held: bool,
}


pub const CYCLONE: PlayerAbilityActionDescriptor = PlayerAbilityActionDescriptor {
    on_start: |world, ability, state| {
        world.player.borrow_mut().player_state = PlayerState::ChargingAbility;
        world.cur_ability_charging = Some(ability);
        println!("CYCLONE CHARGING BEGAN");
        Ok(())
    },
    while_charging: |world, ability, state| {
        if !(state.ability_key_held) {
            let mut_ability_ref = punwrap!(world.player_abilities.get_mut(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
            mut_ability_ref.time_to_charge_left = 0.0; 
        }
        let ability_ref = punwrap!(world.player_abilities.get(ability), Invalid, "while_charging was called with ability id {}, however there is no current ability with ability id {}", ability, ability);
        let stats = ptry!(world.inventory.get_combined_stats());
        let pitem = world.inventory.get_cur_held_item();
        let player = world.player.borrow();
            if let Some(item) = pitem {
                if ability_ref.time_to_charge_left % 1.0 == 0.0 {
                    for i in 0..10 {
                        let angle = PI/5.0 * i as f32 + ability_ref.time_to_charge_left * 0.2 % (PI * 2.0);
                        ptry!(world.add_player_attack_custom(

                                &crate::create_stat_list!(
                                    lifetime => 1.0,
                                    speed => 20.0,
                                    damage => 6.0,
                                    size => 40.0,
                                ),
                                String::from("melee_attack"),
                                1.0,
                                crate::game_engine::player_attacks::PlayerAttackType::RangedAbility,
                                player.x + 16.0 + angle.cos() * 25.0,
                                player.y + 22.0 + angle.sin() * 25.0,
                                angle * 180.0/PI));
                    }
                }
                if ability_ref.time_to_charge_left % 20.0 == 0.0 {
                    for i in 0..4 {
                        let angle = PI/2.0 * i as f32 + ability_ref.time_to_charge_left * 5.0 % (PI * 2.0);
                        ptry!(world.add_player_attack_custom(

                                &crate::create_stat_list!(
                                    lifetime => 30.0,
                                    speed => 8.0,
                                    damage => 30.0,
                                    size => 40.0,
                                ),
                                String::from("melee_attack"),
                                1.0,
                                crate::game_engine::player_attacks::PlayerAttackType::RangedAbility,
                                player.x + 16.0 + angle.cos() * 25.0,
                                player.y + 22.0 + angle.sin() * 25.0,
                                angle * 180.0/PI));
                    }
                }
            }


        println!("Charging cyclone");
        Ok(())
    },
    on_end: |world, ability, state| {
        let mut player_ref = world.player.borrow_mut();
        println!("END");
        if !(player_ref.player_state == PlayerState::ChargingAbility) {
            return Err(perror!(Invalid, "Player State is {:?} at the end of ability charging, however it should be PlayerState::ChargingAbility", player_ref.player_state));
        }
        world.cur_ability_charging = None;
        player_ref.player_state = PlayerState::Idle;
        Ok(())
    }
};
