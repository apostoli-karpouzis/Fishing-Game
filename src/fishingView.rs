use bevy::prelude::*;
use crate::resources::*;
//use crate::button::*;

#[derive(Component)]
pub struct Bar;

#[derive(Component)]
pub struct RotationObj{
    pub rot: f32,
}

#[derive(Component)]
pub struct Rotatable;


#[derive(Component)]
pub struct Power {
    pub meter: i32,
    pub released: bool 
}

pub fn fishing_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut return_val: ResMut<PlayerReturnPos>,
    mut power_bar: Query<(&mut Transform, &mut Power), (Without<Camera>, With<Bar>)>,
    mut rod: Query<(&mut Transform, &mut RotationObj), (Without<Camera>, Without<Bar>, With<Rotatable>)>,
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
){
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
    mut power_bar: Query<(&mut Transform, &mut Power), With<Bar>>,
){
    let (mut pb, mut power) = power_bar.single_mut();
    if power.meter <= 245{
        if input.pressed(KeyCode::KeyP) && power.released == false{
            println!("raising powerbarp {} {}", pb.translation.y, power.meter);
            pb.translation.y += 5.;
            power.meter += 5;
        }
        if input.just_released(KeyCode::KeyP){
            println!("you have released the P button");
            power.released = true;
        }
    }
    else if power.released == true {
        println!("filled");
    }
    else{
        println!("filled");
        power.released = true;
    }
}
pub fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    mut rod: Query<(&mut Transform, &mut RotationObj), With<Rotatable>>,
){
    let (mut rd, mut rot_obj) = rod.single_mut();
    //let 
    //rod.rotation

    if input.pressed(KeyCode::KeyA){
        if rot_obj.rot <= 1.2{
            println!("{}", rot_obj.rot);
            rot_obj.rot += 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    
    }
    if input.pressed(KeyCode::KeyD){
        if rot_obj.rot >= -1.2{
            println!("{}", rot_obj.rot);
            rot_obj.rot -= 0.02;
            rd.rotation = Quat::from_rotation_z(rot_obj.rot);
        }
    }
}


pub fn run_if_in_overworld(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Overworld)
}
pub fn run_if_in_fishing(state: Res<State<FishingMode>>) -> bool{
    state.eq(&FishingMode::Fishing)
}

pub fn switch_mode(
    mut next: ResMut<NextState<FishingMode>>,
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<FishingMode>>
){
    // if state.eq(&FishingMode::Overworld){
    //     if input.just_pressed(KeyCode::KeyX){
    //         next.set(state.get().next());
    //         println!("switching to fising");
    //     }
    // }
    if state.eq(&FishingMode::Fishing){
        if input.just_pressed(KeyCode::KeyX){
            next.set(state.get().next());
            println!("switching to overworld");
        }
    }
}

