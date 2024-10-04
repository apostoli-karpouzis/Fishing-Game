use bevy::prelude::*;
use crate::player::*;
use crate::resources::*;

#[derive(Component)]
pub struct TileBundle {
    pub collision_size: Vec2,
    pub tile_type: CollisionType, 
}

#[derive(Component)]
pub struct Collision;

#[derive(Component, PartialEq)]
pub enum CollisionType {
    NORMAL,
    WATER,
}




pub const TILE_SIZE: u32 = 64;

pub fn collision_detection(
    collision_query: &Query<(&Transform, &TileBundle), (With<Collision>, Without<Player>, Without<GrassTile>)>,

    player_pos: Vec3,
) -> bool {
    
    for object in collision_query.iter() {
        let (transform, tile) = object;
        if  player_pos.y - PLAYER_HEIGHT/2. > transform.translation.y + tile.collision_size.y/2. 
            || player_pos.y + PLAYER_HEIGHT/2. < transform.translation.y - tile.collision_size.y/2. 
            || player_pos.x + PLAYER_WIDTH/2. < transform.translation.x - tile.collision_size.x/2. 
            || player_pos.x - PLAYER_WIDTH/2. > transform.translation.x + tile.collision_size.x/2.
        {
            continue;
        }
        if tile.tile_type == CollisionType::WATER {
            println!("water!!!");
        }
        return false;
    }
    return true;
}