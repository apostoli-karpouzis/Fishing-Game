use bevy::{prelude::*, utils::HashMap};
use crate::weather::*;

#[derive(Component)]
pub struct Fish {
    pub name: &'static str,
    pub id: u32,
    pub is_caught: bool,
    pub is_alive: bool,
    pub touching_lure: bool,
    pub length: f32,
    pub width: f32,
    pub weight: f32,
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    //bounds
    pub depth: (i32, i32),
    //x, y, z
    pub position: (i32, i32),
    pub change_x: Vec3,
    pub change_y: Vec3,
    //length, width, depth
    pub bounds: (i32, i32),
    pub age: f32,
    pub hunger: f32
}

impl Fish {
    pub fn new(
        name: &'static str,
        id: u32, 
        is_caught: bool, 
        is_alive: bool, 
        touching_lure: bool, 
        length: f32, 
        width: f32, 
        weight: f32,
        time_of_day: (usize, usize), 
        weather: Weather,
        depth: (i32, i32),
        position: (i32, i32),
        change_x: Vec3,
        change_y: Vec3,
        bounds: (i32, i32),
        age: f32, 
        hunger: f32) -> Self {
        Self {name, id, is_caught, is_alive, touching_lure, length, width, weight, time_of_day, weather, depth, position, change_x, change_y, bounds, age, hunger }
    }
    
    //call when fish die
    pub fn die(&self) {
        println!("fish {} is swimming to fish heaven", self.id);
    }
    //increase age and decrease hunger when caught
    pub fn hooked_fish(&mut self) {
        self.age += 10.0;           //age increase when caught, tough fight
        self.hunger -= 10.0;        //less likely to go for bait when caught

        //check if dead
        if self.age >= 100.0 {
            self.die();
        }
    }
    //call every in game hour/day whatever
    pub fn update_fish_traits(&mut self) {
        self.age += 1.0;           //age increase hourly
        self.hunger -= 1.0;        //hunger increases hourly
    }
    //calc fish anger
    pub fn fish_anger(&mut self) -> f32 {
        return self.age * self.hunger;
    }
    pub fn fish_weight(&mut self) -> f32 {
        return self.age * self.hunger;
    }
}

#[derive(Default)]
pub struct Pond {
    pub fish_population: HashMap<u32, Fish>, // store specific fish by id
}

impl Pond {
    //get specific fish
    pub fn get_fish(&self, id: u32) -> Option<&Fish> {
        self.fish_population.get(&id)
    }

    //age fish consistently
    pub fn age_all_fish(&mut self) {
        for fish in self.fish_population.values_mut() {
            fish.update_fish_traits();
        }
    }
}

