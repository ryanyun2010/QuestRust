use crate::rendering_engine::abstractions::SpriteIDContainer;

use super::{json_parsing::ParsedData, player::{self, Player}, world::World};

pub fn generate_world_from_json_parsed_data(data: &ParsedData) -> (World, SpriteIDContainer) {

    
    let starting_level_descriptor = data.starting_level_descriptor.clone();
    let player_descriptor = starting_level_descriptor.player;
    let mut world = World::new(Player::new(player_descriptor.x, player_descriptor.y, player_descriptor.health, player_descriptor.max_health, player_descriptor.movement_speed, data.get_texture_id(&player_descriptor.sprite)));
    let mut sprites = SpriteIDContainer::generate_from_json_parsed_data(data, &mut world);
    for entity_descriptor in starting_level_descriptor.entities.iter(){
        let entity = world.create_entity_from_json_archetype(entity_descriptor.x, entity_descriptor.y, &entity_descriptor.archetype, data);
        world.set_sprite(entity, sprites.get_sprite(&entity_descriptor.sprite));
    }
    (world, sprites)
}