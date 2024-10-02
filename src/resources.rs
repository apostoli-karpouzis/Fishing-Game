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
pub const FISHING_ANIM_TIME: f32 = 0.25; // 4 frames per second for fishing animation

pub const CAM_SPEED: f32 = 0.005;

pub const PLAYER_WIDTH: f32 = 64.;
pub const PLAYER_HEIGHT: f32 = 128.;

pub const MAP_TRANSITION_TIME: f32 = 2.;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct ButtonVisible(pub bool);

#[derive(Resource)]
pub struct StartFishingAnimation {
    pub active: bool,
    pub button_control_active: bool, 
}

#[derive(Resource)]
pub struct FishingAnimationDuration(pub Timer);

//---------
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

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Normal,
    MapTransition
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

