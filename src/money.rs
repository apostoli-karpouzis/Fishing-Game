use bevy::prelude::*;
use crate::resources::*;

#[derive(Component)]
pub struct MoneyDisplay;

pub fn spawn_money_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Money: 0",
            TextStyle {
                font: asset_server.load("pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(2.0),
            left: Val::Px(5.0),
            ..default()
        }),
        MoneyDisplay,
    ));
}


pub fn update_money_display(
    playerInventory: Query<&mut PlayerInventory>,
    mut query: Query<&mut Text, With<MoneyDisplay>>,
) {
    let mut text = query.single_mut();
    let inventory_info = playerInventory.single();
    text.sections[0].value = format!("Money: {}", inventory_info.coins);
}