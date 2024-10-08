use bevy::prelude::*;
use super::map::*;
use super::resources::*;
use std::time::Duration;

pub const PLAYER_WIDTH: f32 = 64.;
pub const PLAYER_HEIGHT: f32 = 128.;

const PLAYER_SPEED: f32 = 200.;

#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Clone)]
pub enum PlayerDirection {
    Front,
    Back,
    Left,
    Right,
}

//stack to store movement input to avoid priorit movement
//last key pressed is looked at
#[derive(Default, Component)]
pub struct InputStack {
    stack: Vec<KeyCode>,
}

impl InputStack {
    fn push(&mut self, key: KeyCode) {
        if !self.stack.contains(&key) {
            self.stack.push(key);
        }
    }

    fn remove(&mut self, key: KeyCode) {
        self.stack.retain(|&k| k != key);
    }

    fn last(&self) -> Option<KeyCode> {
        self.stack.last().copied()
    }
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut PlayerDirection, &Location, &Animation, &mut InputStack), With<Player>>,
    collision_query: Query<(&Transform, &Tile), (With<Collision>, Without<Player>)>,
    state: Res<State<GameState>>,
) {
    let (mut pt, mut direction, location, animation, mut input_stack) = player.single_mut();

    // Move player during area transition
    if state.eq(&GameState::MapTransition) {
        let elapsed: f32 = time.elapsed_seconds() - animation.start_time;
        
        if elapsed < animation.duration {
            pt.translation = animation.start_position + elapsed / animation.duration * animation.motion;
        } else {
            pt.translation = animation.start_position + animation.motion;
        }

        return;
    }

    let up = KeyCode::KeyW;
    let left = KeyCode::KeyA;
    let down = KeyCode::KeyS;
    let right = KeyCode::KeyD;

    // Add pressed keys to stack
    if input.pressed(up) {
        input_stack.push(up);
    } else {
        input_stack.remove(up);
    }

    if input.pressed(left) {
        input_stack.push(left);
    } else {
        input_stack.remove(left);
    }

    if input.pressed(down) {
        input_stack.push(down);
    } else {
        input_stack.remove(down);
    }

    if input.pressed(right) {
        input_stack.push(right);
    } else {
        input_stack.remove(right);
    }

    // Get last key for direction
    let mut change_direction = Vec2::ZERO;

    if let Some(last_key) = input_stack.last() {
        match last_key {
            KeyCode::KeyW => {
                change_direction.y += PLAYER_SPEED;
                *direction = PlayerDirection::Back;
            }
            KeyCode::KeyA => {
                change_direction.x -= PLAYER_SPEED;
                *direction = PlayerDirection::Left;
            }
            KeyCode::KeyS => {
                change_direction.y -= PLAYER_SPEED;
                *direction = PlayerDirection::Front;
            }
            KeyCode::KeyD => {
                change_direction.x += PLAYER_SPEED;
                *direction = PlayerDirection::Right;
            }
            _ => {} //ignore rest
        }
    }

    let change_direction = if change_direction != Vec2::ZERO {
        change_direction.normalize_or_zero()
    } else {
        Vec2::ZERO
    };

    if change_direction.length() > 0. {
        let min_pos = Vec3::new(
            location.x as f32 * WIN_W - WIN_W / 2. + PLAYER_WIDTH / 2.,
            location.y as f32 * WIN_H - WIN_H / 2. + PLAYER_HEIGHT / 2.,
            pt.translation.z,
        );
        let max_pos = Vec3::new(
            location.x as f32 * WIN_W + WIN_W / 2. - PLAYER_WIDTH / 2.,
            location.y as f32 * WIN_H + WIN_H / 2. - PLAYER_HEIGHT / 2.,
            pt.translation.z,
        );

        // Update position with bounds checking
        let new_pos = (pt.translation + Vec3::new(change_direction.x, 0., 0.)).clamp(min_pos, max_pos);
        if !collision_detection(&collision_query, new_pos) {
            pt.translation = new_pos;
        }

        let new_pos = (pt.translation + Vec3::new(0., change_direction.y, 0.)).clamp(min_pos, max_pos);
        if !collision_detection(&collision_query, new_pos) {
            pt.translation = new_pos;
        }
    }
}


pub fn animate_player(
    time: Res<Time>,
    mut player: Query<(
        &mut Handle<Image>,
        &mut TextureAtlas,
        &mut AnimationTimer,
        &AnimationFrameCount,
        &PlayerDirection,
        &InputStack
    )>,
) {
    let (_texture_handle, mut texture_atlas, mut timer, _frame_count, direction, input_stack) = player.single_mut();
    timer.set_duration(Duration::from_secs_f32(FISHING_ANIM_TIME));
    //if stack isnt empt player is moving
    let is_moving = input_stack.stack.len() > 0;

    let dir_add = match *direction {
        PlayerDirection::Front => {
            if is_moving { 4 } else { 0 }
        }
        PlayerDirection::Back => {
            if is_moving { 12 } else { 2 }
        }
        PlayerDirection::Left => {
            if is_moving { 16 } else { 3 }
        }
        PlayerDirection::Right => {
            if is_moving { 8 } else { 1 }
        }
    };

    //loop frames if moving
    timer.tick(time.delta());
    if is_moving {
        if timer.just_finished() {
            //cycle 4 frames
            texture_atlas.index = ((texture_atlas.index + 1) % 4) + dir_add;
        }
    } else {
        //reset to still frame when stopped
        texture_atlas.index = dir_add;
    }
}
