use bevy::prelude::*;
use crate::fish::*;
use crate::species::*;
use crate::fishing_view::*;
use crate::player::*;
use crate::map::*;
use std::f32;
use f32::consts::PI;

const REEL: KeyCode = KeyCode::KeyO;

pub const PIXELS_PER_METER: f32 = 150.;
pub const BENDING_RESOLUTION: f32 = 1. / PIXELS_PER_METER;

const MAX_PLAYER_FORCE: f32 = 600.;
const MAX_PLAYER_POWER: f32 = MAX_PLAYER_FORCE * 60.;
const P: f32 = 1. / 250.;

#[derive(Component)]
pub struct PhysicsObject {
    pub mass: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    pub forces: Forces
}

impl PhysicsObject {
    pub fn new(mass: f32, position: Vec3, rotation: Vec3, velocity: Vec3, forces: Forces) -> Self {
        Self { mass, position, rotation, velocity, forces }
    }
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

pub fn bend_fishing_rod (
    mut commands: Commands,
    mut fishing_rod: Query<(&mut FishingRod, &Transform), With<FishingRod>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    hooked_object: Query<(&Species, &PhysicsObject), With<Hooked>>
) {
    let (mut rod_info, rod_transform) = fishing_rod.single_mut();
    let mut line_info = line.single_mut();
    
    let traverse_force = if hooked_object.is_empty() {
        // Rod bending currently only supported for fish
        0.
    } else {
        let (fish_species, physics_object) = hooked_object.single();

        // Temporary 2D calculation
        let rod_dir = Vec2::from_angle(rod_info.rotation);
        let rod_end = rod_transform.translation.xy() + rod_info.rod_type.length / 2. * rod_dir;
        let fish_offset = fish_species.hook_pos.rotate(Vec2::from_angle(physics_object.rotation.z));
        let fish_pos = physics_object.position.xy() + fish_offset;
        let line_dir = fish_pos - rod_end;
        let angle = Vec2::angle_between(rod_dir, line_dir);

        (physics_object.forces.water.length() + physics_object.forces.own.length()) * f32::sin(angle)
    };

    let rod_type = rod_info.rod_type;
    let thickness_ratio = rod_type.thickness / rod_type.radius;
    let thickness_ratio_inverse = 1. - thickness_ratio;
    let base_rotation = Vec2::from_angle(rod_info.rotation);

    let mut position = Vec2::ZERO;
    let mut theta = 0.;

    for i in 0..rod_info.segments.len() {
        // Calculate position of each segment
        let l = i as f32 * BENDING_RESOLUTION;
        let bending_moment_area = 0.5 * (l + l + BENDING_RESOLUTION) * traverse_force * BENDING_RESOLUTION;
        let r2 = rod_type.radius * (thickness_ratio + l / rod_type.length * thickness_ratio_inverse);
        let r1 = r2 - rod_type.thickness;
        let second_moment_area = PI / 4. * (r2 * r2 * r2 * r2 - r1 * r1 * r1 * r1);
        let dt = bending_moment_area / (rod_type.shear_modulus * second_moment_area);
  
        theta += dt;
        position += BENDING_RESOLUTION * Vec2::from_angle(theta);

        // Check if fishing rod will break
        let area = PI * (r2 * r2 - r1 * r1);
        let stress = traverse_force * l / area;

        if stress > rod_info.rod_type.flexural_strength {
            // BREAK
        }

        // Display
        let screen_position = PLAYER_POSITION.xy() + position.rotate(base_rotation) * PIXELS_PER_METER;

        let mut entity = commands.entity(rod_info.segments[i]);
        entity.insert(Transform::from_xyz(screen_position.x, screen_position.y, 901.));
    }
    
    rod_info.tip_pos = (rod_transform.translation.xy() + position.rotate(base_rotation) * PIXELS_PER_METER).extend(0.);
    line_info.start = rod_info.tip_pos;
}

pub fn is_line_broken (
    mut commands: Commands,
    mut next_state: ResMut<NextState<FishingState>>,
    hooked_object: Query<(Entity, &PhysicsObject), With<Hooked>>,
    line: Query<&FishingLine, With<FishingLine>>,
){
    let (entity_id, fish_physics) = hooked_object.single();
    let fishingline = line.single(); 
    let tension_force = fish_physics.forces.player.length() + fish_physics.forces.water.length() + fish_physics.forces.own.length();

    if tension_force > fishingline.line_type.ultimate_tensile_strength {
        commands.entity(entity_id).remove::<Hooked>();
        next_state.set(FishingState::Idle);
    }
}

pub fn calculate_water_force (
    mut fishes: Query<(&Species, &Fish, &mut PhysicsObject), With<Fish>>,
    player: Query<&Location, With<Player>>
) {
    let player_location = player.single();
    let water_current = player_location.get_current_area().zone.current;

    // Currently only works with fish
    for (fish_species, fish_details, mut fish_physics) in fishes.iter_mut() {
        let relative_velocity = fish_physics.velocity - water_current;

        if relative_velocity == Vec3::ZERO {
            fish_physics.forces.water = Vec3::ZERO;
            continue;
        }

        let angle = Vec2::from_angle(fish_physics.rotation.z).extend(0.).angle_between(water_current);
        let proportion = (PI / 2. - f32::abs(angle - PI / 2.)) / (PI / 2.);
        let sa_min = fish_details.width * fish_details.width;
        let sa_max = fish_details.width * fish_details.length;
        let sa = sa_min + (sa_max - sa_min) * proportion;
        let cd = fish_species.cd.0 + (fish_species.cd.1 - fish_species.cd.0) * proportion;
    
        fish_physics.forces.water = P * cd * sa * relative_velocity.length() * relative_velocity.length() * -relative_velocity.normalize();
    }
}

pub fn calculate_player_force (
    input: Res<ButtonInput<KeyCode>>,
    fishing_rod: Query<&FishingRod, With<FishingRod>>,
    mut hooked_object: Query<&mut PhysicsObject, With<Hooked>>,
) {
    let rod_info = fishing_rod.single();
    let mut object_physics = hooked_object.single_mut();

    let reeling = input.pressed(REEL);

    object_physics.forces.player = if reeling {
        let delta = rod_info.tip_pos - object_physics.position;
        let force = (MAX_PLAYER_POWER / object_physics.velocity.length()).min(MAX_PLAYER_FORCE);

        force * delta.normalize_or_zero()
    } else {
        Vec3::ZERO
    };
}

pub fn calculate_fish_force (
    mut fishes: Query<(&mut Fish, &mut PhysicsObject), With<Fish>>,
) {
    for fish in fishes.iter_mut() {
        let (mut fish_details, mut fish_physics) = fish;
        
        fish_physics.forces.own = (fish_physics.forces.player + fish_physics.forces.water) * -0.1;
    }
}

pub fn simulate_physics (
    time: Res<Time>,
    mut objects: Query<&mut PhysicsObject, With<PhysicsObject>>
) {
    for mut object in objects.iter_mut() {
        // Calculate net force and acceleration
        let acceleration = object.forces.net_force() / object.mass;
        object.velocity = object.velocity + acceleration * time.delta_seconds();

        // Bounds check
        let mut new_pos = object.position + object.velocity * time.delta_seconds();
        
        // Surface collision
        if new_pos.z > 0. {
            new_pos.z = 0.;
            object.velocity.z = 0.;
        }

        object.position = new_pos;

        // Calculate rotation
        if object.velocity.x != 0. || object.velocity.y != 0. {
            object.rotation.z = f32::atan2(object.velocity.y, object.velocity.x) + PI;
        }
    }
}