use bevy::prelude::*;
use crate::shop::*;

#[derive(Component)]
pub struct PlayerInventory {
    pub coins: u32,
    pub items: Vec<ShopItem>,
    pub rods: Vec<ShopItem>,
    pub lures: Vec<ShopItem>,
    pub lines: Vec<ShopItem>,
    pub cosmetics: Vec<ShopItem>,
    pub rod_index: usize,
    pub lure_index: usize,
    pub line_index: usize,
}

pub fn handle_inventory (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_inventory: Query<&mut PlayerInventory>
) {    
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    let inventory = player_inventory.single();
        
    for rod in inventory.rods.iter() {
        println!("rods: {}", rod.name);
    }

    for lure in inventory.lures.iter() {
        println!("lures: {}", lure.name);
    }

    for line in inventory.lines.iter() {
        println!("lines: {}", line.name);
    }

    for cosmetic in inventory.cosmetics.iter() {
        println!("cosmetics: {}", cosmetic.name);
    }
}