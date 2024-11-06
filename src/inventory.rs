use bevy::prelude::*;
use crate::shop::*;

#[derive(Component)]
pub struct PlayerInventory {
    pub coins: u32,
    pub items: Vec<String>,
}

pub fn handle_inventory (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_inventory: Query<&mut PlayerInventory>,
    shop_state: Res<ShopState>,
) {
    let inventory = player_inventory.single();
    let items: Vec<_> = inventory.items.iter().collect();
    if keyboard_input.just_pressed(KeyCode::KeyE) && !shop_state.is_open{
        for item in items {
            println!("{}", item);
        }
    }
}