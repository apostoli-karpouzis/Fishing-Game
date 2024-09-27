use bevy::{prelude::*, window::PresentMode};

const CREDIT_TIME: f32 = 3.0;
const CREDIT_LEN: usize = 8;
#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component)]
struct Credits;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fishing Game".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, roll_credits)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let end_sheet_handle = asset_server.load("sprite_sheet.png");
    let end_layout = TextureAtlasLayout::from_grid(UVec2::new(1280,720), 8, 1, None, None);
    //commands.spawn(EndCredLen(end_layout.textures.len()));
    let end_layout_handle = texture_atlases.add(end_layout);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: end_sheet_handle.clone(),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: end_layout_handle.clone(),
        },  
        Credits,
    ));

    commands.spawn(PopupTimer(Timer::from_seconds(CREDIT_TIME, TimerMode::Repeating)));
}

fn roll_credits(
    time: Res<Time>,
    mut end_layout: Query<(&mut TextureAtlas), With<Credits>>,
    mut popup: Query<&mut PopupTimer>,
) {
    let mut texture_atlas = end_layout.single_mut();
    let mut timer = popup.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() && texture_atlas.index < (CREDIT_LEN-1){
        texture_atlas.index = (texture_atlas.index + 1);
    }
}
