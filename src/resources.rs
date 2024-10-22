use bevy::prelude::*;


pub const TITLE: &str = "movement";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const ANIM_TIME: f32 = 0.125; // 8 fps
pub const FISHING_ANIM_TIME: f32 = 0.25; // 4 frames per second for fishing animation

pub const FISHINGROOMX: f32 = 8960.;
pub const FISHINGROOMY: f32 = 3600.;

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
pub struct FishingButton;

#[derive(Component)]
pub struct ShopingButton;

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

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FishingMode {
    #[default]
    Overworld,
    Fishing
}
impl FishingMode{
    pub fn next(&self) -> Self {
        match self {
            FishingMode::Overworld => FishingMode::Fishing,
            FishingMode::Fishing => FishingMode::Overworld,
        }
    }
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShopingMode {
    #[default]
    Overworld,
    Shop
}
impl ShopingMode{
    pub fn next(&self) -> Self {
        match self {
            ShopingMode::Overworld => ShopingMode::Shop,
            ShopingMode::Shop => ShopingMode::Overworld,
        }
    }
}

#[derive(Resource)]
pub struct PlayerReturnPos {
    pub player_save_x: f32,
    pub player_save_y: f32, 
}

//GAMESTATE for switching the game world to the fishing mode








#[derive(Resource)]
pub struct StartFishingAnimation {
    pub active: bool,
    pub button_control_active: bool, 
}

#[derive(Resource)]
pub struct FishingAnimationDuration(pub Timer);


#[derive(Component, PartialEq)]
pub enum TimePeriod{
    Morning,
    Afternoon,
    Night,
}

#[derive(Resource)]
pub struct GameDayTimer{
    pub timer: Timer,
    pub time_period: TimePeriod,
}

impl GameDayTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            time_period: TimePeriod::Morning,
        }
    }    
} 

#[derive(Resource)]
pub struct Money {
    pub amount: i32,
}