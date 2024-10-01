use bevy::{prelude::*, window::PresentMode};
use std::convert::From;
use std::time::Duration;


const TITLE: &str = "button";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 100.;
const ACCEL_RATE: f32 = 500.;

const TILE_SIZE: u32 = 100;

const LEVEL_LEN: f32 = 5000.;

const ANIM_TIME: f32 = 0.125; // 8 fps
const FISHING_ANIM_TIME: f32 = 0.25; // 4 frames per second for fishing animation



#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
struct Background;

#[derive(Component)]
struct ButtonVisible(bool);

#[derive(Resource)]
struct StartFishingAnimation {
    active: bool,
    button_control_active: bool, 
}


#[derive(Resource)]
struct FishingAnimationDuration(Timer);

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

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

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
        
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Visibility,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,        
) {
    for (interaction, mut color, mut border_color, mut visibility, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Throw Rod".to_string();
                *color = PRESSED_BUTTON.into();
                start_fishing_animation.active = true;
                start_fishing_animation.button_control_active = false;
                fishing_timer.0.reset();
                *visibility = Visibility::Hidden;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Throw Rod".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Throw Rod".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
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

    let player_sheet_handle = asset_server.load("characters/angler-back-moving.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
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
        PlayerDirection::Back, 
    ));

 
    spawn_button(&mut commands, asset_server);
}

fn spawn_button(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            ButtonVisible(false), 
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Throw Rod",
                        TextStyle {
                            font: asset_server.load("pixel.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}


fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerDirection), With<Player>>,
    // mut button_query: Query<(&mut Visibility, &mut ButtonVisible)>, 
    // start_fishing_animation: Res<StartFishingAnimation>,
) {
    let (mut pt, mut pv, mut direction) = player.single_mut();
    let mut deltav = Vec2::splat(0.);


    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
        *direction = PlayerDirection::Left;
    } else if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
        *direction = PlayerDirection::Right;
    } else if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
        *direction = PlayerDirection::Back;
    } else if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
        *direction = PlayerDirection::Front;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;


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


    // This turn off the button visibility when the player is above a certain point on the screen 
    // if start_fishing_animation.button_control_active {
    //     for (mut visibility, mut button_visible) in &mut button_query {
    //         if pt.translation.y >= -180. && pt.translation.y <= 0. && !button_visible.0 {
    //             *visibility = Visibility::Visible;
    //             button_visible.0 = true;
    //         } else if (pt.translation.y < 720. && pt.translation.y > 0. || pt.translation.y < -180.) && button_visible.0 {
    //             *visibility = Visibility::Hidden;
    //             button_visible.0 = false;
    //         }
    //     }
    // }

}



fn animate_player(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,
    mut button_query: Query<&mut Visibility, With<Button>>,
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

  
    if start_fishing_animation.active {
        *texture_handle = asset_server.load("characters/angler-back-fishing.png");

        timer.set_duration(Duration::from_secs_f32(FISHING_ANIM_TIME));
        timer.tick(time.delta());

        if timer.just_finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }

        fishing_timer.0.tick(time.delta());
        if !fishing_timer.0.finished() {
            return; 
        }

        
        start_fishing_animation.active = false;
        start_fishing_animation.button_control_active = true;

        //fix this
        for mut visibility in &mut button_query {
            *visibility = Visibility::Visible;
        }
    }


    match *direction {
        PlayerDirection::Front => {
            *texture_handle = asset_server.load("characters/angler-front-moving.png");
        }
        PlayerDirection::Back => {
            *texture_handle = asset_server.load("characters/angler-back-moving.png");
        }
        PlayerDirection::Left => {
            *texture_handle = asset_server.load("characters/angler-left-moving.png");
        }
        PlayerDirection::Right => {
            *texture_handle = asset_server.load("characters/angler-right-moving.png");
        }
    }

    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());
        if timer.just_finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    } else {
        match *direction {
            PlayerDirection::Front => {
                *texture_handle = asset_server.load("characters/angler-front-still.png");
                texture_atlas.index = 0;
            }
            PlayerDirection::Back => {
                *texture_handle = asset_server.load("characters/angler-back-still.png");
                texture_atlas.index = 0;
            }
            PlayerDirection::Left => {
                *texture_handle = asset_server.load("characters/angler-left-still.png");
                texture_atlas.index = 0;
            }
            PlayerDirection::Right => {
                *texture_handle = asset_server.load("characters/angler-right-still.png");
                texture_atlas.index = 0;
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
