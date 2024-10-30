use bevy::prelude::*;
use rand::Rng;
use crate::resources::*;
use crate::fish::*;
use crate::species::*;
use crate::weather::*;
use crate::fishingView::*;

pub fn calc_fish_prob(
    fish: &Fish, 
    species: &Species, 
    weather: &Res<WeatherState>, 
    time: &Res<GameDayTimer>) -> f32
    {
        let fish_hunger = fish.hunger;
        let mut a = 0.05 + (0.05*fish_hunger);
        let mut b_a = 0.;
        let mut b = 0.;
        if species.weather == weather.current_weather && (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = 0.2;
            b = (0.25)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
        }
        else if species.weather == weather.current_weather || (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = 0.1;
            if species.weather == weather.current_weather {
                b = (0.25)*(1. - (((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.));
            }
            else {
                b = (0.75)*(((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32))/24.);
            }
        }
        else{
            b_a = 0.05;
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
    state: Res<State<FishingMode>>,
    mut potential_fish: Query<(&Fish, &Species, &FishDetails, Entity), With<Fish>>,
    hooked_fish: Query<&Fish, With<Fish>>,
    weather: Res<WeatherState>,
    time: Res<GameDayTimer>,
    mut commands: Commands
    ){
        if state.eq(&FishingMode::Fishing){
            /*if !(hooked_fish.iter().count() == 0) {
                println!("Fish already hooked.");
                return;
            }*/

            for fish_info in potential_fish.iter_mut() {
                let (mut fish, species, details, entity_id) = fish_info;
                if details.touching_lure {
                    let prob = 100. * calc_fish_prob(fish, species, &weather, &time);
                    let mut prob_rng = rand::thread_rng();
                    let roll = prob_rng.gen_range(0..100);
                    println!("calculating");
                    if (roll as f32) < prob {
                        println!("fish {:?} Hit in collision zone!", details.name);
                        //commands.entity(entity_id).insert(FishHooked);
                        println!("the prob is: {}", prob);
                        return;
                    }
                    else{
                        println!("fish {:?}  collided failed to catch", details.name);
                        println!("the prob is: {}", prob);
                    }
                } 
    mut potential_fish: (&Fish, &Species),
    weather: &Res<WeatherState>,
    timer: &Res<GameDayTimer>,
    mut prob_timer: &mut ResMut<ProbTimer>,
    time: &Res<Time>
    ) -> bool {

        prob_timer.timer.tick(time.delta());
        if prob_timer.timer.just_finished() {
                let (fish, species) = potential_fish;
                let prob = 100. * calc_fish_prob(fish, species, &weather, &timer);
                let mut prob_rng = rand::thread_rng();
                let roll = prob_rng.gen_range(0..100);
                println!("Prob: {}\tRoll: {}", prob, roll);
                if (roll as f32) < prob {
                    return true;
                }
            }
            return false;      
        
    }