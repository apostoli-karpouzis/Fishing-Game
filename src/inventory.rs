use bevy::prelude::*;
use crate::shop::*;

#[derive(Component)]
pub struct PlayerInventory {
    pub coins: u32,
    pub items: Vec<ShopItem>,
    pub lures: Vec<ShopItem>,
    pub lines: Vec<ShopItem>,
    pub lure_index: usize,
    pub line_index: usize,
}

pub fn handle_inventory (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_inventory: Query<&mut PlayerInventory>
) {
    let inventory = player_inventory.single();
    let items: Vec<_> = inventory.items.iter().collect();
    let lures: Vec<_> = inventory.lures.iter().collect();
    let lines: Vec<_> = inventory.lines.iter().collect();
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        for item in items {
            println!("{}", item.name);
        }

        for lure in lures{
            println!("lures: {}", lure.name);
        }
        for line in lines{
            println!("lines: {}", line.name);
        }
    }
}