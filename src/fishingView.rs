use std::f32;
use f32::consts::PI;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use crate::resources::*;
use crate::fish::*;
use crate::species::*;

const MAX_CAST_DISTANCE: f32 = 400.;

#[derive(Component)]

pub struct Bobber;

#[derive(Component)]
pub struct FishingRod {
    pub length: f32
}

#[derive(Component)]
pub struct Wave;

#[derive(Component)]
pub struct Splash;

#[derive(Component)]
pub struct RotationObj{
    pub rot: f32,
}

#[derive(Component)]
pub struct Rotatable;

#[derive(Component)]
pub struct FishingLine {
    pub length: f32
}

impl FishingLine {
    pub const WIDTH: f32 = 3.;
}

#[derive(Component)]
pub struct PowerBar {
    pub meter: i32,
    pub released: bool,
    pub just_released: bool
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
    mut line: Query<&mut Visibility, With<FishingLine>>,
    mut splash: Query<(&mut Transform, &mut Visibility), (With<Splash>, Without<FishingLine>, Without<PowerBar>)>,
    rod: Query<(&FishingRod, &Transform, &RotationObj), (With<FishingRod>, With<Rotatable>, Without<PowerBar>, Without<Splash>)>, 
){
    let (mut pb, mut power) = power_bar.single_mut();
    let mut line_visibility = line.single_mut();
    let (mut splash_transform, mut splash_visibility ) = splash.single_mut();
    let (rod_info, rod_transform, rod_rotation) = rod.single();

    if power.meter == PowerBar::MAX_POWER {
        if power.released != true {
            *line_visibility = Visibility::Visible;
            println!("you have released the P button");
            println!("filled1");
            let line_length = power.meter as f32 / PowerBar::MAX_POWER as f32 * MAX_CAST_DISTANCE;
            *splash_visibility = Visibility::Visible;
            splash_transform.translation = Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
            power.released = true;
            power.just_released = true;
        }
    } else {
        if input.pressed(KeyCode::KeyP) && power.released == false{
            println!("raising powerbarp {} {}", pb.translation.y, power.meter);
            pb.translation.y += 5.;
            power.meter += 5;
        }
        if input.just_released(KeyCode::KeyP){
            *line_visibility = Visibility::Visible;
            println!("you have released the P button");
            let line_length = power.meter as f32 / PowerBar::MAX_POWER as f32 * MAX_CAST_DISTANCE;
            *splash_visibility = Visibility::Visible;
            splash_transform.translation = Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
            power.released = true;
            power.just_released = true;

        }
    }
}

