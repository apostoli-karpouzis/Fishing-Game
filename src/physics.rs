use bevy::prelude::*;
use crate::resources::*;
use crate::fish::*;

const REEL: KeyCode = KeyCode::KeyO;

const MAX_FISH_SPEED: f32 = 250.;

pub fn simulate_fish(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut fish_info: Query<(&FishSpecies, &mut FishState, &mut Transform), With<FishHooked>>
) {
    let (fish_traits, mut fish_state, mut fish_transform) = fish_info.single_mut();

    let weight: f32 = fish_state.weight;
    let anger: f32 = fish_state.fish_anger();
    // let width: f32 = fish_traits.width;
    // let cd: f32 = fish_traits.cd;
    let width: f32 = 2.0;
    let cd: f32 = 0.04;

    let fish_position: Vec3 = fish_state.position; 
    let fish_velocity: Vec3 = fish_state.velocity; 

    let player_position = Vec3::new(FISHINGROOMX - 100., FISHINGROOMY - WIN_H / 2., 901.);
    
    // Calculate drag
    let p = -fish_position.z;
    let sa = width * width;
    let drag_force = p * cd * sa * fish_velocity * fish_velocity; //Force exerted onto the fish by the water
    
    // Calculate player force
    let reeling = input.pressed(REEL);

    let player_force = if reeling {
        let delta = player_position - fish_position; //calculate force TWORDS the player

        100. * delta.normalize_or_zero()
    } else {
        Vec3::ZERO
    };
    
    // Calculate fish force
    let fish_force: Vec3 = -anger * fish_velocity.normalize_or_zero(); //opposed velocity
    
    // Calculate net force and acceleration 
    let net_force = drag_force + player_force + fish_force; // fish force works against player drag force works against motion of fish

    let acceleration = net_force / weight;
    //fish_velocity += acceleration * time.delta_seconds();

    fish_state.velocity = (fish_state.velocity + acceleration * time.delta_seconds()).clamp_length_max(MAX_FISH_SPEED);

    // Bounds check
    let mut offset = fish_velocity * time.delta_seconds();
    offset.z = 0.;
    fish_state.position += offset;

    fish_transform.translation.x = fish_state.position.x;
    fish_transform.translation.y = fish_state.position.y;
    
    let dist = (fish_state.position - player_position).length();

    if dist < 5.0
    {
        println!("caught fish!");
    }
}
