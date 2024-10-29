extern crate rand;

use std::f32;
use std::f32::consts::PI;
use std::time::Duration;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;
use crate::fish::*;
use crate::map::Collision;
use crate::resources::*;
use crate::weather::*;
use crate::map::*;
use crate::physics::*;
use crate::species::*;

const TUG: KeyCode = KeyCode::KeyP;
const REEL: KeyCode = KeyCode::KeyO;

pub const FISHINGROOMX: f32 = 8960.;
pub const FISHINGROOMY: f32 = 3600.;

const POWER_BAR_Y_OFFSET: f32 = FISHINGROOMY - 308.;
const MAX_POWER: f32 = 250.;
const POWER_FILL_SPEED: f32 = 250.;

const ROD_MIN_ROTATION: f32 = -1.2;
const ROD_MAX_ROTATION: f32 = 1.2;
const ROD_ROTATION_SPEED: f32 = PI / 2.;

const MAX_CAST_DISTANCE: f32 = 400.;
const CASTING_SPEED: f32 = 250.;
const REEL_IN_SPEED: f32 = 150.;

#[derive(Resource)]
pub struct FishingView {
    pub rod_rotation: f32,
    pub power: f32
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FishingState {
    #[default]
    Idle,
    Casting,
    ReelingUnhooked,
    ReelingHooked
}

#[derive(Component)]
struct OnScreenLure;

#[derive(Component)]
struct PowerBar;

#[derive(Component)]
pub struct FishingRod {
    pub length: f32
}

#[derive(Resource)]
struct DirectionTimer {
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct FishingLine {
    pub cast_distance: f32,
    pub length: f32,
    pub start: Vec2,
    pub end: Vec2,
    pub mesh_handle: Handle<Mesh>,
}

impl FishingLine {
    pub const WIDTH: f32 = 3.;
}

#[derive(Component, Default)]
struct Bobber;

#[derive(Component)]
struct Wave;

#[derive(Component, Default)]
struct Splash {
    pub position: Vec3,
}

#[derive(Component)]
struct InPond;

#[derive(Component)]
pub struct IsBass;
//FISH THING
#[derive(Component)]
pub struct FishDetails {
    pub name: &'static str,
    pub fish_id: i32,
    pub length: i32,
    pub width: i32,
    pub weight: i32,
    pub time_of_day: (usize, usize),
    pub weather: Weather,
    //bounds
    pub depth: (i32, i32),
    //x, y, z
    pub position: (i32, i32),
    pub change_x: Vec3,
    pub change_y: Vec3,
    //length, width, depth
    pub bounds: (i32, i32),
    pub hunger: f32,
    pub touching_lure: bool,
}

impl FishDetails {
    pub fn new(
        name: &'static str,
        fish_id: i32,
        length: i32,
        width: i32,
        weight: i32,
        time_of_day: (usize, usize),
        weather: Weather,
        depth: (i32, i32),
        position: (i32, i32),
        change_x: Vec3,
        change_y: Vec3,
        bounds: (i32, i32),
        hunger: f32,
        touching_lure: bool,
    ) -> Self {
        Self {
            name,
            fish_id,
            length,
            width,
            weight,
            time_of_day,
            weather,
            depth,
            position,
            change_x,
            change_y,
            bounds,
            hunger,
            touching_lure,
        }
    }
}

#[derive(Component)]
pub struct FishingViewPlugin;

impl Plugin for FishingViewPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<FishingMode>()
        .init_state::<FishingState>()
        .insert_resource(FishingView { rod_rotation: 0., power: 0. })
        .add_systems(Startup, setup)
        .add_systems(Update,
            (
                move_fish,
                fish_area_bobber,
                power_bar_cast.run_if(in_state(FishingState::Idle)),
                switch_lures.run_if(in_state(FishingState::Idle)),
                rod_rotate,
                (
                    update_fishing_interface,
                    calculate_water_force,
                    calculate_player_force,
                ).after(power_bar_cast).after(rod_rotate),
                calculate_fish_force.after(calculate_water_force).after(calculate_player_force),
                simulate_physics.after(calculate_fish_force),
                cast_line.run_if(in_state(FishingState::Casting)),
                (
                    is_fish_hooked.run_if(in_state(FishingState::ReelingUnhooked)),
                    is_done_reeling.run_if(in_state(FishingState::ReelingUnhooked)),
                    is_fish_caught.run_if(in_state(FishingState::ReelingHooked)),
                ).after(simulate_physics),
                move_physics_objects.after(is_fish_caught),
                (
                    //animate_fishing_line_unhooked.run_if(in_state(FishingState::ReelingUnhooked)),
                    animate_fishing_line_hooked
                ).after(simulate_physics),
                draw_fishing_line.after(animate_fishing_line_hooked),
                animate_waves.after(simulate_physics),
                animate_splash.after(cast_line)
            ).run_if(in_state(FishingMode::Fishing))
        )
        .add_systems(OnEnter(FishingMode::Fishing), fishing_transition)
        .add_systems(OnExit(FishingMode::Fishing), overworld_transition)
        .add_systems(OnEnter(FishingState::Casting), begin_cast)
        .add_systems(OnTransition { exited: FishingState::ReelingUnhooked, entered: FishingState::Idle }, reset_interface)
        .add_systems(OnTransition { exited: FishingState::ReelingHooked, entered: FishingState::Idle }, reset_interface);
    }
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    //commands.insert_resource(FishBoundsDir {change_x: Vec3::new(0.,0.,0.), change_y: Vec3::new(0.,0.,0.)});

