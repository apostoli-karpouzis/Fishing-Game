use bevy::window::EnabledButtons;
use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

mod camera; 
mod player; 
mod map; 
mod resources;
mod button;
mod gameday;
mod weather;
mod fishingView;

use crate::camera::*;
use crate::player::*;
use crate::map::*;
use crate::resources::*;
use crate::button::*;
use crate::gameday::*;
use crate::weather::*;
use crate::fishingView::*;

const OLD_TILE_SIZE: f32 = 64.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .insert_resource(StartFishingAnimation { active: false, button_control_active: true })
        .insert_resource(FishingAnimationDuration(Timer::from_seconds(2.0, TimerMode::Once)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_state::<Weather>()
        .init_state::<FishingMode>()
        .init_resource::<WeatherState>()
        .add_systems(Startup, (setup, spawn_weather_tint_overlay))

        // Run the game timer
        .add_systems(Update, run_game_timer)

        // Handle transitions when entering and exiting FishingMode
        .add_systems(OnEnter(FishingMode::Fishing), fishing_transition)
        .add_systems(OnExit(FishingMode::Fishing), overworld_transition)

        // Run the button system in both FishingMode and Overworld
        .add_systems(Update, fishing_button_system)

        // Overworld systems (player movement, animations)
        .add_systems(Update, move_player.run_if(run_if_in_overworld))
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player).run_if(run_if_in_overworld))
        .add_systems(Update, screen_edge_collision.after(move_player))

        // FishingMode systems (power bar and rod rotation)
        .add_systems(Update, power_bar_cast.run_if(run_if_in_fishing))
        .add_systems(Update, rod_rotate.run_if(run_if_in_fishing))

        // Weather updates
        .add_systems(Update, update_weather)
        .add_systems(Update, update_weather_tint.after(update_weather))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        Animation::new()
    ));

    commands.insert_resource(PlayerReturnPos {player_save_x: 0., player_save_y: 0.});
    //GRASS CODE V
    
    //let bg_texture_handle = asset_server.load("test_bg.png");
    let grass_sheet_handle = asset_server.load("ground_sheet.png");
    let grass_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(OLD_TILE_SIZE as u32), 6, 5, None, None);

    let grass_layout_len = grass_layout.textures.len();
    println!("grasslayout.len {}", grass_layout_len);
    let grass_layout_handle = texture_atlases.add(grass_layout);

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - OLD_TILE_SIZE / 2.;
    let y_bound = WIN_H / 2. - OLD_TILE_SIZE / 2.;
    println!("window w {}", (-WIN_H));

    let mut j = 0.;
    while (j as f32) * OLD_TILE_SIZE - y_bound < WIN_H * 2. {
        //println!("rinning j");
        let mut i = 0;
        let mut t = Vec3::new(-x_bound, (OLD_TILE_SIZE * j) + (-y_bound), 0.);
        println!("spawning at {}", (OLD_TILE_SIZE * j) + y_bound);
        while (i as f32) * OLD_TILE_SIZE < WIN_W {
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
            println!("{}", t);
        }
        j += 1.0;
    }
    // ^ END OF GRASS CODE


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

    // MAP
    let map: Map = Map {
        width: 4,
        height: 4
    };

    //PLAYER

    let player_sheet_handle = asset_server.load("characters/full-spritesheet-64x128-256x640.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 5, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 

    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (OLD_TILE_SIZE * 1.5), 900.),
            ..default()
        },
        TextureAtlas {
            layout: player_layout_handle,
            index: 0,
        },
        AnimationTimer::new(ANIM_TIME),  // Use the constructor
        AnimationFrameCount(player_layout_len), // Use the public field
        //Velocity::new(),
        Player,
        InputStack::default(),
        PlayerDirection::Back, // Default direction facing back
        Location {
            map: map,
            x: 0,
            y: 0
        },
        Animation::new()
    ));
    //tree collision hold
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
    
    //spawn_button(&mut commands, asset_server);
    //spawn_button(&mut commands, asset_server);

    //Time of day timer
    commands.insert_resource(
        GameDayTimer::new(30.),
    );


    //let fishing_sheet_handle = asset_server.load("fishingView.png");

    //let grass_layout_len = grass_layout.textures.len();
    
    let fishing_sheet_handle: Handle<Image> = asset_server.load("fishingStuff/fishingView.png");
    //let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 

    commands.spawn((
        SpriteBundle {
            texture: fishing_sheet_handle.clone(),
                        sprite: Sprite {
                        ..default()
                    },
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX, FISHINGROOMY, 900.),
                ..default()
            },
            ..default()
        },
        
    ));
    
    //powerbar view
    let bar_sheet_handle = asset_server.load("fishingStuff/powerBar.png");
    commands.spawn((
        SpriteBundle {
            texture: bar_sheet_handle.clone(),
                        sprite: Sprite{
                        ..default() 
                        },
            //where do I put it
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX+575., FISHINGROOMY-308., 899.),
                ..default()
            },
            ..default()
        },
        Bar,
        Power {
            meter: 0,
            released: false,
        },
    ));

    let bar_sheet_handle = asset_server.load("fishingStuff/backFishingSprite.png");
    commands.spawn((
        SpriteBundle {
            texture: bar_sheet_handle.clone(),
                        sprite: Sprite{
                        ..default() 
                        },
            //where do I put it
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX-100., FISHINGROOMY-(WIN_H/2.)+50., 901.),
                ..default()
            },
            ..default()
        },
    ));

    let bar_sheet_handle = asset_server.load("fishingStuff/fishingRod.png");
    commands.spawn((
        SpriteBundle {
            texture: bar_sheet_handle.clone(),
                        sprite: Sprite{
                        ..default() 
                        },
            //where do I put it
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX-90., FISHINGROOMY-(WIN_H/2.)+100., 900.),
                ..default()
            },
            ..default()
        },
        Rotatable, 
        RotationObj{
            rot: 0.,
        }
    ));
    
    spawn_fishing_button(&mut commands, asset_server);
}
