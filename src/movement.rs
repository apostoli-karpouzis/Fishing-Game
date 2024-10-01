use bevy::{prelude::*, window::PresentMode};
use rand::Rng;
use std::convert::From;

const TITLE: &str = "movement";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 200.;
const ACCEL_RATE: f32 = 400.;

const TILE_SIZE_GRASS: u32 = 64;
const TILE_SIZE: u32 = 100;

const LEVEL_LEN_W: f32 = 1280.;
const LEVEL_LEN_H: f32 = 720.;

const ANIM_TIME: f32 = 0.125; // 8 fps

const CAM_SPEED: f32 = 0.005;

const PLAYER_WIDTH: f32 = 64.;
const PLAYER_HEIGHT: f32 = 128.;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component, Deref, DerefMut)]
struct CameraSpeed(Timer);

#[derive(Default, Debug, Clone, States, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    CamStill,
    CamMove,
}
//CAMERA MOVE STATE

#[derive(Component)]
struct GrassTile;

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

#[derive(Resource)]
struct Location {
    i: i32,
    j: i32,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

#[derive(Component, PartialEq)]
enum PlayerDirection {
    Front,
    Back,
    Left,
    Right,
}

#[derive(Component)]
struct Collision;


#[derive(Resource, Default)]
pub enum CameraDirection {
    North,
    South,
    West,
    East,
    #[default]
    None,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .insert_state(GameState::CamStill)
        .add_systems(Startup, setup)
        //updating state
        .add_systems(Update, move_player)
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .add_systems(Update, pan_cam.after(move_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());



    //GRASS CODE V
    //let bg_texture_handle = asset_server.load("test_bg.png");
    let grass_sheet_handle = asset_server.load("ground_sheet.png");
    let grass_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE_GRASS), 6, 5, None, None);

    let grass_layout_len = grass_layout.textures.len();
    println!("grasslayout.len {}", grass_layout_len);
    let grass_layout_handle = texture_atlases.add(grass_layout);

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - (TILE_SIZE_GRASS as f32) / 2.;
    let y_bound = WIN_H / 2. - (TILE_SIZE_GRASS as f32) / 2.;
    println!("window w {}", (-WIN_H));

    let mut j = 0.;
    while (j as f32) * (TILE_SIZE_GRASS as f32) - y_bound < WIN_H * 2. {
        //println!("rinning j");
        let mut i = 0;
        let mut t = Vec3::new(-x_bound, (TILE_SIZE_GRASS as f32 * j) + (-y_bound), 0.);
        println!("spawning at {}", (TILE_SIZE_GRASS as f32 * j) + y_bound);
        while (i as f32) * (TILE_SIZE_GRASS as f32) < WIN_W {
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
                },
                GrassTile,
            ));
            //second time
            t += Vec3::new(TILE_SIZE_GRASS as f32, 0., 0.);
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
                },
                GrassTile,
            ));
            //
            //do this twice uhhhhhh....

            i += 1;
            t += Vec3::new(TILE_SIZE_GRASS as f32, 0., 0.);
            println!("{}", t);
        }
        j += 1.0;
    }
    // ^ END OF GRASS CODE


    //PLAYER

    let player_sheet_handle = asset_server.load("characters/full-spritesheet-64x128-256x640.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 5, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    let tree_sheet_handle: Handle<Image> = asset_server.load("tree.png"); 

    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.),
            ..default()
        },
        TextureAtlas {
            layout: player_layout_handle,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Player,
        PlayerDirection::Back, // Default direction facing back
    ));
    //tree collision hold
    commands.spawn((
        SpriteBundle {
            texture: tree_sheet_handle,
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(69., 200., 900.),
                ..default()
            },
            ..default()
        },
        Collision,
    ));


    //cam speed timer addition
    commands.spawn(CameraSpeed(Timer::from_seconds(
        CAM_SPEED,
        TimerMode::Repeating,
    )));
    //adding x and y
    commands.insert_resource(Location { i: 0, j: 0 });
    commands.init_resource::<CameraDirection>();

    //making cam direction
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerDirection), With<Player>>,
    collision_query: Query<(&Transform), (With<Collision>, Without<Player>, Without<GrassTile>)>,
) {
    
    let (mut pt, mut pv, mut direction) = player.single_mut();
    let mut deltav = Vec2::splat(0.);

    // left
    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        *direction = PlayerDirection::Left;
    }

    // right
    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
        *direction = PlayerDirection::Right;
    }

    // up
    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
        *direction = PlayerDirection::Back;
    }

    // down
    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
        *direction = PlayerDirection::Front;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    }/* else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    }*/ else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    // update position with bounds checking

    //update lower bounds to be + tile size
    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    
    if collision_detection(&collision_query, new_pos){
        pt.translation = new_pos;
    }
    // if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 4.
    //     && new_pos.x <= 1280. - (WIN_W / 2. + (TILE_SIZE as f32) / 4.)
    // {
    //     pt.translation = new_pos;
    // }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    
    if collision_detection(&collision_query, new_pos){
        pt.translation = new_pos;
    }
    // if new_pos.y >= -(WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5)
    //     && new_pos.y <= WIN_H / 2. - (TILE_SIZE as f32) / 2.
    // {
    //     pt.translation = new_pos;
    // }
}


