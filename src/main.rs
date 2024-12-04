extern crate rand;
use bevy::window::EnabledButtons;
use bevy::time::Timer;
use bevy::prelude::*;
use bevy::window::PresentMode;
use fishing_game::interface::CurrentInterface;
use rand::Rng;

use fishing_game::camera::*;
use fishing_game::inventory::*;
use fishing_game::player::*;
use fishing_game::map::*;
use fishing_game::resources::*;
use fishing_game::button::*;
use fishing_game::gameday::*;
use fishing_game::weather::*;
use fishing_game::fishing_view::*;
use fishing_game::fishing_zone::*;
use fishing_game::shop::*;
use fishing_game::hud::*;
use fishing_game::window::*;

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
        .init_state::<CurrentInterface>()
        .init_state::<MapState>()
        .init_state::<Weather>()
        .init_state::<FishingLocal>()
        .init_resource::<WeatherState>()
        .add_systems(Startup, (setup, spawn_weather_tint_overlay, spawn_day_tint_overlay))

    
        //Run the game timer
        .add_systems(Update, run_game_timer)
        .add_systems(Update, day_tint.after(run_game_timer))

        // Run the button system in both FishingMode and Overworld
        .add_systems(Update, fishing_button_system)

        .add_systems(Update, update_money_display)
        .add_systems(Update, update_clock_display)
        .add_systems(Update, update_weather_display)

        .add_systems(Update, handle_inventory.run_if(in_state(CurrentInterface::Overworld)))

        // Overworld systems (player movement, animations)
        .add_systems(Update,
            (
                move_player,
                (
                    animate_player,
                    move_camera,
                    screen_edge_collision
                ).after(move_player)
            ).run_if(in_state(CurrentInterface::Overworld))
        )

        // Weather updates
        .add_systems(Update, update_weather)
        .add_systems(Update, update_weather_tint.after(update_weather))
        .add_systems(Update, rain_particle_system.run_if(run_if_raining))
        .add_systems(OnEnter(Weather::Sunny), despawn_rain_particles)
        .add_systems(OnEnter(Weather::Cloudy), despawn_rain_particles)
        
        // Check if we've hooked any fish
        //.add_systems(Update, hook_fish)     
        .add_plugins(
            (
                FishingViewPlugin,
                MapPlugin,
                ShopPlugin
            )
        )
        
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    
    commands.spawn((
        Camera2dBundle::default(),
        Animation::new()
    ));

    commands.insert_resource(PlayerReturnPos {player_save_x: 0., player_save_y: 0.});

    
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

    //PLAYER

    let player_sheet_handle = asset_server.load("characters/full_spritesheet.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 5, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 

    // MAP
    let map: Map = Map {
        areas: vec![vec![Area {
            zone: FishingZone {
                current: Vec3::new(-10.0, 0., 0.)
            },
            layout: [[&Tile::WATER; GRID_ROWS]; GRID_COLUMNS],
            objects: Vec::new()
        }; MAP_HEIGHT]; MAP_WIDTH],
        width: MAP_WIDTH,
        height: MAP_HEIGHT
    };

    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (OLD_TILE_SIZE * 1.5), 901.),
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
    
    //spawn_button(&mut commands, asset_server);
    //spawn_button(&mut commands, asset_server);

    //Time of day timer
    commands.insert_resource(
        GameDayTimer::new(10.),
    );

    

    //let grass_layout_len = grass_layout.textures.len();
    //let tree_sheet_handle: Handle<Image> = asset_server.load("tiles/tree.png"); 

    
    spawn_fishing_button(&mut commands, &asset_server);
    spawn_money_display(&mut commands, &asset_server);
    spawn_clock_display(&mut commands, &asset_server);
    spawn_weather_display(&mut commands, &asset_server);
}
