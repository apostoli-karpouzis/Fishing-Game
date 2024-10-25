use bevy::{prelude::*, sprite::Mesh2dHandle};
use std::f32;
use std::f32::consts::PI;
use rand::Rng;
use crate::fish::*;
use crate::map::Collision;
use crate::resources::*;
use crate::weather::*;
extern crate rand;
use crate::map::*;
use crate::physics::*;
use crate::species::*;

const TUG: KeyCode = KeyCode::KeyP;
const REEL: KeyCode = KeyCode::KeyO;

const POWER_BAR_Y_OFFSET: f32 = FISHINGROOMY - 308.;
const MAX_POWER: f32 = 250.;
const POWER_FILL_SPEED: f32 = 250.;

const ROD_MIN_ROTATION: f32 = -1.2;
const ROD_MAX_ROTATION: f32 = 1.2;
const ROD_ROTATION_SPEED: f32 = PI / 2.;

const MAX_CAST_DISTANCE: f32 = 400.;
const CASTING_SPEED: f32 = 250.;
const REEL_IN_SPEED: f32 = 150.;

#[derive(Resource)]
pub struct FishingView {
    pub rod_rotation: f32,
    pub power: f32
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FishingState {
    #[default]
    Idle,
    Casting,
    Reeling
}

#[derive(Component)]
pub struct PowerBar;

#[derive(Component)]
pub struct FishingRod {
    pub length: f32
}

#[derive(Resource)]
pub struct DirectionTimer {
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct FishingLine {
    pub fish_on: bool,
    pub cast_distance: f32,
    pub length: f32,
    pub start: Vec2,
    pub end: Vec2,
    pub mesh_handle: Handle<Mesh>,
}

impl FishingLine {
    pub const WIDTH: f32 = 3.;
}

#[derive(Component, Default)]
pub struct Bobber;

#[derive(Component)]
pub struct Wave;

#[derive(Component, Default)]
pub struct Splash {
    pub position: Vec3,
}

#[derive(Component)]
pub struct InPond;

#[derive(Component)]
pub struct IsBass;
//FISH THING
#[derive(Component)]
pub struct FishDetails {
    pub name: &'static str,
    pub fish_id: i32,
    pub length: i32,
    pub width: i32,
    pub weight: i32,
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    //bounds
    pub depth: (i32, i32),
    //x, y, z
    pub position: (i32, i32),
    pub change_x: Vec3,
    pub change_y: Vec3,
    //length, width, depth
    pub bounds: (i32, i32),
    pub hunger: f32,
    pub touching_lure: bool,
}

impl FishDetails {
    pub fn new(
        name: &'static str,
        fish_id: i32,
        length: i32,
        width: i32,
        weight: i32,
        time_of_day: (usize, usize),
        weather: Weather,
        depth: (i32, i32),
        position: (i32, i32),
        change_x: Vec3,
        change_y: Vec3,
        bounds: (i32, i32),
        hunger: f32,
        touching_lure: bool,
    ) -> Self {
        Self {
            name,
            fish_id,
            length,
            width,
            weight,
            time_of_day,
            weather,
            depth,
            position,
            change_x,
            change_y,
            bounds,
            hunger,
            touching_lure,
        }
    }
}

pub struct FishingViewPlugin;

impl Plugin for FishingViewPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<FishingMode>()
        .init_state::<FishingState>()
        .insert_resource(FishingView { rod_rotation: 0., power: 0. })
        .add_systems(Update,
            (
                power_bar_cast.run_if(in_state(FishingState::Idle)),
                rod_rotate,
                (
                    update_fishing_interface,
                    calculate_water_force,
                    calculate_player_force
                ).after(power_bar_cast).after(rod_rotate),
                calculate_fish_force.after(calculate_water_force).after(calculate_player_force),
                simulate_physics.after(calculate_fish_force),
                cast_line.run_if(in_state(FishingState::Casting)),
                (
                    is_fish_hooked,
                    is_fish_caught.after(is_fish_hooked),
                ).run_if(in_state(FishingState::Reeling)).after(simulate_physics),
                move_physics_objects.after(is_fish_caught),
                animate_fishing_line.after(move_physics_objects).run_if(in_state(FishingState::Reeling)),
                draw_fishing_line.after(animate_fishing_line),
                animate_waves.after(simulate_physics),
                animate_splash.after(cast_line)
            ).run_if(in_state(FishingMode::Fishing))
        )
        .add_systems(OnEnter(FishingMode::Fishing), fishing_transition)
        .add_systems(OnExit(FishingMode::Fishing), overworld_transition)
        .add_systems(OnEnter(FishingState::Casting), begin_cast)
        .add_systems(OnExit(FishingState::Reeling), reset_interface);
    }
}

