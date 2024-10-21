use bevy::prelude::*;
use crate::weather::*;
use crate::resources::*;



//Species struct
#[derive(Component, Debug)]
pub struct Species{
    pub name: String,
    pub fish_id: i32,
    pub length: (i32, i32),
    pub width: (i32, i32),
    pub weight: (i32, i32),
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    //bounds
    pub depth: (i32, i32),
    //x, y, z
    pub position: (i32, i32),
    //length, width, depth
    pub bounds: (i32, i32),
    pub catch_prob: i32,
}

/*impl<'a> Species<'a> {
    pub const fn new
        (in_name: &'a str, 
        in_fish_id: i32,
        in_length: (i32, i32), 
        in_width: (i32, i32), 
        in_weight: (i32,i32), 
        in_tod: (usize, usize), 
        in_weather: Weather, 
        in_depth: (i32, i32),
        in_position: (i32, i32),
        in_bounds: (i32, i32),
        in_catch_prob: i32) -> Self{
            Self{
                name: in_name,
                fish_id: in_fish_id,
                length: in_length,
                width: in_width,
                weight: in_weight,
                time_of_day: in_tod,
                weather: in_weather,
                depth: in_depth,
                position: in_position,
                bounds: in_bounds,
                catch_prob: in_catch_prob,
            }
    }
}

impl<'a>Species<'a> {
    pub fn fish_id(&self) -> i32{
        self.fish_id
    }
}
*/
//SpeciesTable
//#[derive(Resource)]
/*pub struct SpeciesTable<'a>{
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
/* 
pub const BASS: Species = Species::new(
            "Bass", 
            1,
            (10,15), 
            (5,7), 
            (20,40), 
            (7,12), 
            Weather::Sunny, 
            (0,20),
            (FISHINGROOMX as i32 + 90, FISHINGROOMY as i32 + 50),
            (10,10),
            10,

        );

//Catfish
pub const CATFISH: Species = Species::new(
            "Catfish", 
            2,
            (15,25), 
            (10,12), 
            (50, 70), 
            (18,24), 
            Weather::Rainy, 
            (20,40),
            (FISHINGROOMX as i32, FISHINGROOMY as i32 + 120),
            (5, 4),
            2,
        );
*/