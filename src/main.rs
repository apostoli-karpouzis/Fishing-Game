extern crate rand;
use bevy::window::EnabledButtons;
use bevy::time::Timer;
use bevy::prelude::*;
use bevy::window::PresentMode;
use fishing_game::interface::CurrentInterface;

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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .insert_resource(StartFishingAnimation { active: false, button_control_active: true })
        .insert_resource(FishingAnimationDuration(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(GameDayTimer::new(3.))
        .insert_resource(PlayerReturnPos::default())
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
        .init_state::<MidnightState>()
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
) {   
    commands.spawn((
        Camera2dBundle::default(),
        Animation::new()
    ));

    //PLAYER

    let player_sheet_handle = asset_server.load("characters/full_spritesheet.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 4, 5, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);

    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform::from_xyz(0., -300., 901.),
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
            x: 0,
            y: 0
        },
        CanPickUp{
            isitem: false,
        },
        Animation::new()
    ));
    
    spawn_fishing_button(&mut commands, &asset_server);
    spawn_money_display(&mut commands, &asset_server);
    spawn_clock_display(&mut commands, &asset_server);
    spawn_weather_display(&mut commands, &asset_server);
    spawn_hint(&mut commands, &asset_server);
}
