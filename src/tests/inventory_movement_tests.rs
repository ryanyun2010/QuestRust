
#![cfg(test)]

use crate::{game_engine::game::MousePosition, tests::tests::{basic_camera, basic_world}};


#[tokio::test]
async fn test_inventory_item_move(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let mut headless = crate::tests::lib::headless::HeadlessGame::new(world, camera);
    headless.world.inventory.show_inventory(&mut headless.camera);
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
    headless.world.inventory.show_inventory(&mut headless.camera);
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
    headless.world.inventory.show_inventory(&mut headless.camera);
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
    
    headless.world.inventory.hide_inventory(&mut headless.camera);
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
    headless.world.inventory.show_inventory(&mut headless.camera);
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
    headless.world.inventory.hide_inventory(&mut headless.camera);
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