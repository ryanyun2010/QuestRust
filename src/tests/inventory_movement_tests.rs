
#![cfg(test)]

use crate::{create_stat_list, game_engine::{entity_components::CollisionBox, game::MousePosition, item::{Item, ItemType}}, tests::{lib::headless::HeadlessGame, tests::{basic_camera, basic_world}}};

async fn test_inventory_clicking_blank_slot_in_blank_inventory(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory();
    for i in 0..headless.world.inventory.slots.len(){
        assert!(
            headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
            "There should be no item in slot {}",
            i
        );
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    headless.run(5).await;

    for i in 0..headless.world.inventory.slots.len(){
        assert!(
            headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
            "There should be no item in slot {}",
            i
        );
    }
}
#[tokio::test]
async fn test_inventory_clicking_blank_slot_in_inventory_with_items(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.set_slot_item(7, 1);
    headless.world.inventory.set_slot_item(8, 0);
    headless.world.inventory.show_inventory();
    assert!(
        headless.world.inventory.get_slot(&7).unwrap().item.is_some(),
        "There should be an item in slot 7"
    );
    assert!(
        headless.world.inventory.get_slot(&7).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 7"
    );
    assert!(
        headless.world.inventory.get_slot(&8).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 8"
    );
    assert!(
        headless.world.inventory.get_slot(&8).unwrap().item.is_some(),
        "There should be an item in slot 8"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 7 && i != 8 {
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    headless.run(5).await;

    assert!(
        headless.world.inventory.get_slot(&7).unwrap().item.is_some(),
        "There should be an item in slot 7"
    );
    assert!(
        headless.world.inventory.get_slot(&7).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 7"
    );
    assert!(
        headless.world.inventory.get_slot(&8).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 8"
    );
    assert!(
        headless.world.inventory.get_slot(&8).unwrap().item.is_some(),
        "There should be an item in slot 8"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 7 && i != 8 {
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }

}


#[tokio::test]
async fn test_inventory_item_move(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory();
    headless.world.inventory.set_slot_item(6, 1);
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_none(),
        "There should be no item in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 6"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 5 && i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    headless.run(5).await;
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_some(),
        "There should be an item in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_none(),
        "There should be no item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 5"
    );

    for i in 0..headless.world.inventory.slots.len(){
        if i != 5 && i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    
}

#[tokio::test]
async fn test_inventory_item_swap(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory();
    headless.world.inventory.set_slot_item(6, 1);
    headless.world.inventory.set_slot_item(5, 0);
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_some(),
        "There should be an item in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 5"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 5 && i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 6"
    );

    for i in 0..headless.world.inventory.slots.len(){
        if i != 5 && i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
}

#[tokio::test]
async fn test_close_inventory_with_item_held_basic(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory();
    headless.world.inventory.set_slot_item(6, 1);
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 6"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    
    headless.world.inventory.hide_inventory();
    headless.run(5).await;
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 6"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 6{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
}
#[tokio::test]
async fn test_close_inventory_with_item_held_after_swap(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory();
    headless.world.inventory.set_slot_item(6, 1);
    headless.world.inventory.set_slot_item(5, 0);
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_some(),
        "There should be an item in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 5"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 6 && i != 5{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, false, false);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 201.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false); 
    headless.run(5).await;
    headless.world.inventory.hide_inventory();
    headless.run(5).await;
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.is_some(),
        "There should be an item in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&6).unwrap().item.unwrap() == 0,
        "Item 0 should be in slot 6"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.is_some(),
        "There should be an item in slot 5"
    );
    assert!(
        headless.world.inventory.get_slot(&5).unwrap().item.unwrap() == 1,
        "Item 1 should be in slot 5"
    );
    for i in 0..headless.world.inventory.slots.len(){
        if i != 6 && i != 5{
            assert!(
                headless.world.inventory.get_slot(&i).unwrap().item.is_none(),
                "There should be no item in slot {}",
                i
            );
        }
    }
}

#[tokio::test]
pub async fn test_melee_player_attack_after_inventory_movement() {
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    world.add_entity_archetype(String::from("test_attackable_entity"), vec![
        crate::game_engine::entities::EntityTags::BaseHealth(100),
        crate::game_engine::entities::EntityTags::Damageable(
            CollisionBox {
                x_offset: 0.0,
                y_offset: 0.0,
                w: 32.0,
                h: 32.0
            }
        )
    ]);
    let item = world.inventory.add_item(
        Item {
            name: String::from("test_sword"),
            attack_sprite: String::from("melee_attack"),
            item_type: ItemType::MeleeWeapon,
            lore: String::from("test"),
            sprite: String::from("sword"),
            stats: create_stat_list!(
                damage => 150.0,
                width => 50.0,
                reach => 65.0
            )
        }
    );
    world.create_entity_with_archetype(639.0, 400.0, String::from("test_attackable_entity"));
    world.inventory.set_slot_item(6, item);
    world.inventory.show_inventory();
    let mut headless = HeadlessGame::new(world, camera);
    headless.run(5).await;
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 579.0,
        y_screen: 201.0,
        x_world: 579.0 + headless.camera.camera_x,
        y_world: 201.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.process_mouse_input(MousePosition {
        x_screen: 521.0,
        y_screen: 381.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 381.0 + headless.camera.camera_y,
    }, false, false);    
    headless.run(5).await; 
    headless.world.inventory.on_mouse_click(MousePosition{
        x_screen: 521.0,
        y_screen: 381.0,
        x_world: 521.0 + headless.camera.camera_x,
        y_world: 381.0 + headless.camera.camera_y,
    }, true, false);
    headless.run(5).await;
    headless.world.inventory.hide_inventory();
    headless.run(5).await;
    headless.world.on_mouse_click(MousePosition {
            x_screen: 639.0,
            y_screen: 400.0,
            x_world: 639.0 + headless.camera.camera_x,
            y_world: 400.0 + headless.camera.camera_y,
    }, true, false, headless.camera.viewpoint_width as f32, headless.camera.viewpoint_height as f32);
    assert!(
        headless.world.entity_health_components.get(&0).is_some(),
        "Entity should have a health component prior to player attack"
    );
    headless.run(20).await;
    assert!(
        headless.world.entity_health_components.get(&0).is_none(),
        "Entity should be killed by player attack"
    );
}

