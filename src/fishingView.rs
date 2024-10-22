use std::f32;
use f32::consts::PI;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use crate::resources::*;
use crate::fish::*;
use crate::species::*;

const TUG: KeyCode = KeyCode::KeyP;
const REEL: KeyCode = KeyCode::KeyO;

const MAX_CAST_DISTANCE: f32 = 400.;

#[derive(Component)]

pub struct Bobber;

#[derive(Component)]
pub struct FishingRod {
    pub length: f32
}

#[derive(Component)]
pub struct Wave;

#[derive(Component, Default)]
pub struct Splash {
    pub position: Vec3
}

#[derive(Component)]
pub struct RotationObj{
    pub rot: f32,
}

#[derive(Component)]
pub struct Rotatable;

#[derive(Component, Default)]
pub struct FishingLine {
    pub casting: bool,
    pub cast_distance: f32,
    pub length: f32
}

impl FishingLine {
    pub const WIDTH: f32 = 3.;
}

#[derive(Component)]
pub struct PowerBar {
    pub meter: i32,
    pub released: bool
}

impl PowerBar {
    pub const MAX_POWER: i32 = 250;
}

pub fn fishing_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut return_val: ResMut<PlayerReturnPos>,
    mut power_bar: Query<(&mut Transform, &mut PowerBar), (Without<Camera>, With<PowerBar>)>,
    mut rod: Query<(&mut Transform, &mut RotationObj), (Without<Camera>, Without<PowerBar>, With<Rotatable>)>,
){
    let mut ct = camera.single_mut();

    return_val.player_save_x = ct.translation.x;
    return_val.player_save_y = ct.translation.y;

    ct.translation.x = FISHINGROOMX;
    ct.translation.y = FISHINGROOMY;
    //FISHINGROOMY-308
    //spawn in powerbar
    //commands.spawn
    let (mut pb, mut power) = power_bar.single_mut();
    pb.translation.y = FISHINGROOMY - 308.;
    power.meter = 0;

    power.released = false;

    //rd
    let (mut rd, mut rot_obj) = rod.single_mut();
    rot_obj.rot = 0.;
    rd.rotation = Quat::from_rotation_z(rot_obj.rot);
    
    //new movmemnt system, rotation then space hold.
    //powerbar is space A, D are rotational
    
    
}

pub fn overworld_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    //mut power_bar: Query<(&mut Transform, &mut Power), With<Bar>>,
    return_val: ResMut<PlayerReturnPos>
) {
    let mut ct = camera.single_mut();
    //let (mut pb, mut power) = power_bar.single_mut();
    ct.translation.x = return_val.player_save_x;
    ct.translation.y = return_val.player_save_y;

    //pb.translation.y = (FISHINGROOMY - 308.);
    //power.meter = 0;
    //set powerbar back to 0
    //set rotation back to 0
}

pub fn power_bar_cast(
    input: Res<ButtonInput<KeyCode>>,
    mut power_bar: Query<(&mut Transform, &mut PowerBar), With<PowerBar>>,
    mut line: Query<(&mut Visibility, &mut FishingLine), With<FishingLine>>,
){
    let (mut pb, mut power) = power_bar.single_mut();
    let (mut line_visibility, mut line_info) = line.single_mut();

    if power.released {
        // Casting disabled
        return;
    } else if power.meter == PowerBar::MAX_POWER {
        // Max power reached, release
        power.released = true;
        *line_visibility = Visibility::Visible;
        line_info.casting = true;
        line_info.cast_distance = power.meter as f32 / PowerBar::MAX_POWER as f32 * MAX_CAST_DISTANCE;
    } else if input.pressed(TUG) {
        // Increase power
        pb.translation.y += 5.;
        power.meter += 5;
    } else if input.just_released(TUG) {
        // Manual release
        power.released = true;
        *line_visibility = Visibility::Visible;
        line_info.casting = true;
        line_info.cast_distance = power.meter as f32 / PowerBar::MAX_POWER as f32 * MAX_CAST_DISTANCE;
    }
}