pub fn move_fish(
    mut fish_details: Query<(&mut FishDetails, &mut Transform), (With<InPond>, With<Collision>)>,
    time: Res<Time>,
    mut config: ResMut<DirectionTimer>,
    //mut fish_direction: ResMut<FishBoundsDir>
) {
    let mut rng = rand::thread_rng();
    config.timer.tick(time.delta());
    for (mut fish_details, mut fish_pos) in fish_details.iter_mut() {
        //let mut rng = rand::thread_rng();

        if config.timer.finished() {
            println!("timer finished");
            let dir: i32 = rng.gen_range(0..9);
            println!("numer is {} {:?}", dir, fish_details.name);
            if dir == 0 {
                fish_details.change_x = Vec3::new(0., 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            } else if dir == 1 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            } else if dir == 2 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0., 0.);
            } else if dir == 3 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 4 {
                fish_details.change_x = Vec3::new(0., 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 5 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 6 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0., 0.);
            } else if dir == 7 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            }
            println!(
                "numer is {:?} is going {:?}",
                fish_details.name, fish_details.change_x
            );
        }
        //match it then set up the vector for the next tree seconds, keep the stuff about borders

        //println!("fish pos x {}, fish pos y {}", change_x, change_y);
        //CHANGE THESE TO CONSTANTS LATER!!!!!
        let holdx: Vec3 = fish_pos.translation + fish_details.change_x;
        if (holdx.x) >= (8320. + 160.) && (holdx.x) <= (9391. - 160.) {
            fish_pos.translation += fish_details.change_x;
        }
        let holdy: Vec3 = fish_pos.translation + fish_details.change_y;
        if (holdy.y) >= (3376. + 90.) && (holdy.y) <= (3960. - 90.) {
            fish_pos.translation += fish_details.change_y;
        }
    }
    //fish_pos.translation += change_y;
    //return (self.position.0 + x, self.position.1+y)
}

pub fn fish_area_bobber(
    mut fish_details: Query<(&mut FishDetails, &mut Transform), (With<InPond>, With<Collision>, Without<Bobber>)>,
    bobber: Query<(&Transform, &Tile), With<Bobber>>,
) {
    //let (bob, tile) = bobber.single_mut();
    let (bob, tile) = bobber.single();
    for (mut fishes_details, fish_pos) in fish_details.iter_mut() {
        let fish_pos_loc = fish_pos.translation;
        let bobber_position = bob.translation;

        if fish_pos_loc.y - 180. / 2. > bobber_position.y + tile.hitbox.y / 2.
            || fish_pos_loc.y + 180. / 2. < bobber_position.y - tile.hitbox.y / 2.
            || fish_pos_loc.x + 320. / 2. < bobber_position.x - tile.hitbox.x / 2.
            || fish_pos_loc.x - 320. / 2. > bobber_position.x + tile.hitbox.x / 2.
        {
            //there is no hit
            fishes_details.touching_lure = false;
            println!("no hit");
            return;
        }
        fishes_details.touching_lure = true;
        println!("numer is {:?} {:?}", fishes_details.name, fishes_details.touching_lure);
        println!("bobber hit");
    }
}

