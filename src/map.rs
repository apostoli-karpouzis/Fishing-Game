use crate::fishing_zone::*;
use crate::player::*;
use crate::window::*;
use bevy::prelude::*;
use bevy::sprite::*;

pub const TILE_SIZE: f32 = 16.;

pub const MAP_WIDTH: usize = 4;
pub const MAP_HEIGHT: usize = 4;

pub const GRID_COLUMNS: usize = (WIN_W / TILE_SIZE) as usize;
pub const GRID_ROWS: usize = (WIN_H / TILE_SIZE) as usize;

const MAP_TRANSITION_TIME: f32 = 1.5;

#[derive(Component)]
pub struct Location {
    pub map: Map,
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn get_current_area(&self) -> &Area {
        return &self.map.areas[self.x][self.y];
    }
}

#[derive(Resource)]
pub struct Map {
    pub areas: Vec<Vec<Area>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(areas: Vec<Vec<Area>>, width: usize, height: usize) -> Self {
        Self {
            areas,
            width,
            height,
        }
    }
}

#[derive(Clone)]
pub struct Area {
    pub zone: FishingZone,
    pub layout: [[&'static Tile; GRID_ROWS]; GRID_COLUMNS],
    pub objects: Vec<Object>,
}

impl Area {
    pub fn new(
        zone: FishingZone,
        layout: [[&'static Tile; GRID_ROWS]; GRID_COLUMNS],
        objects: Vec<Object>,
    ) -> Self {
        Self {
            zone,
            layout,
            objects,
        }
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
        Self {
            id,
            interactable,
            hitbox,
        }
    }

    pub const BOBBER: Tile = Tile::new("bobber", false, Vec2::new(100., 100.));
    pub const WATER: Tile = Tile::new("water", true, HITBOX_FULL_TILE);
    pub const TREE: Tile = Tile::new("tree", false, Vec2::new(50., 80.));
    pub const SHOP: Tile = Tile::new("shopEntrance", true, Vec2::new(256., 180.));
}

#[derive(Clone)]
pub struct Object {
    pub tile: &'static Tile,
    pub position: Vec2,
}

impl Object {
    pub const fn new(tile: &'static Tile, position: Vec2) -> Self {
        Self { tile, position }
    }
}

#[derive(Component)]
pub struct Collision;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapState {
    #[default]
    Normal,
    MapTransition,
}

#[derive(Component)]
pub struct Animation {
    pub start_time: f32,
    pub duration: f32,
    pub start_position: Vec3,
    pub motion: Vec3,
}

#[derive(Component)]
pub struct MapPlugin;



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

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 
    println!("map");
    let mut i: f32 = 0.;
    while i <= 30.{
        commands.spawn((
            SpriteBundle {
                texture: tree_sheet_handle.clone(),
                    sprite: Sprite {
                    custom_size: Some(Vec2::new(100.,100.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-1280./2., (100.*i)-(720./2.), 900.),
                    ..default()
                },
                ..default()
            },
            Tile::TREE,
            Collision,
        ));
        i = i + 1.;
    }
    
}

pub fn screen_edge_collision(
    mut player: Query<(&mut Location, &Transform, &PlayerDirection, &mut Animation), With<Player>>,
    mut camera: Query<(&mut Transform, &mut Animation), (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    state: Res<State<MapState>>,
    mut next_state: ResMut<NextState<MapState>>,
) {
    let (mut map_location, pt, direction, mut player_animation) = player.single_mut();
    let (ct, mut camera_animation) = camera.single_mut();

    if !state.eq(&MapState::Normal) {
        let elapsed: f32 = time.elapsed_seconds() - camera_animation.start_time;

        if state.eq(&MapState::MapTransition) && elapsed >= MAP_TRANSITION_TIME {
            next_state.set(MapState::Normal);
        }

        return;
    }

    // Check for edge collision
    let mut player_offset: Vec3 = Vec3::ZERO;
    let mut camera_offset: Vec3 = Vec3::ZERO;

    if *direction == PlayerDirection::Right {
        if pt.translation.x + PLAYER_WIDTH / 2. >= WIN_W * map_location.x as f32 + WIN_W / 2.
            && map_location.x + 1 < map_location.map.width
        {
            map_location.x += 1;
            player_offset = Vec3::new(PLAYER_WIDTH, 0., 0.);
            camera_offset = Vec3::new(WIN_W, 0., 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Left {
        if pt.translation.x - PLAYER_WIDTH / 2. <= WIN_W * map_location.x as f32 - WIN_W / 2.
            && map_location.x != 0
        {
            map_location.x -= 1;
            player_offset = Vec3::new(-PLAYER_WIDTH, 0., 0.);
            camera_offset = Vec3::new(-WIN_W, 0., 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Back {
        if pt.translation.y + PLAYER_HEIGHT / 2. >= WIN_H * map_location.y as f32 + WIN_H / 2.
            && map_location.y + 1 < map_location.map.height
        {
            map_location.y += 1;
            player_offset = Vec3::new(0., PLAYER_HEIGHT, 0.);
            camera_offset = Vec3::new(0., WIN_H, 0.);
        } else {
            return;
        }
    }

    if *direction == PlayerDirection::Front {
        if pt.translation.y - PLAYER_HEIGHT / 2. <= WIN_H * map_location.y as f32 - WIN_H / 2.
            && map_location.y != 0
        {
            map_location.y -= 1;
            player_offset = Vec3::new(0., -PLAYER_HEIGHT, 0.);
            camera_offset = Vec3::new(0., -WIN_H, 0.);
        } else {
            return;
        }
    }

    // Start map transition
    next_state.set(MapState::MapTransition);

    *player_animation = Animation {
        start_time: time.elapsed_seconds(),
        duration: MAP_TRANSITION_TIME,
        start_position: pt.translation,
        motion: player_offset,
    };

    *camera_animation = Animation {
        start_time: time.elapsed_seconds(),
        duration: MAP_TRANSITION_TIME,
        start_position: ct.translation,
        motion: camera_offset,
    }
}
