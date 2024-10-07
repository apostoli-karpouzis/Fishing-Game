use bevy::prelude::*;
use crate::player::*;
use crate::resources::*;

#[derive(Component)]
pub struct Collision;

pub fn collision_detection(
    collision_query: &Query<&Transform, (With<Collision>, Without<Player>, Without<GrassTile>)>,
    player_pos: Vec3,
) -> bool {
    
    for object in collision_query.iter() {
        if  player_pos.y - PLAYER_HEIGHT/2. > object.translation.y + (TILE_SIZE as f32)/2. 
            || player_pos.y + PLAYER_HEIGHT/2. < object.translation.y - (TILE_SIZE as f32)/2. 
            || player_pos.x + PLAYER_WIDTH/2. < object.translation.x - (TILE_SIZE as f32)/2. 
            || player_pos.x - PLAYER_WIDTH/2. > object.translation.x + (TILE_SIZE as f32)/2.
        {
            continue;
        }
        return false;
    }

    return true;
}