pub fn fishing_transition (
    mut return_val: ResMut<PlayerReturnPos>,
    mut fishing_view: ResMut<FishingView>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut power_bar: Query<(&mut Transform, &mut PowerBar), (With<PowerBar>, Without<Camera>)>,
    mut rod: Query<&mut Transform, (With<FishingRod>, Without<Camera>, Without<PowerBar>)>,
) {
    let mut camera_transform = camera.single_mut();
    let (mut power_bar_transform, mut power) = power_bar.single_mut();
    let mut rod_transform = rod.single_mut();

    return_val.player_save_x = camera_transform.translation.x;
    return_val.player_save_y = camera_transform.translation.y;

    camera_transform.translation.x = FISHINGROOMX;
    camera_transform.translation.y = FISHINGROOMY;
    //FISHINGROOMY-308
    //spawn in powerbar
    //commands.spawn
    // power_bar_transform.translation.y = POWER_BAR_Y_OFFSET;
    // fishing_view.power = 0.;

    //rd
    // fishing_view.rod_rotation = 0.;
    // rod_transform.rotation = Quat::from_rotation_z(fishing_view.rod_rotation);

    //new movmemnt system, rotation then space hold.
    //powerbar is space A, D are rotational
}

pub fn overworld_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    //mut power_bar: Query<(&mut Transform, &mut Power), With<Bar>>,
    return_val: ResMut<PlayerReturnPos>,
) {
    let mut ct = camera.single_mut();
    //let (mut pb, mut power) = power_bar.single_mut();
    ct.translation.x = return_val.player_save_x;
    ct.translation.y = return_val.player_save_y;

    //pb.translation.y = (POWER_BAR_Y_OFFSET);
    //power.meter = 0;
    //set powerbar back to 0
    //set rotation back to 0
}

pub fn power_bar_cast(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<FishingState>>,
    mut fishing_view: ResMut<FishingView>,
) {
    if input.pressed(TUG) {
        // Increase power
        fishing_view.power = fishing_view.power + POWER_FILL_SPEED * time.delta_seconds();

        if fishing_view.power >= MAX_POWER {
            // Max power reached, release
            fishing_view.power = MAX_POWER;
            next_state.set(FishingState::Casting);
        }
    } else if input.just_released(TUG) {
        // Manual release
        next_state.set(FishingState::Casting);
    }
}

pub fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut fishing_view: ResMut<FishingView>
) {
    let mut direction = 0.;

    if input.pressed(KeyCode::KeyA) {
        direction += 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        direction += -1.;
    }

    let new_rotation = fishing_view.rod_rotation + direction * ROD_ROTATION_SPEED * time.delta_seconds();
    fishing_view.rod_rotation = new_rotation.clamp(ROD_MIN_ROTATION, ROD_MAX_ROTATION);
}

pub fn begin_cast (
    fishing_view: Res<FishingView>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<&mut Visibility, With<Bobber>>
) {
    let mut line_info = line.single_mut();
    let mut bobber_visibililty = bobber.single_mut(); 

    line_info.cast_distance = fishing_view.power / MAX_POWER * MAX_CAST_DISTANCE;
    *bobber_visibililty = Visibility::Visible;
}

pub fn cast_line (
    time: Res<Time>,
    fishing_view: Res<FishingView>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<&mut Transform, (With<Bobber>, Without<FishingRod>)>,
    mut splash: Query<(&mut Splash, &mut Visibility), With<Splash>>
) {
    let (rod_info, rod_transform) = rod.single();
    let mut line_info = line.single_mut();
    let mut bobber_transform = bobber.single_mut();
    let (mut splash_info, mut splash_visibility) = splash.single_mut();

    let new_length = (line_info.length + CASTING_SPEED * time.delta_seconds()).min(line_info.cast_distance);
    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    
    line_info.start = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
    line_info.end = rod_transform.translation.xy() + (rod_info.length / 2. + line_info.length) * angle_vector;

    if new_length == line_info.cast_distance {
        // Cast finished
        line_info.length = line_info.cast_distance;
        
        // Splash animation
        *splash_visibility = Visibility::Visible;
        splash_info.position = rod_transform.translation + ((rod_info.length / 2. + line_info.length) * angle_vector).extend(0.);
        next_state.set(FishingState::Reeling);
    } else {
        // Casting
        line_info.length = new_length;
    }

    bobber_transform.translation = line_info.end.extend(950.);

}