fn collision_detection(
    collision_query: &Query<(&Transform), (With<Collision>, Without<Player>, Without<GrassTile>)>,
    player_pos: Vec3,
) -> bool {
    
    for object in collision_query.iter() {
        if  player_pos.y - PLAYER_HEIGHT/2. > object.translation.y + (TILE_SIZE as f32)/2. ||
            player_pos.y + PLAYER_HEIGHT/2. < object.translation.y - (TILE_SIZE as f32)/2. || 
            player_pos.x + PLAYER_WIDTH/2. < object.translation.x - (TILE_SIZE as f32)/2.  ||
            player_pos.x - PLAYER_WIDTH/2. > object.translation.x + (TILE_SIZE as f32)/2.
        {
            continue;
        }
        return false;
    }

    return true;
}

fn animate_player(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut player: Query<(
        &Velocity,
        &mut Handle<Image>,
        &mut TextureAtlas,
        &mut AnimationTimer,
        &AnimationFrameCount,
        &PlayerDirection,
    )>,
) {
    let (v, mut texture_handle, mut texture_atlas, mut timer, frame_count, direction) =
        player.single_mut();

    // switch sprite sheets based on direction
    let mut dir_add: usize = 4;
    match *direction {
        PlayerDirection::Front => {
            //*texture_handle = asset_server.load("characters/angler-front-moving.png");
            dir_add = 4;
        }
        PlayerDirection::Back => {
            //*texture_handle = asset_server.load("characters/angler-back-moving.png");
            dir_add = 12;
        }
        PlayerDirection::Left => {
            //*texture_handle = asset_server.load("characters/angler-left-moving.png");
            dir_add = 16;
        }
        PlayerDirection::Right => {
            //*texture_handle = asset_server.load("characters/angler-right-moving.png");
            dir_add = 8;
        }
    }

    if v.velocity.cmpne(Vec2::ZERO).any() {
        // play correct animation based on direction
        timer.tick(time.delta());
        if timer.just_finished() {
            texture_atlas.index = ((texture_atlas.index + 1) % 4) + dir_add;
        }
    } else {
        // when stopped switch to stills
        match *direction {
            PlayerDirection::Front => {
                //*texture_handle = asset_server.load("characters/angler-front-still.png");
                texture_atlas.index = 0;
            }
            PlayerDirection::Back => {
                //*texture_handle = asset_server.load("characters/angler-back-still.png");
                texture_atlas.index = 2;
            }
            PlayerDirection::Left => {
                //*texture_handle = asset_server.load("characters/angler-left-still.png");
                texture_atlas.index = 3;
            }
            PlayerDirection::Right => {
                //*texture_handle = asset_server.load("characters/angler-right-still.png");
                texture_atlas.index = 1;
            }
        }
    }
}

