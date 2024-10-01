use bevy::prelude::*;
use crate::resources::*;
//use crate::player::*;

// if you have multiple states that must be set correctly,
// don't forget to manage them all

pub fn move_camera(
    player: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
    mut new_state: ResMut<NextState<GameState>>,
    grid_loc: ResMut<Location>,
    mut dir: ResMut<CameraDirection>,
) {
    let pt = player.single();

    let new_pos_x = pt.translation.x;
    let new_pos_y = pt.translation.y;
    let grid_hold = grid_loc.i;
    let grid_hold_y = grid_loc.j;
    //ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
    if new_pos_x <= (WIN_W as f32 * grid_hold as f32) - (WIN_W / 2.) + ((TILE_SIZE as f32) / 4.) {
        //println!("hit the left {}", (grid_hold * 1280));

        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::West;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
    if new_pos_x >= (WIN_W as f32 * grid_hold as f32) + WIN_W - (WIN_W / 2. + (TILE_SIZE as f32) / 4.) {
        //println!("hit the right {}", (WIN_W - (WIN_W / 2. + (TILE_SIZE as f32) / 4.)));
        //println!("{:?}", game_state.get());
        if game_state.get() == &GameState::CamStill {
            
            *dir = CameraDirection::East;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }

        //switching move states
    }

    if new_pos_y <= (WIN_H as f32 * grid_hold_y as f32) - (WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5) {
        println!("hit the bottom");
        println!("{}", (WIN_H as f32 * grid_hold_y as f32) - (WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5));
        println!("{}", new_pos_y);
        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::South;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
    if new_pos_y > (grid_hold_y as f32 * WIN_H as f32) + WIN_H / 2. - ((TILE_SIZE as f32) * 0.5) {
        println!("hit the top");
        
        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::North;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
}

pub fn pan_cam(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    game_state: Res<State<GameState>>,
    mut new_state: ResMut<NextState<GameState>>,
    mut time_stuff: Query<&mut CameraSpeed>,
    mut grid_loc: ResMut<Location>,
    mut dir: ResMut<CameraDirection>,
) {
    let grid_hold = grid_loc.i;
    let grid_hold_y = grid_loc.j;

    let mut timer = time_stuff.single_mut();
    //print!("function wokring");

    let mut ct = camera.single_mut();
    //println!("{:?}", game_state.get());
    if game_state.get() == &GameState::CamMove {
        match *dir {
            CameraDirection::North => {
                println!("going North");
                if ct.translation.y <= (720. + (720. * grid_hold_y as f32)){
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        //println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.y += 9.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.j = grid_loc.j + 1;
                    match game_state.get() {
                        
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                    println!("ending");
                }
            }
            CameraDirection::South => {
                if ct.translation.y >= (-720. + (720. * grid_hold_y as f32)) {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.y -= 9.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.j = grid_loc.j - 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                    println!("ending");
                }
            }
            CameraDirection::West => {
                if ct.translation.x >= (-1280. + (TILE_SIZE as f32) / 4.) + (1280 as f32 * grid_hold as f32){
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        //println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.x -= 16.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.i = grid_loc.i - 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                }
            }
            CameraDirection::East => {
                println!("going East");
                if ct.translation.x < 1280. + (1280 as f32 * grid_hold as f32) {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.x += 16.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.i = grid_loc.i + 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                }
            }
            CameraDirection::None => {
                println!("still");
            }
        }
    }
}