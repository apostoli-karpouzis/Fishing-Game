use bevy::prelude::*;

pub const TITLE: &str = "movement";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const TILE_SIZE_GRASS: u32 = 64;
pub const TILE_SIZE: u32 = 100;

pub const ANIM_TIME: f32 = 0.125; // 8 fps
pub const FISHING_ANIM_TIME: f32 = 0.25; // 4 frames per second for fishing animation

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

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Normal,
    MapTransition
}

#[derive(Component)]
pub struct GrassTile;

#[derive(Resource)]
pub struct Location {
    pub i: i32,
    pub j: i32,
}

#[derive(Resource)]
pub struct StartFishingAnimation {
    pub active: bool,
    pub button_control_active: bool, 
}

#[derive(Resource)]
pub struct FishingAnimationDuration(pub Timer);