pub fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut rod: Query<(&mut Transform, &mut RotationObj), With<Rotatable>>,
) {
    let (mut rd, mut rot_obj) = rod.single_mut();

    if input.pressed(KeyCode::KeyA){
        if rot_obj.rot <= 1.2{
            rot_obj.rot += 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    
    }
    
    if input.pressed(KeyCode::KeyD){
        if rot_obj.rot >= -1.2{
            rot_obj.rot -= 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    }
}

pub fn animate_fishing_line(
    mut rod: Query<(&FishingRod, &Transform, &RotationObj), (With<FishingRod>, With<Rotatable>)>,
    mut fish: Query<(&Species, &Fish), With<FishHooked>>,
    mut line: Query<(&mut Transform, &Visibility, &mut Mesh2dHandle, &mut FishingLine), (With<FishingLine>, Without<Rotatable>)>,
    mut power_bar: Query<&mut PowerBar, With<PowerBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut splash: Query<(&mut TextureAtlas, &mut AnimationTimer, &mut Visibility), (With<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    mut wave: Query<(&mut TextureAtlas, &mut Transform, &mut Visibility),(With<Wave>, Without<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    mut bobber: Query<(&mut Transform, &mut Visibility), (With<Bobber>, Without<Wave>, Without<Splash>, Without<FishingLine>, Without<Rotatable>)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (rod_info, rod_transform, rod_rotation) = rod.single();
    let (fish_attributes, fish_state) = fish.single();
    let (mut line_transform, line_visibility, mut line_mesh,mut line_info) = line.single_mut();
    let mut power_info = power_bar.single_mut();
    let (mut texture, mut timer, mut Visibility ) = splash.single_mut();
    let(mut wave_texture, mut wave_transform, mut wave_visibility) = wave.single_mut();
    let(mut bobber_transform, mut bobber_visibility) = bobber.single_mut();

    if line_visibility == Visibility::Hidden {
        return;
    }
    
    *bobber_visibility = Visibility::Visible;

    let fish_hooked = false;
    
    let line_length: f32;
    let line_rotation: f32;
    let line_pos: Vec2;

    // Fish hooked
    if fish_hooked {
        let rod_end = Vec2::new(rod_transform.translation.x + rod_info.length / 2. * f32::cos(rod_rotation.rot + PI / 2.), rod_transform.translation.y + rod_info.length / 2. * f32::sin(rod_rotation.rot + PI / 2.));
        let fish_pos = Vec2::new(fish_state.position.x, fish_state.position.y);
        let pos_delta = Vec2::new(fish_pos.x - rod_end.x, fish_pos.y - rod_end.y);
        
        if pos_delta == Vec2::ZERO {
            line_length = 0.;
            line_rotation = 0.;
            line_pos = rod_end;
        } else {
            line_length = Vec2::distance(rod_end, fish_pos);
            line_rotation = f32::atan2(pos_delta.y, pos_delta.x) + PI / 2.;
            line_pos = (rod_end + fish_pos) / 2.;
            if input.pressed(KeyCode::KeyO){//if reeling in
                *wave_visibility = Visibility::Visible;
                wave_transform.translation = Vec3::new(rod_end.x + line_length * f32::cos(line_rotation - PI / 2.) ,rod_end.y + line_length * f32::sin(line_rotation - PI / 2.), 950.);
                let magnitude = fish_state.velocity.x.powf(2.) + fish_state.velocity.x.powf(2.);
                println!("str: {}" , magnitude);
                if magnitude < 120. {
                    wave_texture.index = 1;
                }if magnitude >= 120. && magnitude < 160. {
                    wave_texture.index = 2;
                }if magnitude >= 160. {
                    wave_texture.index = 3;
                }
            }else{
                *wave_visibility = Visibility::Hidden;
            }
            bobber_transform.translation =  Vec3::new(rod_end.x + line_length * f32::cos(line_rotation - PI / 2.) ,rod_end.y + line_length * f32::sin(line_rotation - PI / 2.), 950.);


        }
    } else {
        if power_info.just_released
        {        
            line_length = power_info.meter as f32 / PowerBar::MAX_POWER as f32 * MAX_CAST_DISTANCE;
            power_info.just_released = false;
            line_info.length = line_length;
        }else {
            if input.pressed(KeyCode::KeyO) {
                if line_info.length >= 1. {
                    line_info.length -= 1.;
                    *wave_visibility = Visibility::Visible;
                    wave_transform.translation = Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_info.length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_info.length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
                }
            }else{
                *wave_visibility = Visibility::Hidden;
            }
            line_length = line_info.length;
        }
        line_rotation = rod_rotation.rot;
        line_pos = Vec2::new(rod_transform.translation.x + (rod_info.length + line_length) / 2. * f32::cos(rod_rotation.rot + PI / 2.), rod_transform.translation.y + (rod_info.length + line_length) / 2. * f32::sin(rod_rotation.rot + PI / 2.));
        bobber_transform.translation =  Vec3::new(rod_transform.translation.x + (rod_info.length + 2. * line_info.length)/2. * f32::cos(rod_rotation.rot + PI / 2.) ,rod_transform.translation.y + (rod_info.length + 2. * line_info.length)/2. * f32::sin(rod_rotation.rot + PI / 2.), 950.);
    }

    if texture.index < 3{
        timer.tick(time.delta());
        if timer.just_finished() {
            if(texture.index == 2)
            {
                *Visibility = Visibility::Hidden;
            }else{
                println!("finished!");
                texture.index += 1;
            }    

        }
    } 

    line_transform.translation = Vec3::new(line_pos.x, line_pos.y, 901.);
    line_transform.rotation = Quat::from_rotation_z(line_rotation);
    *line_mesh = Mesh2dHandle(meshes.add(Rectangle::new(FishingLine::WIDTH, line_length)));
    
}

pub fn run_if_in_overworld(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Overworld)
}
pub fn run_if_in_fishing(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Fishing)
}


