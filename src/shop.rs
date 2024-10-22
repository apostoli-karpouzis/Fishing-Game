use bevy::prelude::*;
use crate::resources::*;
use crate::player::*;



// Define the coordinates for the shop and player positions when in shop mode
pub const SHOP_X: f32 = 500.0;  
pub const SHOP_Y: f32 = 300.0;  
pub const SHOP_PLAYER_X: f32 = 520.0;  
pub const SHOP_PLAYER_Y: f32 = 320.0;  

// Function for entering ShopMode
pub fn shop_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut return_val: ResMut<PlayerReturnPos>,
) {
    let mut camera_transform = camera.single_mut();
    let mut player_transform = player.single_mut();

    // Save player position before entering the shop
    return_val.player_save_x = player_transform.translation.x;
    return_val.player_save_y = player_transform.translation.y;

    // Move the camera and player to the shop area
    camera_transform.translation.x = SHOP_X; 
    camera_transform.translation.y = SHOP_Y; 
    player_transform.translation.x = SHOP_PLAYER_X; 
    player_transform.translation.y = SHOP_PLAYER_Y; 

    println!("Entered Shop Mode");
}


pub fn exit_shop_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut player: Query<&mut Transform, With<Player>>,
    return_val: ResMut<PlayerReturnPos>,
) {
    let mut camera_transform = camera.single_mut();
    let mut player_transform = player.single_mut();

    // Restore camera and player position to where they were before entering the shop
    camera_transform.translation.x = return_val.player_save_x;
    camera_transform.translation.y = return_val.player_save_y;
    player_transform.translation.x = return_val.player_save_x;
    player_transform.translation.y = return_val.player_save_y;

    println!("Exited Shop Mode");
}