// if you have multiple states that must be set correctly,
// don't forget to manage them all

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    game_state: Res<State<GameState>>,
    mut new_state: ResMut<NextState<GameState>>,
    mut grid_loc: ResMut<Location>,
    mut dir: ResMut<CameraDirection>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();
    let new_pos_x = pt.translation.x;
    let new_pos_y = pt.translation.y;
    let grid_hold = grid_loc.i;
    let grid_hold_y = grid_loc.j;
    //ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
    if new_pos_x <= (WIN_W as f32 * grid_hold as f32) - (WIN_W / 2.) + ((TILE_SIZE as f32) / 4.) {
        //println!("hit the left {}", (grid_hold * 1280));

        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::West;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
    if new_pos_x >= (WIN_W as f32 * grid_hold as f32) + WIN_W - (WIN_W / 2. + (TILE_SIZE as f32) / 4.) {
        //println!("hit the right {}", (WIN_W - (WIN_W / 2. + (TILE_SIZE as f32) / 4.)));
        //println!("{:?}", game_state.get());
        if game_state.get() == &GameState::CamStill {
            
            *dir = CameraDirection::East;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }

        //switching move states
    }

    if new_pos_y <= (WIN_H as f32 * grid_hold_y as f32) - (WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5) {
        println!("hit the bottom");
        println!("{}", (WIN_H as f32 * grid_hold_y as f32) - (WIN_H / 2.) + ((TILE_SIZE as f32) * 0.5));
        println!("{}", new_pos_y);
        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::South;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
    if new_pos_y > (grid_hold_y as f32 * WIN_H as f32) + WIN_H / 2. - ((TILE_SIZE as f32) * 0.5) {
        println!("hit the top");
        
        if game_state.get() == &GameState::CamStill {
            *dir = CameraDirection::North;
            match game_state.get() {
                //ok were getting somewhere
                GameState::CamStill => new_state.set(GameState::CamMove),
                GameState::CamMove => new_state.set(GameState::CamMove),
            }
        }
    }
}

fn pan_cam(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    game_state: Res<State<GameState>>,
    mut new_state: ResMut<NextState<GameState>>,
    mut time_stuff: Query<&mut CameraSpeed>,
    mut grid_loc: ResMut<Location>,
    mut dir: ResMut<CameraDirection>,
) {
    let grid_hold = grid_loc.i;
    let grid_hold_y = grid_loc.j;

    let mut timer = time_stuff.single_mut();
    //print!("function wokring");

    let mut ct = camera.single_mut();
    //println!("{:?}", game_state.get());
    if game_state.get() == &GameState::CamMove {
        match *dir {
            CameraDirection::North => {
                println!("going North");
                if ct.translation.y <= (720. + (720. * grid_hold_y as f32)){
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        //println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.y += 9.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.j = grid_loc.j + 1;
                    match game_state.get() {
                        
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                    println!("ending");
                }
            }
            CameraDirection::South => {
                if ct.translation.y >= (-720. + (720. * grid_hold_y as f32)) {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.y -= 9.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.j = grid_loc.j - 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                    println!("ending");
                }
            }
            CameraDirection::West => {
                if ct.translation.x >= (-1280. + (TILE_SIZE as f32) / 4.) + (1280 as f32 * grid_hold as f32){
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        //println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.x -= 16.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.i = grid_loc.i - 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                }
            }
            CameraDirection::East => {
                println!("going East");
                if ct.translation.x < 1280. + (1280 as f32 * grid_hold as f32) {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        println!("{}", grid_loc.i);
                        println!("timer finished");
                        ct.translation.x += 16.0;
                    }
                    //update start timer, move camera
                } else {
                    *dir = CameraDirection::None;
                    grid_loc.i = grid_loc.i + 1;
                    match game_state.get() {
                        //ok were getting somewhere
                        GameState::CamStill => new_state.set(GameState::CamStill),
                        GameState::CamMove => new_state.set(GameState::CamStill),
                    }
                }
            }
            CameraDirection::None => {
                println!("still");
            }
        }
    }
}
