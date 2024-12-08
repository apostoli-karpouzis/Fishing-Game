use bevy::prelude::*;
use crate::weather::*;
use crate::fishing_view::*;


#[derive(Clone, Copy, Debug)]
pub enum Behavior {
    Aggressive, // moves in circles or just random idk yet  (higher multiplier for anger)
    Evasive,    // moves away from the rod x y and z        (medium multiplier for anger)
    Passive,    // moves slowly / minimal                   (lower multiplier for anger)
    Elusive     // for the rare fish
}


//Species struct
#[derive(Component)]
pub struct Species{
    pub name: &'static str,
    pub hook_pos: Vec2,
    pub length: (f32, f32),
    pub width: (f32, f32),
    pub weight: (f32, f32),
    pub cd: (f32, f32),
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    //bounds
    pub depth: (i32, i32),
    //x, y, z
    pub position: (i32, i32),
    //length, width, depth
    pub bounds: (i32, i32),
    pub catch_prob: f32,
    pub obj_pref: (ObstType, i32),
    pub behavior: Behavior,
    pub lure_pref: Lure
}

impl Species {
    pub const fn new
        (in_name: &'static str, 
        in_hook_pos: Vec2,
        in_length: (f32, f32), 
        in_width: (f32, f32), 
        in_weight: (f32,f32), 
        in_cd: (f32, f32),
        in_tod: (usize, usize), 
        in_weather: Weather, 
        in_depth: (i32, i32),
        in_position: (i32, i32),
        in_bounds: (i32, i32),
        in_catch_prob: f32,
        in_obj_pref: (ObstType, i32),
        in_behavior: Behavior,
        in_lure: Lure) -> Self{
            

            Self{
                name: in_name,
                hook_pos: in_hook_pos,
                length: in_length,
                width: in_width,
                weight: in_weight,
                cd: in_cd,
                time_of_day: in_tod,
                weather: in_weather,
                depth: in_depth,
                position: in_position,
                bounds: in_bounds,
                catch_prob: in_catch_prob,
                obj_pref: in_obj_pref,
                behavior: in_behavior,
                lure_pref: in_lure
            }
    }
}
//SpeciesTable
#[derive(Resource)]
pub struct SpeciesTable {
    sp_table: Vec<Species>,
}

impl SpeciesTable {
    pub fn new() -> Self{
        let table: Vec<Species> = vec![BASS, CATFISH];
        Self{
            sp_table: table,
        }
    }
}



//#[derive(Hash, Component, Eq, PartialEq, Debug)]
//pub struct FishHash(HashMap<String, Species>);


//This but as a hash set
/*pub struct SpeciesSet<'a>{
    sp_table: Vec<Species<'a>>,
}

impl<'a> SpeciesTable<'a> {
    pub fn new() -> Self{
        let table: Vec<Species> = vec![BASS, CATFISH];
        Self{
            sp_table: table,
        }
    }
}
*/

//Fish library starts here

//Bass
pub const BASS: Species = Species::new(
    "Bass", 
    Vec2::new(-36., 0.),
    (10.,15.), 
    (5.,7.), 
    (20.,40.), 
    (0.06, 0.94),
    (0,22),
    Weather::Sunny, 
    (0,20),
    (FISHING_ROOM_X as i32 + 90, FISHING_ROOM_Y as i32 + 50),
    (10,10),
    0.5,
    (ObstType::Pad, 2),
    Behavior::Evasive,
    Lure::BOBBER,
);

//Catfish
pub const CATFISH: Species = Species::new(
    "Catfish", 
    Vec2::new(-36., 0.),
    (15.,25.), 
    (10.,12.), 
    (50., 70.), 
    (0.05, 0.89), 
    (0,18),
    Weather::Rainy, 
    (20,40),
    (FISHING_ROOM_X as i32, FISHING_ROOM_Y as i32 + 120),
    (5, 4),
    0.4,
    (ObstType::Fissure, 1),
    Behavior::Aggressive,
    Lure::FROG,
);

//Tuna
pub const TUNA: Species = Species::new(
    "Tuna",
    Vec2::new(-36., 0.),
    (30., 50.),
    (90., 130.),
    (90., 230.),
    (0.37, 0.95),
    (1, 7),
    Weather::Thunderstorm,
    (5, 20),
    (FISHING_ROOM_X as i32, FISHING_ROOM_Y as i32 + 120),
    (5,4),
    0.5,
    (ObstType::Pad, 2),
    Behavior::Passive,
    Lure::BOBBER,
);

//Mahi-mahi
pub const MAHIMAHI: Species = Species::new(
    "Mahi-mahi",
    Vec2::new(-36., 0.),
    (50., 80.),
    (20., 30.),
    (80., 130.),
    (0.27, 0.95),
    (9, 18),
    Weather::Thunderstorm,
    (25, 200),
    (FISHING_ROOM_X as i32, FISHING_ROOM_Y as i32 + 120),
    (5,4),
    0.4,
    (ObstType::Fissure, 1),
    Behavior::Aggressive,
    Lure::FISH,
);

//Swordfish
pub const SWORDFISH: Species = Species::new(
    "Swordfish",
    Vec2::new(-36., 0.),
    (130., 150.),
    (50., 70.),
    (60., 100.),
    (0.17, 0.95),
    (18, 24),
    //is sunny just clear at night?
    Weather::Sunny,
    (100, 200),
    (FISHING_ROOM_X as i32, FISHING_ROOM_Y as i32 + 120),
    (5,4),
    0.4,
    (ObstType::Fissure, 3),
    Behavior::Evasive,
    Lure::FISH,
);

//Red Handfsih
// our "rare" fish, can only be caught on sea floor during its 1 hour feed time
pub const REDHANDFISH: Species = Species::new(
    "Red Handfish",
    Vec2::new(-36., 0.),
    (13., 22.),
    (5., 7.),
    (60., 100.),
    (0.06, 0.12),
    (20, 21),
    //is sunny just clear at night?
    Weather::Sunny,
    (198, 200),
    (FISHING_ROOM_X as i32, FISHING_ROOM_Y as i32 + 120),
    (5,4),
    0.1,
    (ObstType::Fissure, 0),
    Behavior::Elusive,
    Lure::FROG,
);