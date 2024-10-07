use bevy::{prelude::*, sprite::MaterialMesh2dBundle, math::prelude::Rectangle};
use rand::{seq::SliceRandom, Rng};



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

#[derive(Component)]
pub struct RainParticle{
    velocity: Vec2,
}

pub fn update_weather(time: Res<Time>, mut weather_state: ResMut<WeatherState>) {
    // Update weather based on time and weather state.

    // check to see if it's time to change weather.
    if weather_state.change_timer.tick(time.delta()).just_finished() {
        // Choose a random weather state from the next possible states.
        let mut rng = rand::thread_rng();
        let next_states = weather_state.current_weather.get_next_states();

        weather_state.current_weather = *next_states.choose(&mut rng).unwrap();

        println!("Weather changed to: {:?}", weather_state.current_weather);
    }
}

pub fn rain_particle_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &RainParticle, &mut Transform, &mut Sprite)>,) {
    
}
