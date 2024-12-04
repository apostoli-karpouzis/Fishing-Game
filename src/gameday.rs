use bevy::prelude::*;
use crate::interface::CurrentInterface;

#[derive(Component, PartialEq)]
pub enum TimePeriod{
    Morning,
    Afternoon,
    Night,
}

#[derive(Resource)]
pub struct GameDayTimer {
    pub timer: Timer,
    pub hour: i32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MidnightState {
    #[default]
    NotMidnight,
    Midnight
}


impl GameDayTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            hour: 0,
        }
    }    
}

#[derive(Component)]
pub struct DayTintOverlay;

pub const TIME_PER_PERIOD: f32 = 10.;

pub fn run_game_timer(
    time: Res<Time>, 
    mut game_timer: ResMut<GameDayTimer>,
    mut next_state: ResMut<NextState<MidnightState>>,
)
{
    game_timer.timer.tick(time.delta());
    if game_timer.timer.just_finished() {
        game_timer.hour = (game_timer.hour + 1) % 24;
        println!("Hour {}.", game_timer.hour);
        if game_timer.hour == 23{
            println!("entering midnight state");
            next_state.set(MidnightState::Midnight);

        }
        else{
            println!("exiting midnight state");
            next_state.set(MidnightState::NotMidnight);
        }
    }
    
}

pub fn day_tint(
    timer: Res<GameDayTimer>,
    current_interface: Res<State<CurrentInterface>>,
    mut day_overlay: Query<&mut Sprite, With<DayTintOverlay>>
) {
    if let Ok(mut sprite) = day_overlay.get_single_mut() {
        if current_interface.eq(&CurrentInterface::Shop){
            sprite.color = Color::srgba(0.5, 0.5, 0.5, 0.0);
        }
        else{
            if timer.hour > 5 && timer.hour < 19 {
                sprite.color = Color::srgba(0.5, 0.5, 0.5, 0.0);
            }
            else if timer.hour == 5 || timer.hour == 19{
                sprite.color = Color::srgba(0.1, 0.1, 0.3, 0.5);
            }
            else {
                sprite.color = Color::srgba(0.1, 0.1, 0.3, 0.7);
            }
        }
    }
}

pub fn spawn_day_tint_overlay(mut commands: Commands){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.5, 0.5, 0.5, 0.0),
            custom_size: Some(Vec2::new(20000.,20000.)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.),
       ..default()
    },
    DayTintOverlay
    ));
}
