use bevy::{prelude::*, utils::HashMap};
use rand::seq::SliceRandom;
use rand::prelude::*;

use crate::{interface::CurrentInterface, player::Player, window::{WIN_H, WIN_W}};

const WEATHER_UPDATE_PERIOD: f32 = 20.;

#[derive(Event)]
pub struct RegionChangedEvent(pub Region);

#[derive(Resource)]
pub struct CurrentRegion(pub Region);

#[derive(Debug,Copy, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum Region{
    #[default]
    West,
    Central,
    Shore,
}

#[derive(Resource)]
pub struct WeatherState {
    pub weather_by_region: HashMap<Region, Weather>,
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


impl Default for WeatherState {
    fn default() -> Self {
        let mut weather_by_region = HashMap::new();
        for region in [Region::West, Region::Central, Region::Shore].iter() {
            weather_by_region.insert(region.clone(), Weather::Sunny);
        }
        Self{
            weather_by_region,
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
    mut next_weather: ResMut<NextState<Weather>>,
    current_region: Res<State<Region>>,
) {
    // Update weather based on time and weather state.

    // check to see if it's time to change weather.
    if weather_state.change_timer.tick(time.delta()).just_finished() {
        // Choose a random weather state from the next possible states.
        let mut rng = rand::thread_rng();
        for (_region, current_weather) in weather_state.weather_by_region.iter_mut(){
            let next_states = current_weather.get_next_states();
            *current_weather = *next_states.choose(&mut rng).unwrap();
        }
        
        if let Some(current_weather) = weather_state.weather_by_region.get(current_region.get()){
            next_weather.set(*current_weather);
        }
    }
}

pub fn run_if_raining( weather_state: Res<WeatherState>, current_region: Res<State<Region>>) -> bool{
    let current_weather = weather_state.weather_by_region.get(current_region.get()).unwrap_or(&Weather::Sunny);
    return *current_weather == Weather::Rainy || *current_weather == Weather::Thunderstorm;
}
pub fn rain_particle_system(
    mut commands: Commands,
    mut query: Query<(Entity, &RainParticle, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {

    
    
    let (window_width, window_height) = (8.5*WIN_W, 8.5*WIN_H);

    for (_entity, particle, mut transform, mut _sprite) in query.iter_mut() {
        let mut rng = rand::thread_rng();
        // Update position based on velocity
        transform.translation.x += particle.velocity.x * time.delta_seconds();
        transform.translation.y += particle.velocity.y * time.delta_seconds();

        // Check if particle is out of bounds
        if transform.translation.y < -window_height / 2.0 {
            // Respawn at the top with random x position
            let x: f32 = rng.gen();
            transform.translation.y = window_height / 2.0;
            transform.translation.x = x * window_width - window_width / 2.0;
        }

        
        let wind:  f32 = rng.gen();
        transform.translation.x += wind;

    }

    // Spawn new particles if needed
    if query.iter().count() < 1000 { // Adjust this number as needed
        spawn_rain_particle(&mut commands,  window_width, window_height);
    }
}

fn spawn_rain_particle(
    commands: &mut Commands,
    window_width: f32,
    window_height: f32,
) {
    let x = random::<f32>() * window_width - window_width / 2.0;
    let y = window_height / 2.0 + random::<f32>() * 100.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                color: Color::srgba(0.7, 0.7, 0.9, 0.8),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            }, // Adjust size as needed
            transform: Transform::from_translation(Vec3::new(x, y, 999.0)),
            ..Default::default()
        },
        RainParticle {
            velocity: Vec2::new(0.0, -750.0 - random::<f32>() * 100.0), // Adjust speed as needed
        },
    ));
}

pub fn update_weather_tint(
    weather_state: Res<WeatherState>, 
    current_interface: Res<State<CurrentInterface>>,
    current_region: Res<State<Region>>,
    mut query: Query<&mut Sprite, With<WeatherTintOverlay>>,
) {
    if let Ok(mut sprite) = query.get_single_mut() {
        if current_interface.eq(&CurrentInterface::Shop) {
            sprite.color = Color::srgba(0.5, 0.5, 0.5, 0.0);
        } else {
            let current_weather = weather_state.weather_by_region.get(current_region.get()).unwrap_or(&Weather::Sunny);
            match current_weather {
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
            custom_size: Some(Vec2::new(15000.,15000.)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.),
       ..default()
    },
    WeatherTintOverlay
    ));
}

pub fn update_player_region(
    player_query: Query<&GlobalTransform, With<Player>>,
    current_region: ResMut<State<Region>>,
    mut next_region: ResMut<NextState<Region>>,
    mut region_changed_event: EventWriter<RegionChangedEvent>,
){
    if let Ok(player_transform) = player_query.get_single(){
        let player_position = player_transform.translation();
        let new_region = determine_region(player_position);

        if current_region.get() != &new_region {
            next_region.set(new_region);
            region_changed_event.send(RegionChangedEvent(new_region));
            println!("Player moved to region: {:?}", current_region);
        }
    }
}

pub fn handle_region_change(
    mut commands: Commands,
    mut region_changed_events: EventReader<RegionChangedEvent>,
    weather_state: Res<WeatherState>,
    rain_particles: Query<Entity, With<RainParticle>>,
){
    for event in region_changed_events.read(){
        let new_region = event.0;
        let current_weather = weather_state.weather_by_region.get(&new_region).unwrap();
        
        if *current_weather != Weather::Rainy || *current_weather != Weather::Thunderstorm {
            for entity in rain_particles.iter(){
                commands.entity(entity).despawn();
            }
        }
    }
}

fn determine_region(player_position: Vec3) -> Region {
    if player_position.x <= 1.5* WIN_W {
        return Region::West;
    } else if player_position.x > 1.5*WIN_W && player_position.x <= 3.5 * WIN_W {
        return Region::Central;
    }else{
        return Region::Shore;
    }
}