use bevy::prelude::*;
use crate::player::*;
use crate::resources::*;

pub const TILE_SIZE: f32 = 16.;
pub const GRID_COLUMNS: usize = (WIN_W / TILE_SIZE) as usize;
pub const GRID_ROWS: usize = (WIN_H / TILE_SIZE) as usize;

#[derive(Component)]
pub struct Location {
    pub map: Map,
    pub x: usize,
    pub y: usize
}

pub struct Map {
    pub width: usize,
    pub height: usize
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

pub enum FishingZone {
    None,
    River,
    Lake,
    Pond
}

pub struct Area {
    pub zone: FishingZone,
    pub layout: [[Tile; GRID_COLUMNS]; GRID_ROWS]
}

impl Area {
    pub fn new(zone: FishingZone, layout: [[Tile; GRID_COLUMNS]; GRID_ROWS]) -> Self {
        Self { zone, layout }
    }
}

const HITBOX_NO_COLLIDE: Vec2 = Vec2::new(0., 0.);
const HITBOX_FULL_TILE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, PartialEq)]
pub struct Tile {
    pub id: &'static str,
    pub interactable: bool,
    pub hitbox: Vec2,
}

impl Tile {
    pub const fn new(id: &'static str, interactable: bool, hitbox: Vec2) -> Self {
        Self { id, interactable, hitbox }
    }

    pub const WATER: Tile = Tile::new("water", true, HITBOX_FULL_TILE);
    pub const TREE: Tile = Tile::new("tree", false, Vec2::new(50., 80.));
}

#[derive(Component)]
pub struct Collision;

pub fn collision_detection(
    collision_query: &Query<(&Transform, &Tile), (With<Collision>, Without<Player>)>,
    new_pos: Vec3,
) -> bool {
    for object in collision_query.iter() {
        let (transform, tile) = object;

        if new_pos.y - PLAYER_HEIGHT / 2. > transform.translation.y + tile.hitbox.y / 2. 
            || new_pos.y + PLAYER_HEIGHT / 2. < transform.translation.y - tile.hitbox.y / 2. 
            || new_pos.x + PLAYER_WIDTH / 2. < transform.translation.x - tile.hitbox.x / 2. 
            || new_pos.x - PLAYER_WIDTH / 2. > transform.translation.x + tile.hitbox.x / 2.
        {
            continue;
        }

        if tile.interactable {
            match tile {
                &Tile::WATER => println!("Collided with water"),
                _ => {}
            }
        }

        return true;
    }

    return false;
}