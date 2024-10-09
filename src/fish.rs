use std::collections::HashSet;

#[derive(Default)] 
pub struct Fish {
    pub id: u32,                        //fish id 
    pub species: String,                //type of fish
    pub length: f32,                    //fish length
    pub width: f32,                     //fish aerodynamix
    pub weight: f32,                    //fish weight
    pub age: f32,                       //age (will increase constantly, and more when hooked/caught)
    pub hunger: f32,                    //hunger level
    pub time_of_day: HashSet<u32>,      //all the hours they prefer (o1) 0-23
    pub weather: HashSet<String>,       //all the weathers they prefer (o1)
    pub depth: (f32, f32),              //preferred depth range
    pub is_alive: bool                  //to track deaths
}

//do we want to pregenerate dif fish or start all at same or what


impl Fish {
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
    pub fn fish_anger(&mut self) {
        self.age * self.hunger;
    }
    pub fn fish_weight(&mut self) {
        self.age * self.hunger;
    }

    pub fn fish_shape(&mut self) {
        //do some calculation based on length width maybe
        //prob a task for physics 
        self.length;
        self.width;
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

