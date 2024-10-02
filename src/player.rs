use bevy::prelude::*;
use super::collision::*;
use super::resources::*;
use std::time::Duration;

pub const PLAYER_WIDTH: f32 = 64.;
pub const PLAYER_HEIGHT: f32 = 128.;

const PLAYER_SPEED: f32 = 200.;
const ACCEL_RATE: f32 = 400.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

#[derive(Component, PartialEq)]
pub enum PlayerDirection {
    Front,
    Back,
    Left,
    Right,
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerDirection), With<Player>>,
    collision_query: Query<&Transform, (With<Collision>, Without<Player>, Without<GrassTile>)>,
) {

    let (mut pt, mut pv, mut direction) = player.single_mut();
    let mut deltav = Vec2::splat(0.);

    // left
    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        *direction = PlayerDirection::Left;
    }

    // right
    else if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
        *direction = PlayerDirection::Right;
    }

    // up
    else if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
        *direction = PlayerDirection::Back;
    }

    // down
    else if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
        *direction = PlayerDirection::Front;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    }/* else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    }*/ else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    // update position with bounds checking

    //update lower bounds to be + tile size
    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    
    if collision_detection(&collision_query, new_pos){
        pt.translation = new_pos;
    }
    // if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 4.
    //     && new_pos.x <= 1280. - (WIN_W / 2. + (TILE_SIZE as f32) / 4.)
    // {
    //     pt.translation = new_pos;
    // }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    
    if collision_detection(&collision_query, new_pos){
        pt.translation = new_pos;
    }
    // if new_pos.y >= -(WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5)
    //     && new_pos.y <= WIN_H / 2. - (TILE_SIZE as f32) / 2.
    // {
    //     pt.translation = new_pos;
    // }
}

pub fn animate_player(
    time: Res<Time>,
    _asset_server: Res<AssetServer>,
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,
    mut button_query: Query<&mut Visibility, With<Button>>,
    mut player: Query<(
        &Velocity,
        &mut Handle<Image>,
        &mut TextureAtlas,
        &mut AnimationTimer,
        &AnimationFrameCount,
        &PlayerDirection,
    )>,
) {
    //texture handle and frame count not used
    let (v, _texture_handle, mut texture_atlas, mut timer, _frame_count, direction) = player.single_mut();
        
    timer.set_duration(Duration::from_secs_f32(FISHING_ANIM_TIME));
    
    if start_fishing_animation.active {

        // *texture_handle = asset_server.load("characters/angler-back-fishing.png");

        timer.tick(time.delta());

        if timer.just_finished() {
            // *texture_handle = asset_server.load("characters/angler-back-moving.png");
            // texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
        
        fishing_timer.0.tick(time.delta());
        if !fishing_timer.0.finished() {
            return; 
        }
        
        
        start_fishing_animation.active = false;
        start_fishing_animation.button_control_active = true;
        // *texture_handle = asset_server.load("characters/angler-back-moving.png");

        //fix this
        for mut visibility in &mut button_query {
            *visibility = Visibility::Visible;
        }
    }


    // switch sprite sheets based on direction
    let dir_add;
    match *direction {
        PlayerDirection::Front => {
            dir_add = 4;
        }
        PlayerDirection::Back => {
            dir_add = 12;
        }
        PlayerDirection::Left => {
            dir_add = 16;
        }
        PlayerDirection::Right => {
            dir_add = 8;
        }
    }

    if v.velocity.cmpne(Vec2::ZERO).any() {
        // play correct animation based on direction
        timer.tick(time.delta());
        if timer.just_finished() {
            texture_atlas.index = ((texture_atlas.index + 1) % 4) + dir_add;
        }
    } else {
        // when stopped switch to stills
        match *direction {
            PlayerDirection::Front => {
                texture_atlas.index = 0;
            }
            PlayerDirection::Back => {
                texture_atlas.index = 2;
            }
            PlayerDirection::Left => {
                texture_atlas.index = 3;
            }
            PlayerDirection::Right => {
                texture_atlas.index = 1;
            }
        }
    }
}

