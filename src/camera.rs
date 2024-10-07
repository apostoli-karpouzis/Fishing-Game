use bevy::prelude::*;
use crate::player::*;
use crate::map::*;
use crate::resources::*;

const MAP_TRANSITION_TIME: f32 = 1.5;

pub fn move_camera(
    mut player: Query<(&mut Location, &Transform, &PlayerDirection, &mut Animation), With<Player>>,
    mut camera: Query<(&mut Transform, &mut Animation), (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut map_location, pt, direction, mut player_animation) = player.single_mut();
    let (mut ct, mut camera_animation) = camera.single_mut();

    // Camera animation
    if state.eq(&GameState::MapTransition) {
        let elapsed: f32 = time.elapsed_seconds() - camera_animation.start_time;
        
        if elapsed < camera_animation.duration {
            ct.translation = camera_animation.start_position + elapsed / camera_animation.duration * camera_animation.motion;
        } else {
            ct.translation = camera_animation.start_position + camera_animation.motion;
            next_state.set(GameState::Normal);
        }

        return;
    }
    
    // Check for edge collision
    let mut player_offset: Vec3 = Vec3::ZERO;
    let mut camera_offset: Vec3 = Vec3::ZERO;

    if *direction == PlayerDirection::Right {
        if pt.translation.x + PLAYER_WIDTH / 2. >= ct.translation.x + WIN_W / 2. && map_location.x + 1 < map_location.map.width  {
            map_location.x += 1;
            player_offset = Vec3::new(PLAYER_WIDTH, 0., 0.);
            camera_offset = Vec3::new(WIN_W, 0., 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Left {
        if pt.translation.x - PLAYER_WIDTH / 2. <= ct.translation.x - WIN_W / 2. && map_location.x != 0 {
            map_location.x -= 1;
            player_offset = Vec3::new(-PLAYER_WIDTH, 0., 0.);
            camera_offset = Vec3::new(-WIN_W, 0., 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Back {
        if pt.translation.y + PLAYER_HEIGHT / 2. >= ct.translation.y + WIN_H / 2. && map_location.y + 1 < map_location.map.height  {
            map_location.y += 1;
            player_offset = Vec3::new(0., PLAYER_HEIGHT, 0.);
            camera_offset = Vec3::new(0., WIN_H, 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Front {
        if pt.translation.y - PLAYER_HEIGHT / 2. <= ct.translation.y - WIN_H / 2. && map_location.y != 0 {
            map_location.y -= 1;
            player_offset = Vec3::new(0., -PLAYER_HEIGHT, 0.);
            camera_offset = Vec3::new(0., -WIN_H, 0.);
        } else {
            return;
        }
    }

    next_state.set(GameState::MapTransition);

    *player_animation = Animation {
        start_time: time.elapsed_seconds(),
        duration: MAP_TRANSITION_TIME,
        start_position: pt.translation,
        motion: player_offset
    };

    *camera_animation = Animation {
        start_time: time.elapsed_seconds(),
        duration: MAP_TRANSITION_TIME,
        start_position: ct.translation,
        motion: camera_offset
    }
}