    commands.insert_resource(DirectionTimer{
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    });
    //let mut fish: HashMap<String, Species> = HashMap::new();

    //spawn example fish
    //BEMMY
    //BASS
    let cool_fish_handle: Handle<Image> = asset_server.load("awesomeFishy.png");
    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX, FISHINGROOMY, 901.),
                ..default()
            },
            ..default()
        },
        FishDetails {
            name: "Bass1",
            fish_id: 1,
            length: rng.gen_range(4..8),
            width: rng.gen_range(1..3),
            weight: rng.gen_range(3..10),
            time_of_day: (2,15),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHINGROOMX as i32+100, FISHINGROOMY as i32 + 100),
            hunger: 10.,
            touching_lure: false,
        },
        InPond,
        BASS,
        Collision,
    ));

    //FISH BOX
    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX-40., FISHINGROOMY+40., 901.),
                ..default()
            },
            ..default()
        },
        FishDetails {
            name: "Cat1",
            fish_id: 2,
            length: rng.gen_range(5..12),
            width: rng.gen_range(3..5),
            weight: rng.gen_range(3..10),
            time_of_day: (2,15),
            weather: Weather::Rainy,
            depth: (5,20),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHINGROOMX as i32+100, FISHINGROOMY as i32 + 100),
            hunger: 7.,
            touching_lure: false,
        },
        InPond,
        CATFISH,
        Collision,
    ));

    let fish_bass_handle: Handle<Image> = asset_server.load("fish/bass.png");

    commands.spawn((
        SpriteBundle {
            texture: fish_bass_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.),
                ..default()
            },
            ..default()
        },
        BASS,
        Fish {
            id: 0,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            age: 6.0,
            hunger: 10.0
        },
        PhysicsObject {
            mass: 2.0,
            position: Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        }
    ));

    let fishing_sheet_handle: Handle<Image> = asset_server.load("fishingStuff/fishingView.png");

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
        PowerBar
    ));

    let player_fishing_handle = asset_server.load("fishingStuff/backFishingSprite.png");
    commands.spawn((
        SpriteBundle {
            texture: player_fishing_handle.clone(),
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

    let fishing_rod_handle = asset_server.load("fishingStuff/fishingRod.png");
    commands.spawn((
        SpriteBundle {
            texture: fishing_rod_handle.clone(),
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
        FishingRod {
            length: 300.
        }
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(FishingLine::WIDTH, 0.0))),
            material: materials.add(Color::hsl(100.,1., 1.)),
            transform: Transform::from_xyz(FISHINGROOMX-90., FISHINGROOMY-(WIN_H/2.)+100.,   950.),
            ..default()
        },
        FishingLine::default()
    ));

    let splashes_sheet_handle: Handle<Image> = asset_server.load("splashes/splashes.png");
    let splash_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let splash_layout_len = splash_layout.textures.len();
    let splash_layout_handle = texture_atlases.add(splash_layout);
    commands.spawn((
        SpriteBundle {
            texture: splashes_sheet_handle.clone(),
            transform: Transform::from_xyz(FISHINGROOMX-90., FISHINGROOMY-(WIN_H/2.)+100.,   930.),
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: splash_layout_handle.clone(),
            index: 0,
        },
        AnimationTimer::new(0.2), 
        AnimationFrameCount(splash_layout_len), //number of different frames that we have
        Splash::default(),
        Animation::new()
    ));

    let waves_sheet_handle: Handle<Image> = asset_server.load("waves/waves.png");
    let wave_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 4, 1, None, None);
    let wave_layout_len = wave_layout.textures.len();
    let wave_layout_handle = texture_atlases.add(wave_layout);
    commands.spawn((
        SpriteBundle {
            texture: waves_sheet_handle.clone(),
            transform: Transform{
                translation: Vec3::new(FISHINGROOMX-90., FISHINGROOMY-(WIN_H/2.)+100.,   930.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: wave_layout_handle.clone(),
            index: 0,
        },
        //AnimationTimer::new(0.2), 
        AnimationFrameCount(wave_layout_len), //number of different frames that we have
        Wave,
        //Animation::new()
    ));



    let baits_sheet_handle: Handle<Image> = asset_server.load("fishingStuff/Baits/Baits.png");
    let baits_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let baits_layout_len = baits_layout.textures.len();
    let baits_layout_handle = texture_atlases.add(baits_layout);
    
    commands.spawn((
        SpriteBundle {
            texture: baits_sheet_handle.clone(),
            transform: Transform{
                translation: Vec3::new(FISHINGROOMX+545., FISHINGROOMY+255.,   930.),
                scale: (Vec3::splat(3.0)),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        TextureAtlas {
            layout: baits_layout_handle.clone(),
            index: 0,
        },
        OnScreenLure,
        AnimationFrameCount(baits_layout_len),
        Animation::new(),
        AnimationTimer::new(0.2), //number of different frames that we have
    ));


    let baits_sheet_handle: Handle<Image> = asset_server.load("fishingStuff/Baits/Baits.png");
    let baits_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let baits_layout_len = baits_layout.textures.len();
    let baits_layout_handle = texture_atlases.add(baits_layout);
    
    commands.spawn((
        SpriteBundle {
            texture: baits_sheet_handle.clone(),
            transform: Transform::from_xyz(FISHINGROOMX-90., FISHINGROOMY-(WIN_H/2.)+100.,   930.),
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: baits_layout_handle.clone(),
            index: 0,
        },
        Tile::BOBBER,
        PhysicsObject {
            mass: 2.0,
            position: Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },Collision,
        AnimationFrameCount(baits_layout_len), //number of different frames that we have
        Bobber::default(),
    ));

    /*let bobber_handle = asset_server.load("fishingStuff/bobber.png");
    commands.spawn((
        SpriteBundle {
            texture: bobber_handle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0.,   0.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        Tile::BOBBER,        
        PhysicsObject {
            mass: 2.0,
            position: Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        Collision,
        Bobber::default(),
    ));*/
}

fn move_fish(
    mut fish_details: Query<(&mut FishDetails, &mut Transform), (With<InPond>, With<Collision>)>,
    time: Res<Time>,
    mut config: ResMut<DirectionTimer>,
    //mut fish_direction: ResMut<FishBoundsDir>
) {
    let mut rng = rand::thread_rng();
    config.timer.tick(time.delta());
    for (mut fish_details, mut fish_pos) in fish_details.iter_mut() {
        //let mut rng = rand::thread_rng();

        if config.timer.finished() {
            println!("timer finished");
            let dir: i32 = rng.gen_range(0..9);
            println!("numer is {} {:?}", dir, fish_details.name);
            if dir == 0 {
                fish_details.change_x = Vec3::new(0., 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            } else if dir == 1 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            } else if dir == 2 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0., 0.);
            } else if dir == 3 {
                fish_details.change_x = Vec3::new(0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 4 {
                fish_details.change_x = Vec3::new(0., 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 5 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., -0.5, 0.);
            } else if dir == 6 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0., 0.);
            } else if dir == 7 {
                fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                fish_details.change_y = Vec3::new(0., 0.5, 0.);
            }
            println!(
                "numer is {:?} is going {:?}",
                fish_details.name, fish_details.change_x
            );
        }
        //match it then set up the vector for the next tree seconds, keep the stuff about borders

        //println!("fish pos x {}, fish pos y {}", change_x, change_y);
        //CHANGE THESE TO CONSTANTS LATER!!!!!
        let holdx: Vec3 = fish_pos.translation + fish_details.change_x;
        if (holdx.x) >= (8320. + 160.) && (holdx.x) <= (9391. - 160.) {
            fish_pos.translation += fish_details.change_x;
        }
        let holdy: Vec3 = fish_pos.translation + fish_details.change_y;
        if (holdy.y) >= (3376. + 90.) && (holdy.y) <= (3960. - 90.) {
            fish_pos.translation += fish_details.change_y;
        }
    }
    //fish_pos.translation += change_y;
    //return (self.position.0 + x, self.position.1+y)
}



fn fish_area_bobber(
    mut fish_details: Query<(&mut FishDetails, &mut Transform), (With<InPond>, With<Collision>, Without<Bobber>)>,
    bobber: Query<(&Transform, &Tile), With<Bobber>>,
) {
    //let (bob, tile) = bobber.single_mut();
    let (bob, tile) = bobber.single();
    for (mut fishes_details, fish_pos) in fish_details.iter_mut() {
        let fish_pos_loc = fish_pos.translation;
        let bobber_position = bob.translation;

        if fish_pos_loc.y - 180. / 2. > bobber_position.y + tile.hitbox.y / 2.
            || fish_pos_loc.y + 180. / 2. < bobber_position.y - tile.hitbox.y / 2.
            || fish_pos_loc.x + 320. / 2. < bobber_position.x - tile.hitbox.x / 2.
            || fish_pos_loc.x - 320. / 2. > bobber_position.x + tile.hitbox.x / 2.
        {
            //there is no hit
            fishes_details.touching_lure = false;
            println!("no hit");
            return;
        }
        fishes_details.touching_lure = true;
        println!("numer is {:?} {:?}", fishes_details.name, fishes_details.touching_lure);
        println!("bobber hit");
    }
}


fn fishing_transition (
    mut return_val: ResMut<PlayerReturnPos>,
    mut fishing_view: ResMut<FishingView>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut power_bar: Query<(&mut Transform, &mut PowerBar), (With<PowerBar>, Without<Camera>)>,
    mut rod: Query<&mut Transform, (With<FishingRod>, Without<Camera>, Without<PowerBar>)>,
) {
    let mut camera_transform = camera.single_mut();
    let (mut power_bar_transform, mut power) = power_bar.single_mut();
    let mut rod_transform = rod.single_mut();

    return_val.player_save_x = camera_transform.translation.x;
    return_val.player_save_y = camera_transform.translation.y;

    camera_transform.translation.x = FISHINGROOMX;
    camera_transform.translation.y = FISHINGROOMY;
    //FISHINGROOMY-308
    //spawn in powerbar
    //commands.spawn
    // power_bar_transform.translation.y = POWER_BAR_Y_OFFSET;
    // fishing_view.power = 0.;

    //rd
    // fishing_view.rod_rotation = 0.;
    // rod_transform.rotation = Quat::from_rotation_z(fishing_view.rod_rotation);

    //new movmemnt system, rotation then space hold.
    //powerbar is space A, D are rotational
}

fn overworld_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    //mut power_bar: Query<(&mut Transform, &mut Power), With<Bar>>,
    return_val: ResMut<PlayerReturnPos>,
) {
    let mut ct = camera.single_mut();
    //let (mut pb, mut power) = power_bar.single_mut();
    ct.translation.x = return_val.player_save_x;
    ct.translation.y = return_val.player_save_y;

    //pb.translation.y = (POWER_BAR_Y_OFFSET);
    //power.meter = 0;
    //set powerbar back to 0
    //set rotation back to 0
}

