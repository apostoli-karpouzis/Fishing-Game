use std::collections::HashSet;

#[derive(Resource)] //idk if this is a resource
pub struct Fish {
    pub species: String,                //type of fish
    pub length: f32,                    //fish length
    pub width: f32,                     //fish width
    pub weight: f32,                    //fish weight
    pub age: f32,                       //age (will increase constantly, and more when hooked/caught)
    pub hunger: f32,                    //hunger level
    pub time_of_day: HashSet<u32>,      //all the hours they prefer (o1)
    pub weather: HashSet<String>,       //all the weathers they prefer (o1)
    pub depth: (f32, f32)               //preferred depth range
}