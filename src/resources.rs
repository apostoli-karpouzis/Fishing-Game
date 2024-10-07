use bevy::prelude::*;

pub const TITLE: &str = "movement";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

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

#[derive(Component)]
pub struct Animation {
    pub start_time: f32,
    pub duration: f32,
    pub start_position: Vec3,
    pub motion: Vec3,
}

impl Animation {
    pub fn new() -> Self {
        Self {
            start_time: 0.,
            duration: 0.,
            start_position: Vec3::default(),
            motion: Vec3::default(),
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Normal,
    MapTransition
}

#[derive(Resource)]
pub struct StartFishingAnimation {
    pub active: bool,
    pub button_control_active: bool, 
}

#[derive(Resource)]
pub struct FishingAnimationDuration(pub Timer);