fn power_bar_cast(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<FishingState>>,
    mut fishing_view: ResMut<FishingView>,
    mut lure: Query<(&mut PhysicsObject, &TextureAtlas) , With<Bobber>>
) {
    if input.pressed(TUG) {
        // Increase power
        fishing_view.power = fishing_view.power + POWER_FILL_SPEED * time.delta_seconds();

        if fishing_view.power >= MAX_POWER {
            // Max power reached, release
            set_bobber_physics(lure);
            fishing_view.power = MAX_POWER;
            next_state.set(FishingState::Casting);
        }
    } else if input.just_released(TUG) {
        // Manual release
        set_bobber_physics(lure);
        next_state.set(FishingState::Casting);
    }
}

fn set_bobber_physics(
    mut lure: Query<(&mut PhysicsObject, &TextureAtlas) , With<Bobber>>
){
    let (mut bobber_physics, texture) = lure.single_mut();
    if texture.index == 0 {//bobber physics!!!
        *bobber_physics = PhysicsObject::new(2.0, Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.), Vec3::ZERO, Vec3::ZERO, Forces::default());
    }if texture.index == 1  {//frog physics!!!
        *bobber_physics = PhysicsObject::new(5.0, Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.), Vec3::ZERO, Vec3::ZERO, Forces::default());
    }else if texture.index == 2 {//swimbait physics!!!
        *bobber_physics =  PhysicsObject::new(20.0, Vec3::new(FISHINGROOMX, FISHINGROOMY + 100., 0.), Vec3::ZERO, Vec3::ZERO, Forces::default());
    }
}

fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut fishing_view: ResMut<FishingView>
) {
    let mut direction = 0.;
 
    if input.pressed(KeyCode::KeyA) {
        direction += 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        direction += -1.;
    }

    let new_rotation = fishing_view.rod_rotation + direction * ROD_ROTATION_SPEED * time.delta_seconds();
    fishing_view.rod_rotation = new_rotation.clamp(ROD_MIN_ROTATION, ROD_MAX_ROTATION);
}

fn begin_cast (
    fishing_view: Res<FishingView>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<&mut Visibility, With<Bobber>>
) {
    let mut line_info = line.single_mut();
    let mut bobber_visibililty = bobber.single_mut(); 

    line_info.cast_distance = fishing_view.power / MAX_POWER * MAX_CAST_DISTANCE;
    *bobber_visibililty = Visibility::Visible;
}

fn cast_line (
    time: Res<Time>,
    fishing_view: Res<FishingView>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<(&mut Transform, &mut PhysicsObject, Entity), (With<Bobber>, Without<FishingRod>)>,
    mut splash: Query<(&mut Splash, &mut Visibility), With<Splash>>,
    mut commands: Commands,
) {
    let (rod_info, rod_transform) = rod.single();
    let mut line_info = line.single_mut();
    let (mut bobber_transform, mut bobber_physics, entity_id) = bobber.single_mut();
    let (mut splash_info, mut splash_visibility) = splash.single_mut();
    let new_length = (line_info.length + CASTING_SPEED * time.delta_seconds()).min(line_info.cast_distance);
    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    
    line_info.start = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
    line_info.end = rod_transform.translation.xy() + (rod_info.length / 2. + line_info.length) * angle_vector;

    if new_length == line_info.cast_distance {
        // Cast finished
        line_info.length = line_info.cast_distance;
        // Splash animation
        *splash_visibility = Visibility::Visible;
        splash_info.position = rod_transform.translation + ((rod_info.length / 2. + line_info.length) * angle_vector).extend(0.);
        next_state.set(FishingState::ReelingUnhooked);
    } else {
        // Casting
        line_info.length = new_length;
    }

    //setting the position of the bobber along with the physics location of the bobber.
    //also make sure that we are setting the bobber to be a hooked object
    commands.entity(entity_id).insert(Hooked);
    
    bobber_physics.position = line_info.end.extend(950.);
    bobber_transform.translation = line_info.end.extend(950.);

}