pub fn update_fishing_interface (
    fishing_view: Res<FishingView>,
    mut power_bar: Query<&mut Transform, With<PowerBar>>,
    mut rod: Query<&mut Transform, (With<FishingRod>, Without<PowerBar>)>
) {
    let mut power_bar_transform = power_bar.single_mut();
    let mut rod_transform = rod.single_mut();

    power_bar_transform.translation.y = POWER_BAR_Y_OFFSET + fishing_view.power;
    rod_transform.rotation = Quat::from_rotation_z(fishing_view.rod_rotation);
}

pub fn is_fish_hooked(
    bobber: Query<(&Transform, &Tile),  With<Bobber>>,
    fish: Query<(&Fish, &PhysicsObject), With<Fish>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
) {
    let (fish_details, fish_physics) = fish.single();
    let fish_position = fish_physics.position;
    let (bobber_transform, tile) = bobber.single();
    let bobber_position = bobber_transform.translation;

    if fish_position.y - fish_details.width / 2. > bobber_position.y + tile.hitbox.y / 2.
    || fish_position.y + fish_details.width / 2. < bobber_position.y - tile.hitbox.y / 2. 
    || fish_position.x + fish_details.width / 2. < bobber_position.x - tile.hitbox.x / 2. 
    || fish_position.x - fish_details.width / 2. > bobber_position.x + tile.hitbox.x / 2.
    {
        return;
    }

    let mut line_state = line.single_mut();
    line_state.fish_on = true;
}

pub fn animate_fishing_line (
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    fishing_view: Res<FishingView>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut fish: Query<(&Species, &PhysicsObject), With<Fish>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<&mut Transform, (With<Bobber>, Without<FishingRod>)>
) {
    let (rod_info, rod_transform) = rod.single();
    let (fish_species, fish_physics) = fish.single_mut();
    let mut line_info = line.single_mut();
    let mut bobber_transform = bobber.single_mut();

    // Fish hooked
    if line_info.fish_on {
        let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
        let rod_end = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
        let fish_offset = fish_species.hook_pos.rotate(Vec2::from_angle(fish_physics.rotation.z));
        let fish_pos = fish_physics.position.xy() + fish_offset;

        // Hook line to fish
        line_info.start = rod_end;
        line_info.end = fish_pos;

        bobber_transform.translation =  fish_pos.extend(950.);
    } else {
        // Reeling
        if input.pressed(REEL) {
            line_info.length = (line_info.length - REEL_IN_SPEED * time.delta_seconds()).max(0.);

            if line_info.length == 0. {
                // Line fully reeled back in
                next_state.set(FishingState::Idle);
                return;
            }
        }

        let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
        let rod_end = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
        let line_end = rod_transform.translation.xy() + (rod_info.length / 2. + line_info.length) * angle_vector;

        line_info.start = rod_end;
        line_info.end = line_end;
    
        bobber_transform.translation = line_end.extend(950.);
    }
}

pub fn is_fish_caught (
    fishing_view: Res<FishingView>,
    mut money: ResMut<Money>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut fish: Query<(&mut Fish, &mut PhysicsObject), With<Fish>>,
) {
    let (rod_info, rod_transform) = rod.single();
    let (mut fish_details, mut fish_physics) = fish.single_mut();

    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    let catch_pos = rod_transform.translation.with_z(0.) + (rod_info.length / 4. * angle_vector).extend(0.);
    let distance = (fish_physics.position - catch_pos).length();

    if distance < 15. {
        fish_details.is_caught = true;
        money.amount += 100;

        // Reset fish for testing
        fish_physics.position = Vec3::new(FISHINGROOMX, FISHINGROOMY, 0.);
        fish_physics.velocity = Vec3::new(0., 0., 0.);
        fish_physics.forces = Forces::default();
        fish_details.is_caught = false;

        next_state.set(FishingState::Idle);
    }
}

