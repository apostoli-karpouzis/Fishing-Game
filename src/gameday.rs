use bevy::prelude::*;

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

impl GameDayTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            hour: 0,
        }
    }    
}

pub const TIME_PER_PERIOD: f32 = 10.;

pub fn run_game_timer(
    time: Res<Time>, 
    mut game_timer: ResMut<GameDayTimer>,
)
{
    game_timer.timer.tick(time.delta());
    if game_timer.timer.just_finished() {
        game_timer.hour = (game_timer.hour + 1) % 24;
        println!("Hour {}.", game_timer.hour);
    }
    
}

