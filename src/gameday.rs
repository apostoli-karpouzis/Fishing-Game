use bevy::prelude::*;
use crate::resources::*;

#[derive(Component)]
pub struct Timer;

pub const TIME_PER_PERIOD: f32 = 30.;

pub fn run_game_timer(
    time: Res<Time>, 
    mut game_timer: ResMut<GameDayTimer>,
)
{
    game_timer.timer.tick(time.delta());
    if game_timer.timer.just_finished() {
        if game_timer.time_period == TimePeriod::Morning {
            game_timer.time_period = TimePeriod::Afternoon;
            println!("It is afternoon");
        }
        else if game_timer.time_period == TimePeriod::Afternoon {
            game_timer.time_period = TimePeriod::Night;
            println!("It is night");
        }
        else{
            game_timer.time_period = TimePeriod::Morning;
            println!("It is morning");
        }
    }
}

