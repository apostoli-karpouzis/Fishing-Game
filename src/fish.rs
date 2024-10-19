use std::collections::HashSet;
use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Default)] 
pub struct FishSpecies {
    pub length: f32,                    //fish length
    pub width: f32,                     //fish aerodynamix
    pub cd: f32,
    pub time_of_day: HashSet<u32>,      //all the hours they prefer (o1) 0-23
    pub weather: HashSet<String>,       //all the weathers they prefer (o1)
    pub depth: (f32, f32),              //preferred depth range
}

impl FishSpecies {
    pub fn new(length: f32, width: f32, cd: f32, time_of_day: HashSet<u32>, weather: HashSet<String>, depth: (f32, f32)) -> Self {
        Self { length, width, cd, time_of_day, weather, depth }
    }
    
    //check if time is preffered
    pub fn is_preferred_time(&self, hour: u32) -> bool {
        self.time_of_day.contains(&hour)
    }
    //check if weather is preferred
    pub fn is_preferred_weather(&self, weather: &str) -> bool {
        self.weather.contains(weather)
    }
    //check if depth is preferred for fish
    pub fn is_preferred_depth(&self, depth: f32) -> bool {
        depth >= self.depth.0 && depth <= self.depth.1
    }

    pub fn fish_shape(&mut self) {
        //do some calculation based on length width maybe
        //prob a task for physics 
        self.length;
        self.width;
    }
}

#[derive(Component)]
pub struct FishState {
    pub id: u32,
    pub is_alive: bool,
    pub weight: f32,
    pub age: f32,
    pub hunger: f32,
    pub velocity: Vec3,
    pub position: Vec3,
    pub forces: Forces
}

impl FishState {
    pub fn new(id: u32, is_alive: bool, weight: f32, age: f32, hunger: f32, velocity: Vec3, position: Vec3, forces: Forces) -> Self {
        Self { id, is_alive, weight, age, hunger, velocity, position, forces }
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
pub struct Forces {
    pub player: Vec3,
    pub currents: Vec3,
    pub drag: Vec3
}

#[derive(Component)]
pub struct FishHooked;

#[derive(Default)]
pub struct Pond {
    pub fish_population: HashMap<u32, FishState>, // store specific fish by id
}

impl Pond {
    //get specific fish
    pub fn get_fish(&self, id: u32) -> Option<&FishState> {
        self.fish_population.get(&id)
    }

    //age fish consistently
    pub fn age_all_fish(&mut self) {
        for fish in self.fish_population.values_mut() {
            fish.update_fish_traits();
        }
    }
}