pub fn reset_interface (
    mut fishing_view: ResMut<FishingView>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut splash: Query<&mut TextureAtlas, With<Splash>>,
    mut bobber: Query<&mut Visibility, With<Bobber>>
) {
    let mut line_info = line.single_mut();
    let mut splash_texture = splash.single_mut();
    let mut bobber_visibility = bobber.single_mut();
    
    line_info.fish_on = false;
    line_info.length = 0.;
    line_info.start = Vec2::ZERO;
    line_info.end = Vec2::ZERO;
    *bobber_visibility = Visibility::Hidden;
    splash_texture.index = 0;
    fishing_view.power = 0.;
}

pub fn move_physics_objects (
    mut objects: Query<(&PhysicsObject, &mut Transform), With<PhysicsObject>>
) {
    for (physics_object, mut transform) in objects.iter_mut() {
        transform.translation = physics_object.position.with_z(901.);
        transform.rotation = Quat::from_rotation_z(physics_object.rotation.z);
    }
}

pub fn draw_fishing_line (
    mut meshes: ResMut<Assets<Mesh>>,
    mut line: Query<(&mut Transform, &mut Mesh2dHandle, &mut FishingLine), (With<FishingLine>, Without<FishingRod>)>,
) {
    let (mut line_transform, mut line_mesh, mut line_info) = line.single_mut();

    let pos_delta = line_info.end - line_info.start;
    let line_length = pos_delta.length();
    let line_pos = (line_info.start + line_info.end) / 2.;
    let line_rotation =  f32::atan2(pos_delta.y, pos_delta.x) + PI / 2.;

    // Draw fishing line
    line_transform.translation = Vec3::new(line_pos.x, line_pos.y, line_transform.translation.z);
    line_transform.rotation = Quat::from_rotation_z(line_rotation);

    meshes.remove(&line_info.mesh_handle);
    line_info.mesh_handle = meshes.add(Rectangle::new(FishingLine::WIDTH, line_length));
    *line_mesh = Mesh2dHandle(line_info.mesh_handle.clone());
}

pub fn animate_splash(
    mut splash: Query<(&Splash, &mut Transform, &mut Visibility, &mut TextureAtlas, &mut AnimationTimer), With<Splash>>,
    time: Res<Time>
) {
    let (splash, mut transform, mut visibility, mut texture, mut timer) = splash.single_mut();

    if *visibility == Visibility::Hidden {
        return;
    }

    transform.translation = splash.position;

    // Animate splash
    if texture.index < 3 {
        timer.tick(time.delta());

        if timer.just_finished() {
            if texture.index == 2 {
                *visibility = Visibility::Hidden;
            } else {
                texture.index += 1;
            }
        }
    }
}

pub fn animate_waves (
    objects: Query<&PhysicsObject, With<PhysicsObject>>,
    mut wave: Query<(&mut TextureAtlas, &mut Transform, &mut Visibility), With<Wave>>
) {
    let object = objects.single();
    let (mut wave_texture, mut wave_transform, mut wave_visibility) = wave.single_mut();

    let magnitude = object.forces.water.length();

    if magnitude == 0. {
        *wave_visibility = Visibility::Hidden;
        return
    }
    
    if magnitude < 200. {
        wave_texture.index = 0;
    } else if magnitude < 400. {
        wave_texture.index = 1;
    } else if magnitude < 600. {
        wave_texture.index = 2;
    } else {
        wave_texture.index = 3;
    }

    *wave_visibility = Visibility::Visible;
    wave_transform.translation = object.position.with_z(902.);
    wave_transform.rotation = Quat::from_rotation_z(f32::atan2(object.forces.water.y, object.forces.water.x) - PI / 2.);
}
