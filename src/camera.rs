use bevy::prelude::*;
use crate::resources::*;
//use crate::player::*;

// if you have multiple states that must be set correctly,
// don't forget to manage them all




pub fn move_camera(
    player: Query<(&PlayerDirection, &Transform), With<Player>>,
    mut camera: Query<(&mut Transform, &mut CameraAnimation), (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (direction, pt) = player.single();
    let (mut ct, mut animation) = camera.single_mut();

    if state.eq(&GameState::MapTransition) {
        let elapsed: f32 = time.elapsed_seconds() - animation.start_time;
        
        if elapsed < MAP_TRANSITION_TIME {
            ct.translation = animation.start_position + elapsed / MAP_TRANSITION_TIME * animation.motion;
        } else {
            ct.translation = animation.start_position + animation.motion;
            next_state.set(GameState::Normal);
        }
    } else {
        let offset: Vec3;
        if *direction == PlayerDirection::Right && pt.translation.x + PLAYER_WIDTH / 2. >= ct.translation.x + WIN_W / 2. {
            offset = Vec3::new(WIN_W, 0., 0.)
        } else if *direction == PlayerDirection::Left && pt.translation.x - PLAYER_WIDTH / 2. <= ct.translation.x - WIN_W / 2. {
            offset = Vec3::new(-WIN_W, 0., 0.)
        } else if *direction == PlayerDirection::Back && pt.translation.y + PLAYER_HEIGHT / 2. >= ct.translation.y + WIN_H / 2. {
            offset = Vec3::new(0., WIN_H, 0.)
        } else if *direction == PlayerDirection::Front && pt.translation.y - PLAYER_HEIGHT / 2. <= ct.translation.y - WIN_H / 2. {
            offset = Vec3::new(0., -WIN_H, 0.)
        } else {
            return;
        }

        next_state.set(GameState::MapTransition);
        *animation = CameraAnimation {
            start_time: time.elapsed_seconds(),
            start_position: ct.translation,
            motion: offset
        }
    }
}