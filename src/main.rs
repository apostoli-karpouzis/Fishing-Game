use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

mod camera; 
mod player; 
mod collision; 
mod resources;
mod button;

use crate::camera::*;
use crate::player::*;
use crate::collision::*;
use crate::resources::*;
use crate::button::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .insert_resource(StartFishingAnimation { active: false, button_control_active: true })
        .insert_resource(FishingAnimationDuration(Timer::from_seconds(2.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system.after(move_player))

        //updating state
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
    commands.spawn((
        Camera2dBundle::default(),
        CameraAnimation::new()
    ));

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
        AnimationTimer::new(ANIM_TIME),  // Use the constructor
        AnimationFrameCount(player_layout_len), // Use the public field
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
                translation: Vec3::new(100., 100., 900.),
                ..default()
            },
            ..default()
        },
        Collision,
    ));

    //adding x and y
    commands.insert_resource(Location { i: 0, j: 0 });
    
    spawn_button(&mut commands, asset_server);
}