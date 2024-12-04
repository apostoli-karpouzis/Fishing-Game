use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FishingLocal {
    #[default]
    Pond1,
    Pond2,
    Beach
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

#[derive(Resource, Default)]
pub struct PlayerReturnPos {
    pub position: Vec3
}