pub fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut rod: Query<(&mut Transform, &mut RotationObj), With<Rotatable>>,
) {
    let (mut rd, mut rot_obj) = rod.single_mut();

    if input.pressed(KeyCode::KeyA) {
        if rot_obj.rot <= 1.2 {
            rot_obj.rot += 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    }
    
    if input.pressed(KeyCode::KeyD) {
        if rot_obj.rot >= -1.2 {
            rot_obj.rot -= 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    }
}

pub fn animate_fishing_line (
    rod: Query<(&FishingRod, &Transform, &RotationObj), (With<FishingRod>, With<Rotatable>)>,
    fish: Query<(&Species, &Fish), With<FishHooked>>,
    mut line: Query<(&mut Transform, &mut Visibility, &mut Mesh2dHandle, &mut FishingLine), (With<FishingLine>, Without<Rotatable>)>,
    mut power_bar: Query<(&mut PowerBar, &mut Transform), (With<PowerBar>, Without<Rotatable>, Without<FishingLine>, Without<Wave>, Without<Bobber>, Without<FishingRod>)>,
    mut splash: Query<(&mut Splash, &mut TextureAtlas, &mut Visibility), (With<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    mut wave: Query<(&mut TextureAtlas, &mut Transform, &mut Visibility),(With<Wave>, Without<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    mut bobber: Query<(&mut Transform, &mut Visibility), (With<Bobber>, Without<Wave>, Without<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    input: Res<ButtonInput<KeyCode>>
) {
    let (rod_info, rod_transform, rod_rotation) = rod.single();
    let (fish_species, fish_state) = fish.single();
    let (mut line_transform, mut line_visibility, mut line_mesh,mut line_info) = line.single_mut();
    let (mut power_info, mut pb_transform) = power_bar.single_mut();
    
    let (mut splash, mut splash_texture, mut splash_visibility) = splash.single_mut();
    let(mut wave_texture, mut wave_transform, mut wave_visibility) = wave.single_mut();
    let(mut bobber_transform, mut bobber_visibility) = bobber.single_mut();

    if *line_visibility == Visibility::Hidden {
        return;
    }
    
    *bobber_visibility = Visibility::Visible;

    let fish_hooked = false;
    
    let line_rotation: f32;
    let line_pos: Vec2;

    // Fish hooked
    if fish_hooked {
        if fish_state.is_caught == true {
            *line_visibility = Visibility::Hidden;
            *wave_visibility = Visibility::Hidden;
            *bobber_visibility = Visibility::Hidden;
            splash_texture.index = 0;
            power_info.released = false;
            power_info.meter = 0;
            pb_transform.translation.y = 3292.; 
            return;
        }

        let rod_end = Vec2::new(rod_transform.translation.x + rod_info.length / 2. * f32::cos(rod_rotation.rot + PI / 2.), rod_transform.translation.y + rod_info.length / 2. * f32::sin(rod_rotation.rot + PI / 2.));
        let fish_offset = fish_species.hook_pos.xy().rotate(-fish_state.velocity.xy().normalize_or_zero());
        let fish_pos = fish_state.position.xy() + fish_offset;
        let pos_delta = fish_pos - rod_end;
        
        // Hook line to fish
        line_info.length = Vec2::distance(rod_end, fish_pos);
        line_rotation = f32::atan2(pos_delta.y, pos_delta.x) + PI / 2.;
        line_pos = (rod_end + fish_pos) / 2.;

        if input.pressed(REEL) {
            *wave_visibility = Visibility::Visible;
            wave_transform.translation = Vec3::new(rod_end.x + line_info.length * f32::cos(line_rotation - PI / 2.) ,rod_end.y + line_info.length * f32::sin(line_rotation - PI / 2.), 950.);
            let magnitude = fish_state.velocity.x.powf(2.) + fish_state.velocity.y.powf(2.);
            println!("str: {}" , magnitude);
            
            if magnitude < 130. {
                wave_texture.index = 1;
            } if magnitude >= 130. && magnitude < 160. {
                wave_texture.index = 2;
            } if magnitude >= 160. {
                wave_texture.index = 3;
            }
        } else {
            *wave_visibility = Visibility::Hidden;
        }

        bobber_transform.translation =  Vec3::new(rod_end.x + line_info.length * f32::cos(line_rotation - PI / 2.) ,rod_end.y + line_info.length * f32::sin(line_rotation - PI / 2.), 950.);
    } else {
        if line_info.casting
        {
            // Casting animation
            let new_length = line_info.length + 2.;

            if new_length >= line_info.cast_distance {
                // Cast finished
                line_info.length = line_info.cast_distance;
                line_info.casting = false;

                // Splash animation
                *splash_visibility = Visibility::Visible;
                splash.position = Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_info.length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_info.length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
            } else {
                // Casting
                line_info.length = new_length;
            }
        } else {
            // Reeling
            if input.pressed(REEL) {
                if line_info.length >= 1. {
                    line_info.length -= 1.;

                    *wave_visibility = Visibility::Visible;
                    wave_transform.translation = Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_info.length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_info.length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
                } else {
                    // Line fully reeled back in
                    *line_visibility = Visibility::Hidden;
                    *wave_visibility = Visibility::Hidden;
                    *bobber_visibility = Visibility::Hidden;
                    splash_texture.index = 0;
                    power_info.released = false;
                    power_info.meter = 0;
                    pb_transform.translation.y = 3292.;
                    return;
                }
            } else {
                *wave_visibility = Visibility::Hidden;
            }
        }

        line_rotation = rod_rotation.rot;
        line_pos = Vec2::new(rod_transform.translation.x + (rod_info.length + line_info.length) / 2. * f32::cos(rod_rotation.rot + PI / 2.), rod_transform.translation.y + (rod_info.length + line_info.length) / 2. * f32::sin(rod_rotation.rot + PI / 2.));
        bobber_transform.translation =  Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_info.length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_info.length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
    }

    line_transform.translation = Vec3::new(line_pos.x, line_pos.y, 901.);
    line_transform.rotation = Quat::from_rotation_z(line_rotation);
    *line_mesh = Mesh2dHandle(meshes.add(Rectangle::new(FishingLine::WIDTH, line_info.length)));
}

pub fn animate_fish (
    mut fish_info: Query<(&Fish, &mut Transform), With<FishHooked>>
) {
    let (fish, mut fish_transform) = fish_info.single_mut();

    fish_transform.translation.x = fish.position.x;
    fish_transform.translation.y = fish.position.y;

    if fish.velocity.x != 0. || fish.velocity.y != 0. {
        let angle = f32::atan2(fish.velocity.y, fish.velocity.x) + PI;
        fish_transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn animate_splash (
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
            if texture.index == 2
            {
                *visibility = Visibility::Hidden;
            } else {
                println!("finished!");
                texture.index += 1;
            }    
        }
    }
}

pub fn run_if_in_overworld(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Overworld)
}
pub fn run_if_in_fishing(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Fishing)
}


