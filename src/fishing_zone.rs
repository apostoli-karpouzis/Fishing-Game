use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct FishingZone {
    pub current: Vec3
}

impl FishingZone {
    // fn new(current: Vec3) -> Self {
    //     Self { current }
    // }

    pub const DEFAULT: FishingZone = FishingZone {
        current: Vec3::ZERO
    };
}