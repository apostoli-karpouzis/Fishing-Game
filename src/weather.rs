use bevy::prelude::*;
use rand::{seq::SliceRandom,};

use crate::interface::CurrentInterface;

const WEATHER_UPDATE_PERIOD: f32 = 30.;

#[derive(Resource)]
pub struct WeatherState {
    pub current_weather: Weather,
    pub change_timer: Timer,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum Weather {
    #[default]
    Sunny,
    Rainy,
    Cloudy,
    Thunderstorm,
}

#[derive(Component)]
pub struct RainParticle{
    velocity: Vec2,
}

#[derive(Component)]
pub struct WeatherTintOverlay;

#[derive(Component)]
pub struct LightningFlash {
    duration: Timer,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current_weather: Weather::Sunny,
            change_timer: Timer::from_seconds(WEATHER_UPDATE_PERIOD, TimerMode::Repeating),
        }
    }
}

impl Weather{
    fn get_next_states(&self) -> Vec<Weather> {
        match self {
            Weather::Sunny => vec![Weather::Cloudy, Weather::Sunny],
            Weather::Cloudy => vec![Weather::Rainy, Weather::Thunderstorm, Weather::Sunny, Weather::Cloudy],
            Weather::Rainy => vec![Weather::Cloudy, Weather::Thunderstorm, Weather::Rainy],
            Weather::Thunderstorm => vec![Weather::Cloudy, Weather::Rainy, Weather::Thunderstorm],
        }
    }
}



pub fn update_weather(
    time: Res<Time>, 
    mut weather_state: ResMut<WeatherState>,
    mut next_weather: ResMut<NextState<Weather>>
) {
    // Update weather based on time and weather state.

    // check to see if it's time to change weather.
    if weather_state.change_timer.tick(time.delta()).just_finished() {
        // Choose a random weather state from the next possible states.
        let mut rng = rand::thread_rng();
        let next_states = weather_state.current_weather.get_next_states();

        weather_state.current_weather = *next_states.choose(&mut rng).unwrap();
        next_weather.set(weather_state.current_weather);
    }
}

// pub fn rain_particle_system(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query: Query<(Entity, &RainParticle, &mut Transform, &mut Sprite)>,
// ) {
//     //TODO: implement rain_particle_system
// }

pub fn update_weather_tint(weather_state: Res<WeatherState>, 
    current_interface: Res<State<CurrentInterface>>,
    mut query: Query<&mut Sprite, (With<WeatherTintOverlay>, Without<LightningFlash>)>,
) {
    if let Ok(mut sprite) = query.get_single_mut() {
        if current_interface.eq(&CurrentInterface::Shop) {
            sprite.color = Color::srgba(0.5, 0.5, 0.5, 0.0);
        } else {
            match weather_state.current_weather {
                Weather::Cloudy => { 
                    sprite.color = Color::srgba(0.4, 0.4, 0.4, 0.25);
                },
                Weather::Rainy => { 
                    sprite.color = Color::srgba(0.4, 0.4, 0.4, 0.5);
                },
                Weather::Thunderstorm => { 
                    sprite.color = Color::srgba(0.4, 0.4, 0.4, 0.6);
                },
                _ => {
                    sprite.color = Color::srgba(0.5, 0.5, 0.5, 0.0);
                }
            }
        }
    }
}

pub fn spawn_weather_tint_overlay(mut commands: Commands){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.5, 0.5, 0.5, 0.0),
            custom_size: Some(Vec2::new(10000.,10000.)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.),
       ..default()
    },
    WeatherTintOverlay
    ));
}
