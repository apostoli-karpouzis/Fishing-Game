use bevy::prelude::*;
use crate::player::*;
use crate::resources::*;

pub fn move_camera(
    mut camera: Query<(&mut Transform, &mut Animation), (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    state: Res<State<GameState>>
) {
    let (mut ct, camera_animation) = camera.single_mut();

    // Camera animation
    if state.eq(&GameState::MapTransition) {
        let elapsed: f32 = time.elapsed_seconds() - camera_animation.start_time;
        
        if elapsed < camera_animation.duration {
            ct.translation = camera_animation.start_position + elapsed / camera_animation.duration * camera_animation.motion;
        } else {
            ct.translation = camera_animation.start_position + camera_animation.motion;
        }

        return;
    }
}