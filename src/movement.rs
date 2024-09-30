use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "movement";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 100.;
const ACCEL_RATE: f32 = 500.;

const TILE_SIZE: u32 = 100;

const LEVEL_LEN: f32 = 5000.;

const ANIM_TIME: f32 = 0.125; // 8 fps

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
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
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bg_texture_handle = asset_server.load("test_bg.png");

    let mut x_offset = 0.;
    while x_offset < LEVEL_LEN {
        commands
            .spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform::from_xyz(x_offset, 0., 0.),
                ..default()
            })
            .insert(Background);

        x_offset += WIN_W;
    }

    let player_sheet_handle = asset_server.load("characters/full-spritesheet-64x128-256x640.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 5, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);

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
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerDirection), With<Player>>,
) {
    let (mut pt, mut pv, mut direction) = player.single_mut();
    let mut deltav = Vec2::splat(0.);

    // left
    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        *direction = PlayerDirection::Left;
    }

    // right
    else if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
        *direction = PlayerDirection::Right;
    }

    // up 
    else if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
        *direction = PlayerDirection::Back;
    }

    // down
    else if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
        *direction = PlayerDirection::Front;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } /*else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    }*/ else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    // update position with bounds checking
    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_LEN - (WIN_W / 2. + (TILE_SIZE as f32) / 2.)
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5)
        && new_pos.y <= WIN_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }
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
    let (v, mut texture_handle, mut texture_atlas, mut timer, frame_count, direction) = player.single_mut();

    // switch sprite sheets based on direction

    let mut dir_add: usize = 4;
    match *direction {
        PlayerDirection::Front => {
            // *texture_handle = asset_server.load("characters/angler-front-moving.png");
            dir_add = 4;
        }
        PlayerDirection::Back => {
            // *texture_handle = asset_server.load("characters/angler-back-moving.png");
            dir_add = 12;
        }
        PlayerDirection::Left => {
            // *texture_handle = asset_server.load("characters/angler-left-moving.png");
            dir_add = 16;
        }
        PlayerDirection::Right => {
            // *texture_handle = asset_server.load("characters/angler-right-moving.png");
            dir_add = 8;
        }
    }

    if v.velocity.cmpne(Vec2::ZERO).any() {
        // play correct animation based on direction
        timer.tick(time.delta());
        if timer.just_finished() {
           // texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
           texture_atlas.index = ((texture_atlas.index + 1) % 4) + dir_add;
        }
    } else {
        // when stopped switch to stills
        match *direction {
            PlayerDirection::Front => {
                // *texture_handle = asset_server.load("characters/angler-front-still.png");
                texture_atlas.index = 0;
            }
            PlayerDirection::Back => {
                // *texture_handle = asset_server.load("characters/angler-back-still.png");
                texture_atlas.index = 2;
            }
            PlayerDirection::Left => {
                // *texture_handle = asset_server.load("characters/angler-left-still.png");
                texture_atlas.index = 3;
            }
            PlayerDirection::Right => {
                // *texture_handle = asset_server.load("characters/angler-right-still.png");
                texture_atlas.index = 1;
            }
        }
    }
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
