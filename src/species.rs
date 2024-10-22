use bevy::prelude::*;
use crate::weather::*;

//Species struct
#[derive(Component)]
pub struct Species{
    pub name: &'static str,
    pub length: (f32, f32),
    pub width: (f32, f32),
    pub weight: (f32, f32),
    pub cd: f32,
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    pub depth: (f32, f32),
}

impl Species {
    pub const fn new
        (in_name: &'static str, 
        in_length: (f32, f32), 
        in_width: (f32, f32), 
        in_weight: (f32,f32), 
        in_cd: f32,
        in_tod: (usize, usize), 
        in_weather: Weather, 
        in_depth: (f32, f32)) -> Self{
            Self{
                name: in_name,
                length: in_length,
                width: in_width,
                weight: in_weight,
                cd: in_cd,
                time_of_day: in_tod,
                weather: in_weather,
                depth: in_depth,
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


//Fish library starts here

//Bass
pub const BASS: Species = Species::new(
            "Bass", 
            (10.,15.), 
            (5.,7.), 
            (20.,40.), 
            0.03,
            (7,12), 
            Weather::Sunny, 
            (0.,20.)
        );

//Catfish
pub const CATFISH: Species = Species::new(
            "Catfish", 
            (15.,25.), 
            (10.,12.), 
            (50., 70.), 
            0.04,
            (18,24), 
            Weather::Rainy, 
            (20.,40.)
        );
 