use bevy::prelude::*;
use std::convert::From;

pub const TITLE: &str = "movement";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const PLAYER_SPEED: f32 = 200.;
pub const ACCEL_RATE: f32 = 400.;

pub const TILE_SIZE_GRASS: u32 = 64;
pub const TILE_SIZE: u32 = 100;

//pub const LEVEL_LEN_W: f32 = 1280.;
//pub const LEVEL_LEN_H: f32 = 720.;

pub const ANIM_TIME: f32 = 0.125; // 8 fps

pub const CAM_SPEED: f32 = 0.005;

pub const PLAYER_WIDTH: f32 = 64.;
pub const PLAYER_HEIGHT: f32 = 128.;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl AnimationTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(pub usize);

#[derive(Component, Deref, DerefMut)]
pub struct CameraSpeed {
    pub timer: Timer,
}

impl CameraSpeed {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

#[derive(Default, Debug, Clone, States, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    CamStill,
    CamMove,
}

// CAMERA MOVE STATE
#[derive(Component)]
pub struct GrassTile;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

#[derive(Resource)]
pub struct Location {
    pub i: i32,
    pub j: i32,
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

#[derive(Component)]
pub struct Collision;

#[derive(Resource, Default)]
pub enum CameraDirection {
    North,
    South,
    West,
    East,
    #[default]
    None,
}