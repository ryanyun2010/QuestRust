#![cfg(test)]
use crate::tests::{lib::headless::HeadlessGame, tests::{basic_world, basic_camera}};

use super::lib::headless;

#[tokio::test]
async fn test_player_movement_right() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x > player_starting_x,
        "Player should move right when D is pressed"
    );
}

#[tokio::test]
async fn test_player_movement_left() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("a"), true);
    let player_starting_x = headless.world.player.borrow().x;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x < player_starting_x,
        "Player should move left when A is pressed"
    );
}

#[tokio::test]
async fn test_player_movement_down() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("s"), true);
    let player_starting_y = headless.world.player.borrow().y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().y > player_starting_y,
        "Player should move down when S is pressed"
    );
}

#[tokio::test]
async fn test_player_movement_up() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("w"), true);
    let player_starting_y = headless.world.player.borrow().y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().y < player_starting_y,
        "Player should move up when W is pressed"
    );
}

#[tokio::test]
async fn test_movement_both_d_and_a() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("d"), true);
    headless.state.keys_down.insert(String::from("a"), true);
    let player_starting_x = headless.world.player.borrow().x;
    let player_starting_y = headless.world.player.borrow().y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x == player_starting_x && headless.world.player.borrow().y == player_starting_y,
        "Player should not move when both D and A are pressed"
    );
}

#[tokio::test]
async fn test_movement_both_w_and_s() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("w"), true);
    headless.state.keys_down.insert(String::from("s"), true);
    let player_starting_x = headless.world.player.borrow().x;
    let player_starting_y = headless.world.player.borrow().y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x == player_starting_x && headless.world.player.borrow().y == player_starting_y,
        "Player should not move when both W and S are pressed"
    );
}

#[tokio::test]
async fn test_movement_w_and_d() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("w"), true);
    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x + headless.world.player.borrow().frac_x;
    let player_starting_y = headless.world.player.borrow().y + headless.world.player.borrow().frac_y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x > player_starting_x && headless.world.player.borrow().y + headless.world.player.borrow().frac_y < player_starting_y,
        "Player should move right and up when W and D are pressed"
    );
    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x - player_starting_x == player_starting_y - headless.world.player.borrow().y - headless.world.player.borrow().frac_y ,
        "Player should move right and up equal amounts when W and D are pressed"
    );
}

#[tokio::test]
async fn test_movement_w_and_a() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("w"), true);
    headless.state.keys_down.insert(String::from("a"), true);
    let player_starting_x = headless.world.player.borrow().x + headless.world.player.borrow().frac_x;
    let player_starting_y = headless.world.player.borrow().y + headless.world.player.borrow().frac_y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x < player_starting_x && headless.world.player.borrow().y + headless.world.player.borrow().frac_y < player_starting_y,
        "Player should move left and up when W and A are pressed"
    );
    assert!(
        player_starting_x - headless.world.player.borrow().x - headless.world.player.borrow().frac_x == player_starting_y - headless.world.player.borrow().y - headless.world.player.borrow().frac_y,
        "Player should move left and up equal ammounts when W and A are pressed"
    );
}

#[tokio::test]
async fn test_movement_s_and_d() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("s"), true);
    headless.state.keys_down.insert(String::from("d"), true);
    let player_starting_x = headless.world.player.borrow().x + headless.world.player.borrow().frac_x;
    let player_starting_y = headless.world.player.borrow().y + headless.world.player.borrow().frac_y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x > player_starting_x && headless.world.player.borrow().y + headless.world.player.borrow().frac_y > player_starting_y,
        "Player should move right and down when S and D are pressed"
    );
    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x - player_starting_x == headless.world.player.borrow().y + headless.world.player.borrow().frac_y - player_starting_y,
        "Player should move right and down equal amounts when S and D are pressed"
    );
}

#[tokio::test]
async fn test_movement_s_and_a() {
    let world = basic_world().await;
    let camera = basic_camera().await;
    let mut headless = HeadlessGame::new(world, camera);

    headless.state.keys_down.insert(String::from("s"), true);
    headless.state.keys_down.insert(String::from("a"), true);
    let player_starting_x = headless.world.player.borrow().x + headless.world.player.borrow().frac_x;
    let player_starting_y = headless.world.player.borrow().y + headless.world.player.borrow().frac_y;
    headless.run(20).await;

    assert!(
        headless.world.player.borrow().x + headless.world.player.borrow().frac_x < player_starting_x && headless.world.player.borrow().y + headless.world.player.borrow().frac_y > player_starting_y,
        "Player should move left and down when S and A are pressed"
    );
    assert!(
        player_starting_x - headless.world.player.borrow().x - headless.world.player.borrow().frac_x == headless.world.player.borrow().y + headless.world.player.borrow().frac_y - player_starting_y,
        "Player should move left and down equal amounts when S and A are pressed"
    );
}