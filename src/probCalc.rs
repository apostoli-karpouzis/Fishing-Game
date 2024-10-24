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
        let mut a = 0.6 - (0.05*fish_hunger);
        let mut b_a = 0.;
        let mut b = 0.;
        if species.weather == weather.current_weather && (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = 0.8;
            b = (0.25)*((24.-((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32)))/24.);
        }
        else if species.weather == weather.current_weather || (time.hour >= (species.time_of_day.0 as i32) && time.hour <= (species.time_of_day.1 as i32)) {
            b_a = 0.3;
            if species.weather == weather.current_weather {
                b = (0.25)*(1. - ((24.-((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32)))/24.));
            }
            else {
                b = (0.75)*((24.-((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32)))/24.);
            }
        }
        else{
            b_a = 0.05;
            b = (0.75)*(1. - ((24.-((species.time_of_day.1 as f32)-(species.time_of_day.0 as f32)))/24.));
        }

        let mut result = (b_a*a)/b;
        //println!("a = {}\nb = {}\nb_a = {}\nProb: {}", a, b, b_a, result);
        if(result > 0.99){
            result = 0.99;
        }
        return result;
}

pub fn hook_fish(
    state: Res<State<FishingMode>>,
    mut potential_fish: Query<(&Fish, &Species, Entity), With<Fish>>,
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
                let (mut fish, species, entity_id) = fish_info;
                if fish.touching_lure {
                    let prob = 100. * calc_fish_prob(fish, species, &weather, &time);
                    let mut prob_rng = rand::thread_rng();
                    let roll = prob_rng.gen_range(0..100);
                    if (roll as f32) < prob {
                        println!("Hit in collision zone!");
                        //commands.entity(entity_id).insert(FishHooked);
                        return;
                    }
                } 
            }
        }

    }