fn switch_lures(
    mut screen_lure: Query< (&mut TextureAtlas, &mut AnimationTimer ), With<OnScreenLure> >,
    mut bait_lure: Query< &mut TextureAtlas , (With<Bobber>, Without<OnScreenLure>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,  
){
    let (mut screen_texture, mut timer)  = screen_lure.single_mut();
    let mut bait_texture = bait_lure.single_mut();

    timer.tick(time.delta());

    if timer.just_finished()
    {
        if input.pressed(KeyCode::KeyZ) {
            if bait_texture.index == 2 
            {
                bait_texture.index = 0;
                screen_texture.index = 0;
                return;
            }
            bait_texture.index = bait_texture.index + 1;
            screen_texture.index = screen_texture.index + 1;
        }

        if input.pressed(KeyCode::KeyX) {
            if bait_texture.index == 0 
            {
                bait_texture.index = 2;
                screen_texture.index = 2;
                return;
            }
            bait_texture.index = bait_texture.index - 1;
            screen_texture.index = screen_texture.index - 1;
        }
    }
}

fn update_fishing_interface (
    fishing_view: Res<FishingView>,
    mut power_bar: Query<&mut Transform, With<PowerBar>>,
    mut rod: Query<&mut Transform, (With<FishingRod>, Without<PowerBar>)>
) {
    let mut power_bar_transform = power_bar.single_mut();
    let mut rod_transform = rod.single_mut();

    power_bar_transform.translation.y = POWER_BAR_Y_OFFSET + fishing_view.power;
    rod_transform.rotation = Quat::from_rotation_z(fishing_view.rod_rotation);
}

fn is_fish_hooked (
    mut commands: Commands,
    mut next_state: ResMut<NextState<FishingState>>,
    mut bobber: Query<(&Transform, &Tile, Entity, &PhysicsObject, &mut Visibility),  With<Bobber>>,
    mut fishes: Query<(Entity, &Fish, &mut PhysicsObject), (With<Fish>, Without<Bobber>)>
) {
    let (bobber_transform, tile,  bobber_entity_id, bobber_physics, mut bobber_visibility) = bobber.single_mut();
    let bobber_position = bobber_transform.translation;

    for (entity_id, fish_details, mut fish_physics) in fishes.iter_mut() {
        if fish_physics.position.y - fish_details.width / 2. > bobber_position.y + tile.hitbox.y / 2.
        || fish_physics.position.y + fish_details.width / 2. < bobber_position.y - tile.hitbox.y / 2. 
        || fish_physics.position.x + fish_details.width / 2. < bobber_position.x - tile.hitbox.x / 2. 
        || fish_physics.position.x - fish_details.width / 2. > bobber_position.x + tile.hitbox.x / 2.
        {
            continue;
        }
    
        //no longer reeling in bobber so remove that entity. instead add the fish as the hooked entity.
        //also add weight of the bobber to the fish
        *bobber_visibility = Visibility::Hidden;
        fish_physics.mass = fish_physics.mass + bobber_physics.mass;
        commands.entity(bobber_entity_id).remove::<Hooked>();
        commands.entity(entity_id).insert(Hooked);
        next_state.set(FishingState::ReelingHooked);
        break;
    }
}

fn is_done_reeling(
    mut commands: Commands,
    fishing_view: Res<FishingView>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut casted_lure: Query<(Entity, &PhysicsObject), With<Hooked>>,
){
    let (rod_info, rod_transform) = rod.single();
    let (entity_id, lure_physics) = casted_lure.single_mut();

    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    let catch_pos = rod_transform.translation.with_z(0.) + (rod_info.length / 4. * angle_vector).extend(0.);
    let distance = (lure_physics.position - catch_pos).length();

    if distance < 100. {
        commands.entity(entity_id).remove::<Hooked>();
        next_state.set(FishingState::Idle);
    }

}

fn is_fish_caught (
    mut commands: Commands,
    fishing_view: Res<FishingView>,
    mut playerInventory: Query<&mut PlayerInventory>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut hooked_object: Query<(Entity, &mut Fish, &mut PhysicsObject), With<Hooked>>,
) {
    let (rod_info, rod_transform) = rod.single();
    let (entity_id, mut fish_details, mut fish_physics) = hooked_object.single_mut();
    let mut inventory_info = playerInventory.single_mut();

    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    let catch_pos = rod_transform.translation.with_z(0.) + (rod_info.length / 4. * angle_vector).extend(0.);
    let distance = (fish_physics.position - catch_pos).length();

    if distance < 15. {
        fish_details.is_caught = true;
        inventory_info.coins += fish_details.weight as u32 * 2;

        // Reset fish for testing
        fish_physics.position = Vec3::new(FISHINGROOMX, FISHINGROOMY, 0.);
        fish_physics.velocity = Vec3::new(0., 0., 0.);
        fish_physics.forces = Forces::default();
        fish_details.is_caught = false;
        commands.entity(entity_id).remove::<Hooked>();

        next_state.set(FishingState::Idle);
    }
}

fn animate_fishing_line_unhooked (
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    fishing_view: Res<FishingView>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<(&mut Transform, Entity ), (With<Bobber>, Without<FishingRod>)>
) {
    let (rod_info, rod_transform) = rod.single();
    let mut line_info = line.single_mut();
    let (mut bobber_transform, bobber_entity_id) = bobber.single_mut();

    // Reeling
    if input.pressed(REEL) {
        line_info.length = (line_info.length - REEL_IN_SPEED * time.delta_seconds()).max(0.);

        if line_info.length == 0. {
            // Line fully reeled back in
            //remove hooked to stop physics interaction
            commands.entity(bobber_entity_id).remove::<Hooked>();
            next_state.set(FishingState::Idle);
            return;
        }
    }

    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    let rod_end = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
    let line_end = rod_transform.translation.xy() + (rod_info.length / 2. + line_info.length) * angle_vector;

    line_info.start = rod_end;
    line_info.end = line_end;

    bobber_transform.translation = line_end.extend(950.);
}

fn animate_fishing_line_hooked (
    fishing_view: Res<FishingView>,
    rod: Query<(&FishingRod, &Transform), With<FishingRod>>,
    mut fish: Query<(&Species, &PhysicsObject), With<Fish>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<(&mut Transform, &PhysicsObject), (With<Bobber>, Without<FishingRod>)>,
    state: Res<State<FishingState>>
) {

    if *state.get() != FishingState::ReelingHooked && *state.get() != FishingState::ReelingUnhooked{        
        return;
    }

    println!("reeling in line");
    let (rod_info, rod_transform) = rod.single();
    let (fish_species, fish_physics) = fish.single_mut();
    let mut line_info = line.single_mut();
    let(mut bobber_transform, bobber_physics) = bobber.single_mut();

    let angle_vector = Vec2::from_angle(fishing_view.rod_rotation + PI / 2.);
    let rod_end = rod_transform.translation.xy() + rod_info.length / 2. * angle_vector;
    let fish_offset = fish_species.hook_pos.rotate(Vec2::from_angle(fish_physics.rotation.z));
    let fish_pos = fish_physics.position.xy() + fish_offset;

    if *state.get() == FishingState::ReelingHooked{
        line_info.start = rod_end;
        line_info.end = fish_pos;
    }else{
        line_info.start = rod_end;
        line_info.end = bobber_physics.position.xy();
    }
    // Hook line to fish
    //bobber_transform.translation =  fish_pos.extend(950.);
}

fn reset_interface (
    mut fishing_view: ResMut<FishingView>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut splash: Query<&mut TextureAtlas, With<Splash>>,
    mut bobber: Query<&mut Visibility, With<Bobber>>
) {
    let mut line_info = line.single_mut();
    let mut splash_texture = splash.single_mut();
    let mut bobber_visibility = bobber.single_mut();
    
    line_info.length = 0.;
    line_info.start = Vec2::ZERO;
    line_info.end = Vec2::ZERO;
    *bobber_visibility = Visibility::Hidden;
    splash_texture.index = 0;
    fishing_view.power = 0.;
}

pub fn move_physics_objects (
    mut objects: Query<(&PhysicsObject, &mut Transform), With<PhysicsObject>>
) {
    for (physics_object, mut transform) in objects.iter_mut() {
        transform.translation = physics_object.position.with_z(901.);
        transform.rotation = Quat::from_rotation_z(physics_object.rotation.z);
    }
}

fn draw_fishing_line (
    mut meshes: ResMut<Assets<Mesh>>,
    mut line: Query<(&mut Transform, &mut Mesh2dHandle, &mut FishingLine), (With<FishingLine>, Without<FishingRod>)>,
) {
    println!("drawing line");
    let (mut line_transform, mut line_mesh, mut line_info) = line.single_mut();

    let pos_delta = line_info.end - line_info.start;
    let line_length = pos_delta.length();
    let line_pos = (line_info.start + line_info.end) / 2.;
    let line_rotation =  f32::atan2(pos_delta.y, pos_delta.x) + PI / 2.;

    // Draw fishing line
    line_transform.translation = Vec3::new(line_pos.x, line_pos.y, line_transform.translation.z);
    line_transform.rotation = Quat::from_rotation_z(line_rotation);

    meshes.remove(&line_info.mesh_handle);
    line_info.mesh_handle = meshes.add(Rectangle::new(FishingLine::WIDTH, line_length));
    *line_mesh = Mesh2dHandle(line_info.mesh_handle.clone());
}

fn animate_splash(
    mut splash: Query<(&Splash, &mut Transform, &mut Visibility, &mut TextureAtlas, &mut AnimationTimer), With<Splash>>,
    time: Res<Time>
) {
    let (splash, mut transform, mut visibility, mut texture, mut timer) = splash.single_mut();

    if *visibility == Visibility::Hidden {
        return;
    }

    transform.translation = splash.position;

    // Animate splash
    if texture.index < 3 {
        timer.tick(time.delta());

        if timer.just_finished() {
            if texture.index == 2 {
                *visibility = Visibility::Hidden;
            } else {
                texture.index += 1;
            }
        }
    }
}

fn animate_waves (
    objects: Query<&PhysicsObject, (With<PhysicsObject>, With<Fish>)>, 
    mut wave: Query<(&mut TextureAtlas, &mut Transform, &mut Visibility), With<Wave>>
) {
    let object = objects.single();
    let (mut wave_texture, mut wave_transform, mut wave_visibility) = wave.single_mut();

    let magnitude = object.forces.water.length();

    if magnitude == 0. {
        *wave_visibility = Visibility::Hidden;
        return
    }
    
    if magnitude < 200. {
        wave_texture.index = 0;
    } else if magnitude < 400. {
        wave_texture.index = 1;
    } else if magnitude < 600. {
        wave_texture.index = 2;
    } else {
        wave_texture.index = 3;
    }

    *wave_visibility = Visibility::Visible;
    wave_transform.translation = object.position.with_z(902.);
    wave_transform.rotation = Quat::from_rotation_z(f32::atan2(object.forces.water.y, object.forces.water.x) - PI / 2.);
}
