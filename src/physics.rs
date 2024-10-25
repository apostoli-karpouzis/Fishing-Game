use bevy::prelude::*;
use crate::fish::*;
use crate::species::*;
use crate::fishingView::*;
use crate::player::*;
use crate::map::*;
use std::f32;
use f32::consts::PI;

const REEL: KeyCode = KeyCode::KeyO;

const PLAYER_FORCE: f32 = 600.;

#[derive(Component)]
pub struct PhysicsObject {
    pub mass: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    pub forces: Forces
}

#[derive(Default)]
pub struct Forces {
    pub own: Vec3,
    pub player: Vec3,
    pub water: Vec3
}

impl Forces {
    pub fn net_force(&self) -> Vec3 {
        return self.own + self.player + self.water;
    }
}

#[derive(Component)]
pub struct Hooked;

pub fn calculate_water_force (
    mut fishes: Query<(&Species, &Fish, &mut PhysicsObject), With<Fish>>,
    player: Query<&Location, With<Player>>
) {
    let player_location = player.single();
    let water_current = player_location.get_current_area().zone.current;

    // Currently only works with fish
    for (fish_species, fish_details, mut fish_physics) in fishes.iter_mut() {
        let p = 1.0;
        let sa = fish_details.width * fish_details.width / 2000.;
        let relative_velocity = fish_physics.velocity - water_current;
    
        fish_physics.forces.water = p * fish_species.cd * sa * relative_velocity.length() * relative_velocity.length() * -relative_velocity.normalize_or_zero();
    }
}

pub fn calculate_player_force (
    input: Res<ButtonInput<KeyCode>>,
    fishing_view: ResMut<FishingView>,
    fishing_rod: Query<(&Transform, &FishingRod), With<FishingRod>>,
    mut hooked_object: Query<&mut PhysicsObject, With<Hooked>>
) {
    let (rod_transform, rod_info) = fishing_rod.single();
    let mut object_physics = hooked_object.single_mut();

    let reeling = input.pressed(REEL);

    object_physics.forces.player = if reeling {
        let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
        let rod_end = rod_transform.translation.with_z(0.) + (rod_info.length / 4. * angle_vector).extend(0.);
        let delta = rod_end - object_physics.position;

        PLAYER_FORCE * delta.normalize_or_zero()
    } else {
        Vec3::ZERO
    };
}

pub fn calculate_fish_force (
    mut fishes: Query<(&mut Fish, &mut PhysicsObject), With<Fish>>,
) {
    for fish in fishes.iter_mut() {
        let (mut fish_details, mut fish_physics) = fish;
        
        fish_physics.forces.own = if fish_physics.velocity.length() > 5.0 {
            -fish_details.fish_anger() * fish_physics.velocity.normalize_or_zero()
        }  else {
            Vec3::ZERO
        }
    }
}

pub fn simulate_physics (
    time: Res<Time>,
    mut objects: Query<&mut PhysicsObject, With<PhysicsObject>>
) {
    for mut fish_physics in objects.iter_mut() {
        // Calculate net force and acceleration 
        let acceleration = fish_physics.forces.net_force() / fish_physics.mass;
        fish_physics.velocity = fish_physics.velocity + acceleration * time.delta_seconds();

        // Bounds check
        let mut new_pos = fish_physics.position + fish_physics.velocity * time.delta_seconds();
        
        // Surface collision
        if new_pos.z > 0. {
            new_pos.z = 0.;
            fish_physics.velocity.z = 0.;
        }

        fish_physics.position = new_pos;

        // Calculate rotation
        if fish_physics.velocity.x != 0. || fish_physics.velocity.y != 0. {
            fish_physics.rotation.z = f32::atan2(fish_physics.velocity.y, fish_physics.velocity.x) + PI;
        }
    }
}