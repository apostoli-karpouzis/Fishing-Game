use bevy::prelude::*;
use crate::resources::*;
use crate::weather::*;

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct ClockDisplay;

#[derive(Component)]
pub struct WeatherDisplay;

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
            top: Val::Px(0.0),
            left: Val::Px(5.0),
            ..default()
        }),
        MoneyDisplay,
    ));
}

pub fn spawn_clock_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Time: 0",
            TextStyle {
                font: asset_server.load("pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ClockDisplay,
    ));
}

pub fn spawn_weather_display(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Weather: 0",
            TextStyle {
                font: asset_server.load("pixel.ttf"),
                font_size: 65.0,
                color: Color::srgb(0.0, 0.0, 0.0),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(5.0),
            ..default()
        }),
        WeatherDisplay,
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

pub fn update_clock_display(
    time: Res<GameDayTimer>,
    mut query: Query<(&mut Text, &mut Visibility), With<ClockDisplay>>,
    shop_state: Res<ShopState>,
) {
    let (mut text, mut visibility) = query.single_mut();
    text.sections[0].value = format!("Hour: {}", time.hour);
    if shop_state.is_open {
        *visibility = Visibility::Hidden;
    }
    else {
        *visibility = Visibility::Visible;
    }
}

pub fn update_weather_display(
    weather: Res<WeatherState>,
    mut query: Query<(&mut Text, &mut Visibility), With<WeatherDisplay>>,
    shop_state: Res<ShopState>,
) {
    let (mut text, mut visibility) = query.single_mut();
    if shop_state.is_open {
        *visibility = Visibility::Hidden;
    }
    else {
        *visibility = Visibility::Visible;
    }
    match weather.current_weather {
        Weather::Cloudy => { 
            text.sections[0].value = format!("Weather: Cloudy");
        },
        Weather::Rainy => { 
            text.sections[0].value = format!("Weather: Rainy");
        },
        Weather::Thunderstorm => { 
            text.sections[0].value = format!("Weather: Thunderstorm");
        },
        _ => {
            text.sections[0].value = format!("Weather: Sunny");
        }
    }
}