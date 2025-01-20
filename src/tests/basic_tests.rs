#![cfg(test)]
use crate::tests::tests::{basic_world, basic_camera};
use super::lib::headless::HeadlessGame;

#[tokio::test]
async fn test_nothing_happens_in_blank_world(){
    let mut world = basic_world().await;
    let camera = basic_camera(&mut world).await;
    let starting_x = world.player.borrow().x;
    let starting_y = world.player.borrow().y;
    let mut headless = HeadlessGame::new(world, camera);
    headless.run(1000).await;

    assert!(
        headless.world.player.borrow().x == starting_x,
        "Player X should stay constant without input"
    );
    assert!(
        headless.world.player.borrow().y == starting_y,
        "Player Y should stay constant without input"
    );
    assert!(
        headless.world.player.borrow().health == headless.world.player.borrow().max_health as f32,
        "Player should not take damage in a blank world"
    );
}