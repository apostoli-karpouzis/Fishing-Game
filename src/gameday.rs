use bevy::prelude::*;
use crate::resources::*;

#[derive(Component)]
pub struct Timer;

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

