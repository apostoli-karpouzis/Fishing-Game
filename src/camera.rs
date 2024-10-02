use bevy::prelude::*;
use crate::player::*;
use crate::resources::*;

const MAP_TRANSITION_TIME: f32 = 1.5;

#[derive(Component)]
pub struct CameraAnimation {
    pub start_time: f32,
    pub start_position: Vec3,
    pub motion: Vec3,
}

impl CameraAnimation {
    pub fn new() -> Self {
        Self {
            start_time: 0.,
            start_position: Vec3::default(),
            motion: Vec3::default(),
        }
    }
}

pub fn move_camera(
    player: Query<(&PlayerDirection, &Transform), With<Player>>,
    mut camera: Query<(&mut Transform, &mut CameraAnimation), (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (direction, pt) = player.single();
    let (mut ct, mut animation) = camera.single_mut();

    // Camera animation
    if state.eq(&GameState::MapTransition) {
        let elapsed: f32 = time.elapsed_seconds() - animation.start_time;
        
        if elapsed < MAP_TRANSITION_TIME {
            ct.translation = animation.start_position + elapsed / MAP_TRANSITION_TIME * animation.motion;
        } else {
            ct.translation = animation.start_position + animation.motion;
            next_state.set(GameState::Normal);
        }

        return;
    }
    
    // Check for edge collision
    let offset: Vec3;

    // Right side 
    if *direction == PlayerDirection::Right && pt.translation.x + PLAYER_WIDTH / 2. >= ct.translation.x + WIN_W / 2. {
        offset = Vec3::new(WIN_W, 0., 0.)
    // Left side
    } else if *direction == PlayerDirection::Left && pt.translation.x - PLAYER_WIDTH / 2. <= ct.translation.x - WIN_W / 2. {
        offset = Vec3::new(-WIN_W, 0., 0.)
    // Top side
    } else if *direction == PlayerDirection::Back && pt.translation.y + PLAYER_HEIGHT / 2. >= ct.translation.y + WIN_H / 2. {
        offset = Vec3::new(0., WIN_H, 0.)
    // Bottom side
    } else if *direction == PlayerDirection::Front && pt.translation.y - PLAYER_HEIGHT / 2. <= ct.translation.y - WIN_H / 2. {
        offset = Vec3::new(0., -WIN_H, 0.)
    // No edge collision
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