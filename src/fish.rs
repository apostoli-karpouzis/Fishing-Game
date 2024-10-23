use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Fish {
    pub id: u32,
    pub is_caught: bool,
    pub is_alive: bool,
    pub length: f32,
    pub width: f32,
    pub weight: f32,
    pub age: f32,
    pub hunger: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    pub forces: Forces
}

impl Fish {
    pub fn new(id: u32, is_caught: bool, is_alive: bool, length: f32, width: f32, weight: f32, age: f32, hunger: f32, position: Vec3, rotation: Vec3, velocity: Vec3, forces: Forces) -> Self {
        Self { id, is_caught, is_alive, length, width, weight, age, hunger, position, rotation, velocity, forces }
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

