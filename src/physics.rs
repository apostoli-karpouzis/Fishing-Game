use bevy::prelude::*;
use crate::fish::*;
use crate::species::*;
use crate::fishing_view::*;
use crate::player::*;
use crate::map::*;
use std::f32;
use f32::consts::PI;
use std::collections::HashSet;

const REEL: KeyCode = KeyCode::KeyO;

pub const PIXELS_PER_METER: f32 = 150.;
pub const BENDING_RESOLUTION: f32 = 1. / PIXELS_PER_METER;

pub const GRAVITY: f32 = 30.;

const MAX_PLAYER_FORCE: f32 = 600.;
const MAX_PLAYER_POWER: f32 = MAX_PLAYER_FORCE * 60.;
const P: f32 = 1. / 250.;

#[derive(Component)]
pub struct PhysicsObject {
    pub mass: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    pub forces: Forces,
    pub cd: (f32, f32),
    pub sa: (f32, f32),
    pub waves: Entity
}

impl PhysicsObject {
    pub fn new(mass: f32, position: Vec3, rotation: Vec3, velocity: Vec3, forces: Forces, cd: (f32, f32), sa: (f32, f32), waves: Entity) -> Self {
        Self { mass, position, rotation, velocity, forces, cd, sa, waves }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Forces {
    pub own: Vec3,
    pub player: Vec3,
    pub water: Vec3,
    pub gravity: Vec3,
    pub buoyancy: Vec3
}

impl Forces {
    pub fn net_force(&self) -> Vec3 {
        return self.own + self.player + self.water + self.gravity + self.buoyancy;
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

pub fn get_particle_positions(
    mut particles: Query<&mut ParticleList, With<ParticleList>>
){
    let mut particle_info = particles.single_mut();
    let particle_list = &mut particle_info.particle_list;
    let mut particle_hash: HashSet<&Particle> = HashSet::new();
    for  particle in particle_list.iter_mut(){
        //do movement calculations here
        particle.position = particle.position + particle.velocity;
        if !particle_hash.insert(particle) {
            let mut collision_particle = particle_hash.get(particle).unwrap(); //get particle we are colliding with
            let particle_velocity = ((2.* collision_particle.mass * collision_particle.velocity) + (particle.velocity* (particle.mass + collision_particle.mass)))/(particle.mass + collision_particle.mass); 
            let collision_particle_velocity = ((particle.mass * particle.velocity) + (collision_particle.mass * collision_particle.velocity) - (particle.mass * particle.velocity))/(collision_particle.mass);
        }

    }


}

pub fn is_line_broken (
    mut commands: Commands,
    mut next_state: ResMut<NextState<FishingState>>,
    mut hooked_object: Query<(Entity, &mut PhysicsObject), With<Hooked>>,
    line: Query<&FishingLine, With<FishingLine>>
){
    if hooked_object.is_empty() {
        return;
    }

    let (entity_id, mut physics_object) = hooked_object.single_mut();
    let line_info = line.single();
    
    let line_dir = (line_info.end - line_info.start).normalize();
    let tension = physics_object.forces.player.dot(line_dir) + physics_object.forces.water.dot(line_dir) + physics_object.forces.own.dot(line_dir);

    if tension > line_info.line_type.ultimate_tensile_strength {
        commands.entity(entity_id).remove::<Hooked>();
        physics_object.forces.player = Vec3::ZERO;
        next_state.set(FishingState::Idle);
    }
}

pub fn calculate_buoyancy_force (
    mut lure: Query<(&Lure, &mut PhysicsObject), With<Lure>>
) {
    let (lure_info, mut lure_physics) = lure.single_mut();

    let buoyancy = if lure_physics.position.z > 0. {
        0.
    } else {
        (lure_physics.position.z / lure_info.depth).powi(2) * GRAVITY * lure_physics.mass
    };

    lure_physics.forces.buoyancy = Vec3::new(0., 0., buoyancy);
}

pub fn calculate_water_force (
    map: Res<Map>,
    mut physics_objects: Query<&mut PhysicsObject>,
    player: Query<&Location, With<Player>>
) {
    let player_location = player.single();
    let water_current = map.areas[player_location.x][player_location.y].zone.current;

    for mut physics_object in physics_objects.iter_mut() {
        let relative_velocity = physics_object.velocity - water_current;

        if physics_object.position.z > 0. || relative_velocity == Vec3::ZERO {
            physics_object.forces.water = Vec3::ZERO;
            continue;
        }

        let angle = Vec2::from_angle(physics_object.rotation.z).extend(0.).angle_between(water_current);
        let proportion = (PI / 2. - f32::abs(angle - PI / 2.)) / (PI / 2.);
        let sa = physics_object.sa.0 + (physics_object.sa.1 - physics_object.sa.0) * proportion;
        let cd = physics_object.cd.0 + (physics_object.cd.1 - physics_object.cd.0) * proportion;
    
        physics_object.forces.water = P * cd * sa * relative_velocity.length() * relative_velocity.length() * -relative_velocity.normalize();
    }
}

pub fn calculate_player_force (
    input: Res<ButtonInput<KeyCode>>,
    fishing_rod: Query<&FishingRod, With<FishingRod>>,
    mut hooked_object: Query<&mut PhysicsObject, With<Hooked>>,
) {
    if hooked_object.is_empty() {
        return;
    }

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