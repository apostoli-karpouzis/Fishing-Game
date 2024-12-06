use rand::Rng;
use crate::fishing_zone::*;
use crate::player::*;
use crate::window::*;
use bevy::prelude::*;

pub const TILE_SIZE: f32 = 16.;
const OLD_TILE_SIZE: f32 = 64.;

pub const MAP_WIDTH: usize = 5;
pub const MAP_HEIGHT: usize = 5;

pub const GRID_COLUMNS: usize = (WIN_W / TILE_SIZE) as usize;
pub const GRID_ROWS: usize = (WIN_H / TILE_SIZE) as usize;

const MAP_TRANSITION_TIME: f32 = 1.5;

#[derive(Component)]
pub struct Location {
    pub x: usize,
    pub y: usize,
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

    pub const fn get_area_center(x: i32, y: i32) -> Vec2 {
        Vec2::new((x * WIN_W as i32) as f32, (y * WIN_H as i32) as f32)
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

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    pub id: &'static str,
    pub texture: &'static str,
    pub interactable: bool,
    pub hitbox: Vec2,
}

impl Tile {
    pub const fn new(id: &'static str, texture: &'static str, interactable: bool, hitbox: Vec2) -> Self {
        Self { id, texture, interactable, hitbox }
    }
    
    pub const EMPTY: Tile = Tile::new("", "", false, HITBOX_NO_COLLIDE);
    pub const WATER: Tile = Tile::new("water", "tiles/water.png", true, HITBOX_FULL_TILE);
    pub const WATER2: Tile = Tile::new("water2", "tiles/water.png", true, HITBOX_FULL_TILE);
    pub const WATEROCEAN: Tile = Tile::new("water_ocean", "tiles/water.png", true, HITBOX_FULL_TILE);
    pub const TREE: Tile = Tile::new("tree", "tiles/tree.png", false, Vec2::new(50., 80.));
    pub const SHOP: Tile = Tile::new("shop", "tiles/shop.png", true, Vec2::new(64., 64.));
    pub const GOLDLINE: Tile = Tile::new("gold_line", "lines/goldenline.png", true, Vec2::new(75.,75.));
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

fn spawn_tile(commands: &mut Commands, asset_server: &AssetServer, tile: &Tile, x: f32, y: f32, z: f32) {
    let texture_handle = asset_server.load(tile.texture);

    let entity = commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                translation: Vec3::new(x, y, z),
                ..default()
            },
            ..default()
        },
        *tile,
    )).id();

    if tile.hitbox != HITBOX_NO_COLLIDE {
        commands.entity(entity).insert(Collision);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
) {
    // MAP
    let map: Map = Map {
        areas: vec![vec![Area {
            zone: FishingZone {
                current: Vec3::new(-10.0, 0., 0.)
            },
            layout: [[&Tile::EMPTY; GRID_ROWS]; GRID_COLUMNS],
            objects: Vec::new()
        }; MAP_HEIGHT]; MAP_WIDTH],
        width: MAP_WIDTH,
        height: MAP_HEIGHT
    };

    for area_x in 0..map.width {
        let start_x = area_x as f32 * WIN_W + TILE_SIZE / 2. - WIN_W / 2.;
        let center_x = area_x as f32 * WIN_W;

        for area_y in 0..map.height {
            let area = &map.areas[area_x][area_y];
            let start_y = area_y as f32 * WIN_H + TILE_SIZE / 2. - WIN_H / 2.;
            let center_y = area_y as f32 * WIN_H;
            
            // Spawn tiles
            for tile_x in 0..GRID_COLUMNS {
                let x = start_x + tile_x as f32 * TILE_SIZE;

                for tile_y in 0..GRID_ROWS {
                    let y = start_y + tile_y as f32 * TILE_SIZE;

                    let tile = area.layout[tile_x][tile_y];
                    spawn_tile(&mut commands, &asset_server, tile, x, y, 900.);
                }
            }

            // Spawn objects
            for object in area.objects.iter() {
                let x = center_x + object.position.x;
                let y = center_y + object.position.y;

                spawn_tile(&mut commands, &asset_server, object.tile, x, y, 901.);
            }
        }
    }

    commands.insert_resource(map);

    //GRASS CODE V
    
    //let bg_texture_handle = asset_server.load("map/test_bg.png");
    let grass_sheet_handle = asset_server.load("map/ground_sheet.png");
    let grass_layout = TextureAtlasLayout::from_grid(UVec2::splat(OLD_TILE_SIZE as u32), 6, 5, None, None);

    let grass_layout_len = grass_layout.textures.len();
    //println!("grasslayout.len {}", grass_layout_len);
    let grass_layout_handle = texture_atlases.add(grass_layout);

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - OLD_TILE_SIZE / 2.;
    let y_bound = WIN_H / 2. - OLD_TILE_SIZE / 2.;
    //println!("window w {}", (-WIN_H));

    let mut j = 0.;
    while (j as f32) * OLD_TILE_SIZE - y_bound < WIN_H * 4.5 {
        //println!("rinning j");
        let mut i = 0;
        let mut t = Vec3::new(-x_bound, (OLD_TILE_SIZE * j) + (-y_bound), 0.);
        //println!("spawning at {}", (OLD_TILE_SIZE * j) + y_bound);
        while (i as f32) * OLD_TILE_SIZE < WIN_W * 8.75 {
            //println!("rinning i");
            //IF THE SPRITE SHEET FOR BACKGROUND IS MADE LARGER, THIS NEEDS TO GROW
            let mut random_index = rng.gen_range(0..29);
            if random_index % 2 != 0 {
                random_index -= 1;
            }
            commands.spawn((
                SpriteBundle {
                    texture: grass_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    //rng.gen_range
                    //implement random function.
                    index: random_index,
                    layout: grass_layout_handle.clone(),
                }
            ));
            //second time
            t += Vec3::new(OLD_TILE_SIZE, 0., 0.);
            commands.spawn((
                SpriteBundle {
                    texture: grass_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    //rng.gen_range
                    //implement random function.
                    index: (random_index + 1),
                    layout: grass_layout_handle.clone(),
                }
            ));
            //
            //do this twice uhhhhhh....

            i += 1;
            t += Vec3::new(OLD_TILE_SIZE, 0., 0.);
            //println!("{}", t);
        }
        j += 1.0;
    }
    // ^ END OF GRASS CODE

    // After the grass spawning loop

let sand_sheet_handle: Handle<Image> = asset_server.load("tiles/sand.png");
let shore_sheet_handle: Handle<Image> = asset_server.load("tiles/shore.png");
let shore_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 3, 3, None, None);
let shore_layout_handle = texture_atlases.add(shore_layout);

