use bevy::prelude::*;
use crate::fish::*;
use crate::species::*;
use crate::fishingView::*;
use crate::player::*;
use crate::map::*;
use std::f32;
use f32::consts::PI;
use crate::money::*;


const REEL: KeyCode = KeyCode::KeyO;

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

pub fn simulate_fish (
    input: Res<ButtonInput<KeyCode>>,
    mut fishes: Query<(&Species, &mut Fish, &mut PhysicsObject), With<Fish>>,
    fishing_rod: Query<(&Transform, &FishingRod, &RotationObj), (With<FishingRod>, Without<Fish>)>,
    line_info: Query<&FishingLine, With<FishingLine>>,
    player: Query<&Location, With<Player>>
) {
    let (rod_transform, rod_info, rod_rotation) = fishing_rod.single();
    let line = line_info.single();
    let player_location = player.single();

    for fish in fishes.iter_mut() {
        let (fish_species, mut fish_details, mut fish_physics) = fish;
        
        // Calculate force from water
        let p = 1.0;
        let sa = fish_details.width * fish_details.width / 2000.;
        let relative_velocity = fish_physics.velocity - player_location.get_current_area().zone.current;

        fish_physics.forces.water = p * fish_species.cd * sa * relative_velocity.length() * relative_velocity.length() * -relative_velocity.normalize_or_zero(); //Force exerted onto the fish by the water
        
        // Calculate player force
        let reeling = input.pressed(REEL);

        fish_physics.forces.player = if reeling && line.fish_on {
            let angle_vector = Vec2::from_angle(rod_rotation.rot + PI / 2.);
            let rod_end = rod_transform.translation.with_z(0.) + (rod_info.length / 4. * angle_vector).extend(0.);
            let delta = rod_end - fish_physics.position;

            800. * delta.normalize_or_zero()
        } else {
            Vec3::ZERO
        };

        // Calculate fish force
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