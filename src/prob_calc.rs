use bevy::prelude::*;
use rand::Rng;
use crate::fish::*;
use crate::gameday::*;
use crate::species::*;
use crate::weather::*;

#[derive(Resource)]
pub struct ProbTimer{
    pub timer: Timer,
}

impl ProbTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }    
} 

pub fn calc_fish_prob(
    fish: &mut Fish, 
    species: &Species, 
    weather: &Res<WeatherState>, 
    time: &Res<GameDayTimer>) -> f32
    {
        let fish_hunger = fish.hunger;
        let mut a = 0.05 + (0.05*fish_hunger);
        let mut b_a = 0.;
        let mut b = 0.;
        if species.weather == weather.current_weather && (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = species.catch_prob;
            b = (0.25)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
        }
        else if species.weather == weather.current_weather || (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = species.catch_prob/2.;
            if species.weather == weather.current_weather {
                b = (0.25)*(1. - (((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.));
            }
            else {
                b = (0.75)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
            }
        }
        else{
            b_a = species.catch_prob / 4.;
            b = (0.75)*(1. - (((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.));
        }

        let mut result = (b_a*a)/b;
        println!("a = {}\nb = {}\nb_a = {}\nProb: {}", a, b, b_a, result);
        if result > 0.99 {
            result = 0.99;
        }
        
        return result;
}

pub fn hook_fish(
    mut potential_fish: (&mut Fish, &Species),
    weather: &Res<WeatherState>,
    timer: &Res<GameDayTimer>,
    mut prob_timer: &mut ResMut<ProbTimer>,
    time: &Res<Time>
    ) -> bool {

        prob_timer.timer.tick(time.delta());
        if prob_timer.timer.just_finished() {
                let (fish, species) = potential_fish;
                let prob = 100. * calc_fish_prob(fish, species, &weather, &timer);
                println!("ok");
                let mut prob_rng = rand::thread_rng();
                let roll = prob_rng.gen_range(0..100);
                println!("Prob: {}\tRoll: {}", prob, roll);
                if (roll as f32) < prob {
                    return true;
                }
            }
            return false;      
        
    }