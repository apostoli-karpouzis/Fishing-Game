use bevy::{prelude::*};
use crate::{fish::*, map::Collision};

#[derive(Component, Default)]
pub struct FishState {
    pub velocity: Vec3,
    pub position: Vec3,
}

impl FishState {
    pub fn new(x: f32, y: f32, z: f32 ) -> Self {
        Self {
            velocity: Vec3::splat(0.),
            position: Vec3::new(x,y,z),
        }
    }
}

#[derive(Component)]
pub struct FishHooked;

const FISH_SPEED: f32 = 250.;

pub fn simulate_fish(
    time: Res<Time>,
    mut fish_info: Query<(&mut Fish, &mut FishState, &mut Transform), With<FishHooked>>,
) {

    let (mut fish_traits,mut f_state, mut f_pos) = fish_info.single_mut();
    //let weight: f32 = fish_traits.weight;
    //let anger: f32 = fish_traits.fish_anger();
    //let width: f32 = fish_traits.width;
    let weight: f32 = 2.0;
    let anger: f32 = 2.0;
    let width: f32 = 10.0;
    let cd: f32 = 0.04;
    let fish_position: Vec3 = f_state.position; 
    let fish_velocity: Vec3 = f_state.velocity; 

    let player_position = Vec3::new(0., 0., 0.);
    let reeling = true;
    
    // Calculate drag
    let p = -fish_position.z;
    let sa = width * width;
    let drag_force = p * cd * sa * fish_velocity * fish_velocity; //Force exerted onto the fish by the water

    // Calculate player force
    let player_force = if reeling {
        let delta = player_position - fish_position; //calculate force TWORDS the player

        30. * delta.normalize_or_zero()
    } else {
        Vec3::ZERO
    };
    
    // Calculate fish force
    let fish_force: Vec3 = -anger * fish_velocity.normalize_or_zero(); //opposed velocity
    println!("anger: {}", anger);
    println!("fish_velocity normal: {}", fish_velocity.normalize_or_zero()); 
    println!("fish_velocity: {}", fish_velocity);
    
    // Calculate net force and acceleration 
    let net_force = drag_force + player_force + fish_force; // fish force works against player drag force works against motion of fish

    let acceleration = net_force / weight;
    //fish_velocity += acceleration * time.delta_seconds();

    f_state.velocity = (f_state.velocity + acceleration * time.delta_seconds()).clamp_length_max(FISH_SPEED);
    println!("velocity: {}", f_state.velocity);
    // Bounds check
    let mut offset = fish_velocity * time.delta_seconds();
    offset.z = 0.;
    f_state.position += offset;

    f_pos.translation.x = f_state.position.x;
    f_pos.translation.y = f_state.position.y;

    
    let dist = (f_state.position - player_position).length();

    if dist < 5.0
    {
        println!("caught fish!");
    }
}