let beach_width = WIN_W * 0.5;
let grass_end = WIN_W * 4.5;
let beach_start = grass_end;

let mut j = 0.;
while (j as f32) * OLD_TILE_SIZE - y_bound < WIN_H * 5.5 {
    let mut i = 0.;
    let mut t = Vec3::new(beach_start - x_bound, (OLD_TILE_SIZE * j) + (-y_bound), 1.);
    
    while (i as f32) * OLD_TILE_SIZE < beach_width {
        if i <= 1.{
            // Spawn sand
            commands.spawn((
            SpriteBundle {
                texture: sand_sheet_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: shore_layout_handle.clone(),
            },
            ));
        }else if i == 2. {  // This will be the middle column of the beach
            commands.spawn((
                SpriteBundle {
                    texture: shore_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: 3,
                    layout: shore_layout_handle.clone(),
                },
                Collision,
                Tile::WATEROCEAN,
            ));
        } else if i >= 3. {  // This will be the rightmost column of the beach
            commands.spawn((
                SpriteBundle {
                    texture: shore_sheet_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    index: 4,
                    layout: shore_layout_handle.clone(),
                },
                Collision,
                Tile::WATEROCEAN,
            ));
        }

        i += 1.;
        t += Vec3::new(OLD_TILE_SIZE, 0., 0.);
    }
    j += 1.;
}
    
    
    //start of water code
    let water_sheet_handle = asset_server.load("tiles/water.png");
    for y in -10..0 {
        for x in -10..0{
            commands.spawn((
                SpriteBundle {
                    texture: water_sheet_handle.clone(),
                        sprite: Sprite {
                        custom_size: Some(Vec2::new(16.,16.)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x as f32 * 16.,  y as f32 * 16., 900.),
                        ..default()
                    },
                    ..default()
                },
                Tile::WATER,
                Collision,
            ));
        }
    }
    //second pond
    for y in -10..0 {
        for x in -10..0{
            commands.spawn((
                SpriteBundle {
                    texture: water_sheet_handle.clone(),
                        sprite: Sprite {
                        custom_size: Some(Vec2::new(16.,16.)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x as f32 * 16. + 400.,  y as f32 * 16. + 100., 900.),
                        ..default()
                    },
                    ..default()
                },
                Tile::WATER2,
                Collision,
            ));
        }
    }

    let gold_line_sheet_handle: Handle<Image> = asset_server.load("lines/goldenline.png");
    commands.spawn((
        SpriteBundle{
            texture: gold_line_sheet_handle.clone(),
            sprite: Sprite{
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            }, transform: Transform{
                translation: Vec3::new(200., 200., 902.),
                ..default()
            },
            ..default()
        },
        Tile::GOLDLINE,
        Collision,
        Forageable,
    ));

    let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 
    
    commands.spawn((
        SpriteBundle {
            texture: tree_sheet_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100., 100., 900.),
                ..default()
            },
            ..default()
        },
        Tile::TREE,
        Collision,
    ));
    commands.spawn((
        SpriteBundle {
            texture: tree_sheet_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(300., 300., 900.),
                ..default()
            },
            ..default()
        },
        Tile::TREE,
        Collision,
    ));

    let mut i: f32 = 0.;
    while i <= 35.{
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
    map: Res<Map>,
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
            && map_location.x + 1 < map.width
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
            && map_location.y + 1 < map.height
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
