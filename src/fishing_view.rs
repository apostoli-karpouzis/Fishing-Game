extern crate rand;

use std::hash::{Hash, Hasher};
use std::f32;
use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::*;
use rand::Rng;
use crate::fish::*;
use crate::gameday::*;
use crate::interface::*;
use crate::inventory::*;
use crate::resources::*;
use crate::weather::*;
use crate::map::*;
use crate::physics::*;
use crate::species::*;
use crate::prob_calc::*;
use crate::window::*;

const TUG: KeyCode = KeyCode::KeyP;
const REEL: KeyCode = KeyCode::KeyO;
const ROTATE_ROD_COUNTERLCOCKWISE: KeyCode = KeyCode::KeyA;
const ROTATE_ROD_CLOCKWISE: KeyCode = KeyCode::KeyD;
const SWITCH_ROD: KeyCode = KeyCode::KeyN;
const SWITCH_LINE: KeyCode = KeyCode::KeyM;
const SWITCH_BAIT_NEXT: KeyCode = KeyCode::KeyX;
const SWITCH_BAIT_PREV: KeyCode = KeyCode::KeyZ;

pub const PARTICLECOUNT: usize = 10;

const CATCH_MARGIN: f32 = 30.;

pub const FISHING_ROOM_CENTER: Vec2 = Map::get_area_center(0, -1);
pub const FISHING_ROOM_X: f32 = FISHING_ROOM_CENTER.x;
pub const FISHING_ROOM_Y: f32 = FISHING_ROOM_CENTER.y;

pub const PLAYER_POSITION: Vec3 = Vec3::new(FISHING_ROOM_X-100., FISHING_ROOM_Y-(WIN_H/2.)+50., 902.);


//pub const FISHINGROOMX: f32 = 8960.;
//pub const FISHINGROOMY: f32 = 3600.;


const POWER_BAR_Y_OFFSET: f32 = FISHING_ROOM_Y - 308.;
const MAX_POWER: f32 = 250.;
const POWER_FILL_SPEED: f32 = 250.;

const ROD_MIN_ROTATION: f32 = PI / 6.;
const ROD_MAX_ROTATION: f32 = 5. / 6. * PI;
const ROD_ROTATION_SPEED: f32 = PI / 2.;

const MAX_CAST_DISTANCE: f32 = 400.;
const CASTING_SPEED: f32 = 250.;
const REEL_IN_SPEED: f32 = 150.;

#[derive(Resource)]
pub struct StartFishingAnimation {
    pub active: bool,
    pub button_control_active: bool, 
}

#[derive(Resource)]
pub struct FishingAnimationDuration(pub Timer);

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
struct PondScreen;

#[derive(Component)]
struct BeachScreen;

#[derive(Component)]
struct PowerBar {
    power: f32
}

#[derive(Component)]
struct MysteryFish;

#[derive(Component)]
struct PhysicsFish;

#[derive(Component)]
pub struct FishingRod {
    pub rod_type: &'static FishingRodType,
    pub rotation: f32,
    pub material: Handle<ColorMaterial>,
    pub segments: Vec<Entity>,
    pub tip_pos: Vec3
}

#[derive(Component, Default)]
pub struct ParticleList{
    pub particle_list: Vec<Particle>
}


#[derive(Component, Default)]
struct Bobber;


#[derive(Component, Clone)]

pub struct Particle{
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl Particle{
    pub const fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self{
        Self{position, velocity, mass}
    }
}

impl Hash for Particle {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.position.x as i32;
        self.position.y as i32;
        self.position.z as i32; 
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.position.x as i32 == other.position.x as i32 &&
        self.position.y as i32 == other.position.y as i32 &&
        self.position.z as i32 == other.position.z as i32 
    }
}

impl Eq for Particle {}

#[derive(Component)]
struct FishingRodSegment;

pub struct FishingRodType {
    pub texture: &'static str,
    pub length: f32,
    pub radius: f32,
    pub thickness: f32,
    pub flexural_strength: f32,
    pub shear_modulus: f32,
    pub blank_color: Color
}

impl FishingRodType {
    pub const fn new(texture: &'static str, length: f32, radius: f32, thickness: f32, shear_strength: f32, shear_modulus: f32, blank_color: Color) -> Self {
        Self { texture, length, radius, thickness, flexural_strength: shear_strength, shear_modulus, blank_color }
    }
    
    pub const NORMAL: FishingRodType = FishingRodType::new("rods/default.png", 1.5, 0.015, 0.004, 3450E6, 72E9, Color::BLACK);
    pub const SURF: FishingRodType = FishingRodType::new("rods/surf.png", 2., 0.015, 0.004, 3450E6, 72E9, Color::BLACK);
}

#[derive(Resource)]
struct DirectionTimer {
    pub timer: Timer,
}

#[derive(Resource)]
struct ExclamationTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct FishingLine {
    pub cast_distance: f32,
    pub length: f32,
    pub start: Vec3,
    pub end: Vec3,
    pub line_type: &'static FishingLineType
}

impl FishingLine {
    pub fn new(line_type: &'static FishingLineType) -> Self {
        Self {
            cast_distance: 0.0,
            length: 0.0,
            start: Vec3::ZERO,
            end: Vec3::ZERO,
            line_type
        }
    }

    pub const WIDTH: f32 = 3.;
}

#[derive(PartialEq)]
pub struct FishingLineType {
    pub ultimate_tensile_strength: f32,
    pub color: Color
}

impl FishingLineType {
    pub const fn new(ultimate_tensile_strength: f32, color: Color) -> Self {
        Self { ultimate_tensile_strength, color }
    }
    
    pub const FLUOROCARBON: FishingLineType = FishingLineType::new(3000., Color::srgb(0.1, 0.1, 0.8));
    pub const BRAIDED: FishingLineType = FishingLineType::new(4000., Color::srgb(0.0, 0.7, 0.2));
    pub const MONOFILILMENT: FishingLineType = FishingLineType::new(2000., Color::srgb(0.9, 0.9, 0.9));
}

#[derive(Component, Default)]
pub struct Lure {
    texture_index: usize,
    mass: f32
}

impl Lure {
    pub const fn new(texture_index: usize, mass: f32) -> Self {
        Self { texture_index, mass }
    }

    pub const BALL: Lure = Lure::new(0, 2.0);
    pub const FROG: Lure = Lure::new(1, 5.0);
    pub const FISH: Lure = Lure::new(2, 20.0);
}

#[derive(Component)]
struct Wave;

#[derive(Component, Default)]
struct Splash {
    pub position: Vec3,
}

#[derive(Component)]
pub struct InPond;

#[derive(Component)]
pub struct IsBass;

#[derive(Component)]
pub struct exclam_point;

#[derive(Component)]
pub struct PondObstruction;

#[derive(Component, PartialEq)]
pub enum ObstType{
    Tree,
    Fissure,
    Pad,
}

#[derive(Component, PartialEq)]
pub enum FishLoc{
    Pond1,
    Pond2,
    Ocean,
}

//FISH THING
#[derive(Component)]
struct FishDetails {
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
        .init_state::<FishingState>()
        .insert_resource(ProbTimer::new(2.))
        .add_systems(Startup, setup)
        .add_systems(Update,
            (
                move_fish,
                fish_area_bobber.run_if(in_state(FishingState::ReelingUnhooked)).after(move_fish),
                (
                    power_bar_cast,
                    switch_rod,
                    switch_line,
                    switch_bait
                ).run_if(in_state(FishingState::Idle)),
                rod_rotate,
                (
                    calculate_water_force,
                    calculate_player_force.run_if(in_state(FishingState::ReelingUnhooked).or_else(in_state(FishingState::ReelingHooked))),
                ).after(rod_rotate),
                calculate_fish_force.after(calculate_water_force).after(calculate_player_force),
                simulate_physics.after(calculate_fish_force),
                (
                    is_done_reeling.run_if(in_state(FishingState::ReelingUnhooked)),
                    is_fish_caught.run_if(in_state(FishingState::ReelingHooked)),
                    is_line_broken.run_if(in_state(FishingState::ReelingHooked)),
                    cast_line.run_if(in_state(FishingState::Casting)),
                    animate_fishing_line.run_if(not(in_state(FishingState::Casting)))
                ).after(simulate_physics),
                move_physics_objects.after(is_fish_caught),
                bend_fishing_rod.after(is_line_broken),
                draw_fishing_line.after(animate_fishing_line),
                animate_waves.after(simulate_physics),
                animate_splash.after(cast_line)
            ).run_if(in_state(CurrentInterface::Fishing))
        )
        .add_systems(OnEnter(CurrentInterface::Fishing), (fishing_transition, add_fish))
        .add_systems(OnExit(CurrentInterface::Fishing), overworld_transition)
        .add_systems(OnEnter(FishingState::Casting), begin_cast)
        .add_systems(OnTransition { exited: FishingState::ReelingUnhooked, entered: FishingState::Idle }, reset_interface)
        .add_systems(OnEnter(MidnightState::Midnight), fishPopulation)
        .add_systems(OnTransition { exited: FishingState::ReelingHooked, entered: FishingState::Idle }, reset_interface)
        .add_systems(Update, fish_update.run_if(in_state(CurrentInterface::Fishing)));
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

    commands.insert_resource(ExclamationTimer{
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
    });
    //let mut fish: HashMap<String, Species> = HashMap::new();

    //spawn example fish
    //BEMMY
    //BASS
    let cool_fish_handle: Handle<Image> = asset_server.load("fishing_view/awesome_fishy.png");
    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.),
                ..default()
            },
            ..default()
        },
        Fish{
            name: "bass",
            id: 0,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        InPond,
        BASS,
        Collision,
        MysteryFish,
        FishLoc::Pond1,
    ));


    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.),
                ..default()
            },
            ..default()
        },
        Fish{
            name: "bass",
            id: 2,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        InPond,
        BASS,
        Collision,
        MysteryFish,
        FishLoc::Pond2,
    ));
    let fish_bass_handle: Handle<Image> = asset_server.load("fish/bass.png");

    commands.spawn((
        SpriteBundle {
            texture: fish_bass_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 901.),
                ..default()
            },
            ..default()
        },
        BASS,
        Fish{
            name: "Bass2",
            id: 2,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        PhysicsObject{
            mass: 2.0,
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        InPond,
        Collision,
        PhysicsFish,
        FishLoc::Pond2,
    ));



    //FISH BOX
    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X-40., FISHING_ROOM_Y+40., 901.),
                ..default()
            },
            
            ..default()
        },
        Fish {
            name: "catfish",
            id: 1,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        InPond,
        CATFISH,
        Collision,
        MysteryFish,
        FishLoc::Pond1,
    ));


    commands.spawn((
        SpriteBundle {
            texture: fish_bass_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 901.),
                ..default()
            },
            ..default()
        },
        BASS,
        Fish{
            name: "Bass2",
            id: 0,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        PhysicsObject{
            mass: 2.0,
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        InPond,
        Collision,
        PhysicsFish,
        FishLoc::Pond1,
    ));

    let fish_bass_handle: Handle<Image> = asset_server.load("fish/catfish.png");

    commands.spawn((
        SpriteBundle {
            texture: fish_bass_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
                ..default()
            },
            ..default()
        },
        CATFISH,
        Fish{
            name: "Catfish2",
            id: 1,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        PhysicsObject{
            mass: 3.0,
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        InPond,
        Collision,
        PhysicsFish,
        FishLoc::Pond1,
    ));

    let fishing_sheet_handle: Handle<Image> = asset_server.load("fishing_view/fishing_view.png");

    commands.spawn((
        SpriteBundle {
            texture: fishing_sheet_handle.clone(),
                        sprite: Sprite {
                        ..default()
                    },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 900.),
                ..default()
            },
            ..default()
        },
        PondScreen
        
    ));

    let beach_sheet_handle: Handle<Image> = asset_server.load("fishing_view/beach_view.png");


    commands.spawn((
        SpriteBundle {
            texture: beach_sheet_handle.clone(),
                        sprite: Sprite {
                        ..default()
                    },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 900.),
                ..default()
            },
            ..default()
        },
        BeachScreen
    ));
    
    //powerbar view
    let bar_sheet_handle = asset_server.load("fishing_view/power_bar.png");
    commands.spawn((
        SpriteBundle {
            texture: bar_sheet_handle.clone(),
                        sprite: Sprite{
                        ..default() 
                        },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X+575., FISHING_ROOM_Y-308., 899.),
                ..default()
            },
            ..default()
        },
        PowerBar {
            power: 0.
        }
    ));

    let player_fishing_handle = asset_server.load("fishing_view/back_fishing_sprite.png");
    commands.spawn((
        SpriteBundle {
            texture: player_fishing_handle.clone(),
                        sprite: Sprite{
                        ..default() 
                        },
            transform: Transform {
                translation: PLAYER_POSITION,
                ..default()
            },
            ..default()
        },
    ));

    let exclamation_point_handle = asset_server.load("fishing_view/ExclamationPoint.png");
    commands.spawn((
        SpriteBundle {
            texture: exclamation_point_handle.clone(),
                                    
                        sprite: Sprite{
                        ..default() 
                        },

            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        exclam_point,
    ));
    

    let exclamation_point_handle = asset_server.load("fishing_view/ExclamationPoint.png");
    commands.spawn((
        SpriteBundle {
            texture: exclamation_point_handle.clone(),
                                    
                        sprite: Sprite{
                        ..default() 
                        },

            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        exclam_point,
    ));
    

    let default_rod_type = &FishingRodType::NORMAL;
    let segment_count: usize = (FishingRodType::NORMAL.length / BENDING_RESOLUTION) as usize;

    let mut rod_info: FishingRod = FishingRod {
        rod_type: default_rod_type,
        rotation: PI / 2.,
        material: materials.add(default_rod_type.blank_color),
        segments: Vec::with_capacity(segment_count),
        tip_pos: Vec3::new(PLAYER_POSITION.x, PLAYER_POSITION.y + default_rod_type.length * PIXELS_PER_METER, 0.)
    };

    for i in (0..segment_count).rev() {
        let l = i as f32 * BENDING_RESOLUTION;
        let radius = default_rod_type.thickness * l / default_rod_type.length;
        let radius_pixels = (radius * 750.).max(1.);

        let segment = commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(radius_pixels, radius_pixels))),
                material: rod_info.material.clone(),
                ..default()
            },
            FishingRodSegment
        ));

        rod_info.segments.push(segment.id());
    }

    let mut particle_info: ParticleList = ParticleList{
        particle_list : Vec::with_capacity(PARTICLECOUNT), 
        ..default()
    };

    for i in  0 .. PARTICLECOUNT {
        let particlepos =  Vec3::new(0.,0.,0.);
        particle_info.particle_list.push(Particle::new(particlepos, Vec3::new(0., 0., 0.),10. ));
    }

    commands.spawn(particle_info);


    let fishing_rod_handle = asset_server.load(default_rod_type.texture);
    
    commands.spawn((
        SpriteBundle {
            texture: fishing_rod_handle.clone(),
            transform: Transform {
                translation: Vec3::new(PLAYER_POSITION.x, PLAYER_POSITION.y, 901.),
                ..default()
            },
            ..default()
        },
        rod_info
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(FishingLine::WIDTH, 0.0))),
            material: materials.add(Color::hsl(100.,1., 1.)),
            transform: Transform::from_xyz(FISHING_ROOM_X-90., FISHING_ROOM_Y-(WIN_H/2.)+100.,   950.),
            ..default()
        },
        FishingLine::new(&FishingLineType::MONOFILILMENT)
    ));

    let splashes_sheet_handle: Handle<Image> = asset_server.load("fishing_view/splashes.png");
    let splash_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let splash_layout_len = splash_layout.textures.len();
    let splash_layout_handle = texture_atlases.add(splash_layout);
    commands.spawn((
        SpriteBundle {
            texture: splashes_sheet_handle.clone(),
            transform: Transform::from_xyz(FISHING_ROOM_X-90., FISHING_ROOM_Y-(WIN_H/2.)+100.,   930.),
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: splash_layout_handle.clone(),
            index: 0,
        },
        AnimationTimer::new(0.2), 
        AnimationFrameCount(splash_layout_len), //number of different frames that we have
        Splash::default()
    ));

    let waves_sheet_handle: Handle<Image> = asset_server.load("fishing_view/waves.png");
    let wave_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 4, 1, None, None);
    let wave_layout_len = wave_layout.textures.len();
    let wave_layout_handle = texture_atlases.add(wave_layout);
    commands.spawn((
        SpriteBundle {
            texture: waves_sheet_handle.clone(),
            transform: Transform{
                translation: Vec3::new(FISHING_ROOM_X-90., FISHING_ROOM_Y-(WIN_H/2.)+100.,   930.),
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
        Wave
    ));



    let baits_sheet_handle: Handle<Image> = asset_server.load("lures/baits.png");
    let baits_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let baits_layout_len = baits_layout.textures.len();
    let baits_layout_handle = texture_atlases.add(baits_layout);
    
    commands.spawn((
        SpriteBundle {
            texture: baits_sheet_handle.clone(),
            transform: Transform{
                translation: Vec3::new(FISHING_ROOM_X+545., FISHING_ROOM_Y+255.,   930.),
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
        AnimationTimer::new(0.2), //number of different frames that we have
    ));


    let baits_sheet_handle: Handle<Image> = asset_server.load("lures/baits.png");
    let baits_layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 3, 1, None, None);
    let baits_layout_len = baits_layout.textures.len();
    let baits_layout_handle = texture_atlases.add(baits_layout);
    
    commands.spawn((
        SpriteBundle {
            texture: baits_sheet_handle.clone(),
            transform: Transform::from_xyz(FISHING_ROOM_X-90., FISHING_ROOM_Y-(WIN_H/2.)+100.,   930.),
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
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },Collision,
        AnimationFrameCount(baits_layout_len), //number of different frames that we have
        Lure::default(),
        Bobber::default(),
        
    ));

    /*let bobber_handle = asset_server.load("fishing_view/bobber.png");
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
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        Collision,
        Bobber::default(),
    ));*/

    //spawning in the lilypad
    let lily_sheet_handle: Handle<Image> = asset_server.load("fishing_view/lilypad.png");
    let deep_sheet_handle: Handle<Image> = asset_server.load("fishing_view/deep.png");
    commands.spawn((
        SpriteBundle {
            texture: lily_sheet_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(128.,128.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X+160., FISHING_ROOM_Y+100., 901.),
                ..default()
            },
            ..default()
        },
        Collision,
        PondObstruction,
        ObstType::Pad,
        InPond,
        FishLoc::Pond1,
    ));

    commands.spawn((
        SpriteBundle {
            texture: lily_sheet_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(128.,128.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X+160., FISHING_ROOM_Y+100., 901.),
                ..default()
            },
            ..default()
        },
        Collision,
        PondObstruction,
        ObstType::Pad,
        InPond,
        FishLoc::Pond2,
    ));

    commands.spawn((
        SpriteBundle {
            texture: deep_sheet_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(128.,128.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(FISHING_ROOM_X-360., FISHING_ROOM_Y-100., 901.),
                ..default()
            },
            ..default()
        },
        Collision,
        PondObstruction,
        ObstType::Fissure,
        InPond,
        FishLoc::Pond1,
    ));

}


fn move_fish(
    mut fish_details: Query<(&mut Fish, &mut Transform, &Species), (With<InPond>, With<Collision>, Without<PhysicsObject>, Without<PondObstruction>)>,
    mut obst_details: Query<(&mut Transform, &mut ObstType), (With<PondObstruction>, With<Collision>, With<InPond>, Without<FishDetails>)>,
    time: Res<Time>,
    mut config: ResMut<DirectionTimer>,
    //mut fish_direction: ResMut<FishBoundsDir>
) {
    let mut rng = rand::thread_rng();
    let mut obst_rng = rand::thread_rng();
    config.timer.tick(time.delta());
    //let mut obst_details = obst_details.single_mut();

    for (mut fish_details, mut fish_pos, fish_species) in fish_details.iter_mut() {
        //let mut rng = rand::thread_rng();             
        
        //move towards the obsticle on the x bounds

        
        if config.timer.finished() {


            let move_type: i32 = rng.gen_range(0..9); 
            let dir: i32 = obst_rng.gen_range(0..9);
            let mut move_skew: i32 = 0;
            //finding where to go in relation to the 
            //position in relation to x row
            for (obst_details, obstical_type) in obst_details.iter_mut(){
                //go back and account for margin of error done
                if *obstical_type == fish_species.obj_pref.0 {
                    //if fish_details.name == "catfish"{
                    move_skew = fish_species.obj_pref.1;
                    if obst_details.translation.x >= fish_pos.translation.x{
                        fish_details.change_x = Vec3::new(0.5, 0., 0.);
            
                    }
                    else if obst_details.translation.x < fish_pos.translation.x{
                        fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                    }
            
                    //move towards the obsticle on the right bounds
                    if obst_details.translation.y >= fish_pos.translation.y{
                        fish_details.change_y = Vec3::new(0., 0.5, 0.);
            
                    }
                    else if obst_details.translation.y < fish_pos.translation.y{
                        fish_details.change_y = Vec3::new(0.0, -0.5, 0.);
                    }
                    //}
                }
                // else if *obstical_type == ObstType::Pad{
                //     if fish_details.name == "bass"{
                //         if obst_details.translation.x >= fish_pos.translation.x{
                //             fish_details.change_x = Vec3::new(0.5, 0., 0.);
                
                //         }
                //         else if obst_details.translation.x < fish_pos.translation.x{
                //             fish_details.change_x = Vec3::new(-0.5, 0., 0.);
                //         }
                
                //         //move towards the obsticle on the right bounds
                //         if obst_details.translation.y >= fish_pos.translation.y{
                //             fish_details.change_y = Vec3::new(0., 0.5, 0.);
                
                //         }
                //         else if obst_details.translation.y < fish_pos.translation.y{
                //             fish_details.change_y = Vec3::new(0.0, -0.5, 0.);
                //         }
                //     }
                // }
                //for each collision object add a            
            }
            

            println!("timer finished");

            

            //println!("numer is {} {:?}", dir, fish_details.name);
            if move_type >= 4+move_skew{
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
                
            }
            else{
                println!("moving toward the object");
                //if it isnt moving in a random direction,
            }
        }
        
        //match it then set up the vector for the next tree seconds, keep the stuff about borders

        //println!("fish pos x {}, fish pos y {}", change_x, change_y);
        //CHANGE THESE TO CONSTANTS LATER!!!!!
        /*
        
//pub const FISHINGROOMX: f32 = 8960.;
//pub const FISHINGROOMY: f32 = 3600.;
        
         */


        let holdx: Vec3 = fish_pos.translation + fish_details.change_x;
        if (holdx.x) >= (-640. + 160.) && (holdx.x) <= (431. - 160.) {
            //println!("{:?}", fish_pos.translation);
            fish_pos.translation += fish_details.change_x;
        }
        else{
            // println!("fish: {:?} {:?}", fish_details.name, fish_details.id);
            // println!("holdx = {:?}", holdx);
        }
        let holdy: Vec3 = fish_pos.translation + fish_details.change_y;
        if (holdy.y) >= (-719.5-224. + 90.) && (holdy.y) <= (-719.5 + 360. - 90.) {
            //println!("{:?}", fish_pos.translation);
            fish_pos.translation += fish_details.change_y;
        }
        else{
            
            // println!("fish: {:?} {:?}", fish_details.name, fish_details.id);
            // println!("holdx = {:?}", holdy);
        }
        
    }
    //fish_pos.translation += change_y;
    //return (self.position.0 + x, self.position.1+y)
}

//function to poplulate 

fn add_fish(
    mut commands: Commands,
    mut fish_details: Query<(&mut Fish, &Species, &mut Transform, &mut Visibility, &FishLoc), (With<InPond>, With<Fish>, With<Collision>, With<MysteryFish>, Without<PhysicsObject>, Without<Bobber>)>,
    mut backgroundDeetsPond: Query<(&mut Transform), (Without<BeachScreen>, Without<Collision>, Without<PhysicsObject>, Without<Bobber>, With<PondScreen>, Without<MysteryFish>, Without<InPond>)>,
    mut backgroundDeetsBeach: Query<(&mut Transform), (With<BeachScreen>, Without<Collision>, Without<PhysicsObject>, Without<Bobber>, Without<PondScreen>, Without<MysteryFish>, Without<InPond>)>,
    mut fishes_phys: Query<(Entity, &mut Transform, &FishLoc), (With<PhysicsFish>, With<Fish>, With<Collision>, With<InPond>, With<PhysicsObject>, Without<Bobber>, Without<MysteryFish>, Without<PondScreen>, Without<BeachScreen>)>,
    mut obst_details: Query<(&mut Transform, &mut ObstType, &FishLoc), (With<PondObstruction>, With<Collision>, With<InPond>, Without<FishDetails>, Without<MysteryFish>, Without<PhysicsObject>, Without<Bobber>, Without<PondScreen>, Without<BeachScreen>)>,
    state: Res<State<FishingLocal>>,  
){
    let mut beachScr = backgroundDeetsBeach.single_mut(); 
    let mut pondScr = backgroundDeetsPond.single_mut(); 

    if state.eq(&FishingLocal::Pond1){
        for(mut fish, species, mut transform , mut visibility, loc) in &mut fish_details{
            if *loc == FishLoc::Pond1{
                transform.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                transform.translation = Vec3::new(-8000., -8000., 901.);
            }
        }
        for(mut obstPos, obstType, obstLoc) in &mut obst_details{
            if *obstLoc == FishLoc::Pond1{
                if *obstType == ObstType::Pad{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X+160., FISHING_ROOM_Y+100., 901.);
                }
                else if *obstType == ObstType::Fissure{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X-360., FISHING_ROOM_Y-100., 901.);
                }
            }
            else{
                obstPos.translation = Vec3::new(8000., 8000., 901.);
            }
        }
        for(mut ent, mut pos, location) in &mut fishes_phys{
            if *location == FishLoc::Pond1{
                pos.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                pos.translation = Vec3::new(-8000., -8000., 901.);
            }
        }
        //POND BEACH
        pondScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 900.);
        beachScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 100.);

    }
    if state.eq(&FishingLocal::Pond2){
        for(mut fish, species, mut transform , mut visibility, loc) in &mut fish_details{
            if *loc == FishLoc::Pond2{
                transform.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                transform.translation = Vec3::new(-8000., -8000., 901.);
            }
        }
        for(mut obstPos, obstType, obstLoc) in &mut obst_details{
            if *obstLoc == FishLoc::Pond2{
                if *obstType == ObstType::Pad{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X-160., FISHING_ROOM_Y+300., 901.);
                }
                else if *obstType == ObstType::Fissure{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X+260., FISHING_ROOM_Y, 901.);
                }
            }
            else{
                obstPos.translation = Vec3::new(8000., 8000., 901.);
            }
        }
        for(mut ent, mut pos, location) in &mut fishes_phys{
            if *location == FishLoc::Pond2{
                pos.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                pos.translation = Vec3::new(-8000., -8000., 901.);
            }
        }

        //POND BEACH
        pondScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 900.);
        beachScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 100.);
        
    }
    if state.eq(&FishingLocal::Beach){
        for(mut fish, species, mut transform , mut visibility, loc) in &mut fish_details{
            if *loc == FishLoc::Ocean{
                transform.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                transform.translation = Vec3::new(-8000., -8000., 901.);
            }
        }
        for(mut obstPos, obstType, obstLoc) in &mut obst_details{
            if *obstLoc == FishLoc::Ocean{
                if *obstType == ObstType::Pad{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X+160., FISHING_ROOM_Y+100., 901.);
                }
                else if *obstType == ObstType::Fissure{
                    obstPos.translation = Vec3::new(FISHING_ROOM_X-360., FISHING_ROOM_Y-100., 901.);
                }
            }
            else{
                obstPos.translation = Vec3::new(8000., 8000., 901.);
            }
        }
        for(mut ent, mut pos, location) in &mut fishes_phys{
            if *location == FishLoc::Ocean{
                pos.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 901.);
            }
            else{
                pos.translation = Vec3::new(-8000., -8000., 901.);
            }
        }
        
        //POND BEACH
        pondScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 100.);
        beachScr.translation = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 900.);
    }
    
    //check what fishing state youre going into, check if pond one move pond 1 fish in, move pond 2 fish out





}





fn fishPopulation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let cool_fish_handle: Handle<Image> = asset_server.load("fishing_view/awesome_fishy.png");
    
    commands.spawn((
        SpriteBundle {
            texture: cool_fish_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(320.,180.)),
                ..default()
            },
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(-8000., -8000., 901.),
                ..default()
            },
            ..default()
        },
        Fish{
            name: "bass",
            id: 0,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        InPond,
        BASS,
        Collision,
        MysteryFish,
        FishLoc::Ocean,
    ));


    let fish_bass_handle: Handle<Image> = asset_server.load("fish/bass.png");

    commands.spawn((
        SpriteBundle {
            texture: fish_bass_handle.clone(),
                sprite: Sprite {
                custom_size: Some(Vec2::new(100.,100.)),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(-8000., -8000., 901.),
                ..default()
            },
            ..default()
        },
        BASS,
        Fish{
            name: "Bass2",
            id: 2,
            is_caught: false,
            is_alive: true,
            touching_lure: false,
            length: 8.0,
            width: 5.0,
            weight: 2.0,
            time_of_day: (0, 12),
            weather: Weather::Sunny,
            depth: (0,5),
            //x, y, z
            position: (8320, 3960),
            change_x: Vec3::new(0.,0.,0.),
            change_y: Vec3::new(0.,0.,0.),
            //length, width, depth
            bounds: (FISHING_ROOM_X as i32+100, FISHING_ROOM_Y as i32 + 100),
            age: 6.0,
            hunger: 10.0
        },
        PhysicsObject{
            mass: 2.0,
            position: Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y, 0.),
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            forces: Forces::default()
        },
        InPond,
        Collision,
        PhysicsFish,
        FishLoc::Ocean,
    ));
    //this is working lets start adding stuff/ look at psuedo code.

    //pond section and river section. 
    println!("ITS 23!!! adding new fish now!!!");
    

}








fn fish_area_bobber(
    mut commands: Commands,
    mut fish_details: Query<(&mut Fish, &Species, &mut Transform, &mut Visibility), (With<InPond>, With<Fish>, With<Collision>, With<MysteryFish>, Without<PhysicsObject>, Without<Bobber>)>,
    mut bobber: Query<(&Transform, &Tile, Entity, &PhysicsObject, &mut Visibility), (With<Bobber>, With<PhysicsObject>, Without<Fish>, Without<MysteryFish>)>,
    mut fishes: Query<(Entity, &mut Fish, &Species, &mut PhysicsObject, &mut Transform, &mut Visibility), (With<PhysicsFish>, With<Fish>, With<Collision>, With<InPond>, With<PhysicsObject>, Without<Bobber>, Without<MysteryFish>)>, //add this in as the fish query, change the position of it at the end 
    mut exclamation: Query<(&mut Transform, &mut Visibility), (With<exclam_point>, Without<InPond>, Without<Bobber>, Without<PhysicsFish>)>,
    //mut fishes_physics: Query<(Entity, &Fish, &Species, &mut PhysicsObject), (With<Fish>, Without<Bobber>)>,
    weather: Res<WeatherState>,
    timer: Res<GameDayTimer>,
    mut prob_timer: ResMut<ProbTimer>,
    mut next_state: ResMut<NextState<FishingState>>,
    time: Res<Time>,
    mut config: ResMut<ExclamationTimer>,
    
) {
    //let (bob, tile) = bobber.single_mut();
    //let (bob, tile, mut bobber_vis) = bobber.single_mut();
    //let (mut exclam_transform, mut exclam_vis) = exclamation.single_mut();
    let (bob, tile,  bobber_entity_id, bobber_physics, mut bobber_vis) = bobber.single_mut();
    for (mut fish_details, fish_species, fish_pos, mut fish_vis) in fish_details.iter_mut() {
        let fish_pos_loc = fish_pos.translation;
        let bobber_position = bob.translation;
        
        //println!("fish {:?} {} x {} y \n bobber:  {} x {} y ", fishes_details.name, fish_pos.translation.x, fish_pos.translation.y, bobber_position.x, bobber_position.y);
        
        //let bobber_position = bob.translation;

        if fish_pos_loc.y - 180. / 2. > bobber_position.y + 50.
            || fish_pos_loc.y + 180. / 2. < bobber_position.y - 50.
            || fish_pos_loc.x + 320. / 2. < bobber_position.x - 50.
            || fish_pos_loc.x - 320. / 2. > bobber_position.x + 50.
        {
            //there is no hit
            fish_details.touching_lure = false;
            //println!("fish {:?}, {:?}", fishes_details.name, fish_pos.translation);
            //println!("no hit");
            continue;
        }
        fish_details.touching_lure = true;
        println!("fish {:?}, {:?}", fish_details.name, fish_pos.translation);
        println!("bobber hit");

        //let (entity_id, mut fishy_details, fish_species, mut fish_physics, mut fishy_transform, mut fishy_vis) = fishes.single_mut();

        

        //ERROR HERE
        if hook_fish((&mut fish_details, fish_species), &weather, &timer, &mut prob_timer, &time){
            for(entity_id, mut fishy_details, fish_species, mut fish_physics, mut fishy_transform, mut fishy_vis) in fishes.iter_mut(){
                if fishy_details.id == fish_details.id{ //fish number matches the other number of the caught fish
                    println!("FIRST: {:?}", fishy_transform.translation);
                    //println!("FIRST: {:?}", exclam_transform.translation);
                    fishy_transform.translation = bobber_position;
                    //exclam_transform.translation = bobber_position;
                    //exclam_transform.translation.y += 40.;
                    //*exclam_vis = Visibility::Visible;
                    

                    config.timer.tick(time.delta());

                    /*if config.timer.finished()
                    {
                        println!("hiding it");
                        *exclam_vis = Visibility::Hidden;
                    }*/
                    
                    //println!("SECOND: {:?}", exclam_transform.translation);
                    fish_physics.position = bobber_position;
                    //fishy_transform.translation = fish_physics.position.with_z(901.);
                    *bobber_vis = Visibility::Hidden; //yes
                    *fish_vis = Visibility::Hidden; //yes
                    *fishy_vis = Visibility::Visible;
                    fishy_details.is_caught = true;
                    //println!("FIRST: {:?}", fishy_transform.translation);
                    
                    println!("SECOND: {:?}", fishy_transform.translation);
                    //for (physics_object, mut transform) in objects.iter_mut() {
                        //transform.translation = physics_object.position.with_z(901.);
                    //unhide the actual fish
                    fish_physics.mass = fish_physics.mass + bobber_physics.mass; //yes
                    println!("fish name {:?}", fishy_details.name);
                    commands.entity(bobber_entity_id).remove::<Hooked>(); //yes
                    commands.entity(entity_id).insert(Hooked); //yes
                    next_state.set(FishingState::ReelingHooked);
                    break;
                }
                else{
                    println!("wrong fish this is a {:?}", fish_details.name);
                }
            }
            //next_state.set(FishingState::ReelingHooked); //yes
            //break; //yes
        }  
        
        
    }
}


fn spawn_mark(
    commands: Commands,
    position: Vec3,
){

}

fn fishing_transition (
    mut return_pos: ResMut<PlayerReturnPos>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut power_bar: Query<(&mut Transform, &mut PowerBar), (With<PowerBar>, Without<Camera>)>,
    mut rod: Query<&mut Transform, (With<FishingRod>, Without<Camera>, Without<PowerBar>)>,
) {
    let mut camera_transform = camera.single_mut();
    let (mut power_bar_transform, mut power) = power_bar.single_mut();
    let mut rod_transform = rod.single_mut();

    return_pos.position = camera_transform.translation;

    camera_transform.translation.x = FISHING_ROOM_X;
    camera_transform.translation.y = FISHING_ROOM_Y;
    //FISHING_ROOM_Y-308
    //spawn in powerbar
    //commands.spawn
    // power_bar_transform.translation.y = POWER_BAR_Y_OFFSET;
    // power_bar_info.power = 0.;

    //rd
    // rod_info.rotation = 0.;
    // rod_transform.rotation = Quat::from_rotation_z(rod_info.rotation);

    //new movmemnt system, rotation then space hold.
    //powerbar is space A, D are rotational
}

fn overworld_transition(
    mut camera: Query<&mut Transform, With<Camera>>,
    //mut power_bar: Query<(&mut Transform, &mut Power), With<Bar>>,
    return_pos: ResMut<PlayerReturnPos>,
) {
    let mut ct = camera.single_mut();
    //let (mut pb, mut power) = power_bar.single_mut();
    ct.translation = return_pos.position;

    //pb.translation.y = (POWER_BAR_Y_OFFSET);
    //power.meter = 0;
    //set powerbar back to 0
    //set rotation back to 0
}

fn power_bar_cast(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<FishingState>>,
    mut power_bar: Query<(&mut PowerBar, &mut Transform), With<PowerBar>>
) {
    let (mut power_bar_info, mut power_bar_transform) = power_bar.single_mut();

    if input.pressed(TUG) {
        // Increase power
        power_bar_info.power = power_bar_info.power + POWER_FILL_SPEED * time.delta_seconds();

        if power_bar_info.power >= MAX_POWER {
            // Max power reached, release
            power_bar_info.power = MAX_POWER;
            next_state.set(FishingState::Casting);
        }

        power_bar_transform.translation.y = POWER_BAR_Y_OFFSET + power_bar_info.power;
    } else if input.just_released(TUG) {
        // Manual release
        next_state.set(FishingState::Casting);
    } else {
        return;
    }
}

fn switch_rod (
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rod: Query<(&mut FishingRod, &mut Handle<Image>, &Transform), With<FishingRod>>,
    segments: Query<(Entity, &Mesh2dHandle), With<FishingRodSegment>>,
    mut player_inventory: Query<&mut PlayerInventory>
 ) {
    if !input.just_pressed(SWITCH_ROD) {
        return;
    }

    let (mut rod_info, mut rod_texture, rod_transform) = rod.single_mut();
    let mut inventory = player_inventory.single_mut();

    inventory.rod_index = if inventory.rod_index == inventory.rods.len() - 1 { 0 } else { inventory.rod_index + 1 };
    let current_rod = inventory.rods[inventory.rod_index].name;

    let new_type = match current_rod {
        "Default Rod" => &FishingRodType::NORMAL,
        "Surf Rod" => &FishingRodType::SURF,
        _ => &FishingRodType::NORMAL,
    };

    rod_info.rod_type = new_type;
    materials.remove(&rod_info.material);
    rod_info.material = materials.add(new_type.blank_color);
    *rod_texture = asset_server.load(new_type.texture);
    rod_info.tip_pos = (rod_transform.translation.xy() + new_type.length * PIXELS_PER_METER * Vec2::from_angle(rod_info.rotation)).extend(0.);

    // Remove old segments
    for (segment_id, mesh_handle) in segments.iter() {
        meshes.remove(mesh_handle.id());
        commands.entity(segment_id).despawn();
    }

    // Create new segments
    let new_segment_count: usize = (new_type.length / BENDING_RESOLUTION) as usize;
    rod_info.segments = Vec::with_capacity(new_segment_count);
    
    for i in (0..new_segment_count).rev() {
        let l = i as f32 * BENDING_RESOLUTION;
        let radius = new_type.thickness * l / new_type.length;
        let radius_pixels = (radius * 750.).max(1.);

        let entity = commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(radius_pixels, radius_pixels))),
                material: rod_info.material.clone(),
                ..default()
            },
            FishingRodSegment
        )).id();

        rod_info.segments.push(entity);
    }



}

fn switch_line (
    input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut line: Query<(&mut FishingLine, &mut Handle<ColorMaterial>), With<FishingLine>>,
    mut player_inventory: Query<&mut PlayerInventory>,
) {
    if !input.just_pressed(SWITCH_LINE) {
        return;
    }

    let mut inventory = player_inventory.single_mut();
    let (mut line_properties, line_material) = line.single_mut();
    
    inventory.line_index = if inventory.line_index == inventory.lines.len() - 1 { 0 } else { inventory.line_index + 1 };
    let current_line = inventory.lines[inventory.line_index].name;

    line_properties.line_type = match current_line {
        "FluoroCarbon Fishing Line" => &FishingLineType::FLUOROCARBON,
        "Braided Fishing Line" => &FishingLineType::BRAIDED,
        "Monofilament Fishing Line" => &FishingLineType::MONOFILILMENT,
        _ => &FishingLineType::MONOFILILMENT
    };

    let material = materials.get_mut(line_material.id()).unwrap();
    material.color = line_properties.line_type.color;
}

fn switch_bait (
    input: Res<ButtonInput<KeyCode>>,
    mut screen_lure: Query<&mut TextureAtlas, With<OnScreenLure>>,
    mut bait_lure: Query<(&mut PhysicsObject, &mut TextureAtlas), (With<Lure>, Without<OnScreenLure>)>,
    mut player_inventory: Query<&mut PlayerInventory>,
) {
    let mut inventory = player_inventory.single_mut();
    let mut screen_texture  = screen_lure.single_mut();
    let (mut bait_physics, mut bait_texture) = bait_lure.single_mut();

    if input.just_pressed(SWITCH_BAIT_NEXT) {
        inventory.lure_index = if inventory.lure_index == inventory.lures.len() - 1 { 0 } else { inventory.lure_index + 1 };
    } else if input.just_pressed(SWITCH_BAIT_PREV) {
        inventory.lure_index = if inventory.lure_index == 0 { inventory.lures.len() - 1 } else { inventory.lure_index - 1 };
    } else {
        return;
    }

    let new_bait = match inventory.lures[inventory.lure_index].name {
        "Ball Bait" => &Lure::BALL,
        "Frog Bait" => &Lure::FROG,
        "Swim Bait" => &Lure::FISH,
        _ => &Lure::BALL
    };
    
    *bait_physics = PhysicsObject::new(new_bait.mass, Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.), Vec3::ZERO, Vec3::ZERO, Forces::default());
    screen_texture.index = new_bait.texture_index;
    bait_texture.index = new_bait.texture_index;
}

fn begin_cast (
    mut commands: Commands,
    power_bar: Query<&PowerBar>,
    mut line: Query<&mut FishingLine>,
    mut bobber: Query<(Entity, &mut Visibility), With<Lure>>
) {
    let power_bar_info = power_bar.single();
    let mut line_info = line.single_mut();
    let (entity_id, mut bobber_visibililty) = bobber.single_mut(); 
    
    line_info.cast_distance = power_bar_info.power / MAX_POWER * MAX_CAST_DISTANCE;
    commands.entity(entity_id).insert(Hooked);
    *bobber_visibililty = Visibility::Visible;
}

fn is_done_reeling(
    mut commands: Commands,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<&FishingRod, With<FishingRod>>,
    mut casted_lure: Query<(Entity, &PhysicsObject), With<Hooked>>,
){
    let rod_info = rod.single();
    let (entity_id, lure_physics) = casted_lure.single_mut();

    let distance = (lure_physics.position - rod_info.tip_pos).length();

    if distance <= CATCH_MARGIN {
        commands.entity(entity_id).remove::<Hooked>();
        next_state.set(FishingState::Idle);
    }
}

fn is_fish_caught (
    mut commands: Commands,
    mut player_inventory: Query<&mut PlayerInventory>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<&FishingRod, With<FishingRod>>,
    mut hooked_object: Query<(Entity, &mut Fish, &mut PhysicsObject), With<Hooked>>,
) {
    let rod_info = rod.single();
    let (entity_id, mut fish_details, mut fish_physics) = hooked_object.single_mut();
    let mut inventory_info = player_inventory.single_mut();

    let distance = (fish_physics.position - rod_info.tip_pos).length();

    if distance < CATCH_MARGIN {
        fish_details.is_caught = true;
        inventory_info.coins += fish_details.weight as u32 * 2;

        // Reset fish for testing
        fish_physics.position = Vec3::new(FISHING_ROOM_X, FISHING_ROOM_Y + 100., 0.);
        fish_physics.velocity = Vec3::new(0., 0., 0.);
        fish_physics.forces = Forces::default();
        fish_details.is_caught = false;
        fish_details.hooked_fish();
        commands.entity(entity_id).remove::<Hooked>();

        next_state.set(FishingState::Idle);
    }
}

fn rod_rotate(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut fishing_rod: Query<(&mut FishingRod, &mut Transform), With<FishingRod>>
) {
    let mut direction = 0.;
    
    if input.pressed(ROTATE_ROD_COUNTERLCOCKWISE) {
        direction += 1.;
    }
    
    if input.pressed(ROTATE_ROD_CLOCKWISE) {
        direction += -1.;
    }
    
    let (mut rod_info, mut rod_transform) = fishing_rod.single_mut();
    let new_rotation = rod_info.rotation + direction * ROD_ROTATION_SPEED * time.delta_seconds();
    rod_info.rotation = new_rotation.clamp(ROD_MIN_ROTATION, ROD_MAX_ROTATION);
    rod_transform.rotation = Quat::from_rotation_z(rod_info.rotation);
}

fn cast_line (
    time: Res<Time>,
    mut next_state: ResMut<NextState<FishingState>>,
    rod: Query<&FishingRod, With<FishingRod>>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<(&mut Transform, &mut PhysicsObject), (With<Lure>, Without<FishingRod>)>,
    mut splash: Query<(&mut Splash, &mut Visibility), With<Splash>>,
) {
    let rod_info = rod.single();
    let mut line_info = line.single_mut();
    let (mut bobber_transform, mut bobber_physics) = bobber.single_mut();
    let (mut splash_info, mut splash_visibility) = splash.single_mut();
    let angle_vector = Vec2::from_angle(rod_info.rotation).extend(0.);
    
    line_info.length = (line_info.length + CASTING_SPEED * time.delta_seconds()).min(line_info.cast_distance);
    line_info.end = rod_info.tip_pos + line_info.length * angle_vector;

    if line_info.length == line_info.cast_distance {
        // Cast finished
        line_info.length = line_info.cast_distance;
        splash_info.position = line_info.end.with_z(902.);
        *splash_visibility = Visibility::Visible;
        next_state.set(FishingState::ReelingUnhooked);
    }

    //setting the position of the bobber along with the physics location of the bobber.
    //also make sure that we are setting the bobber to be a hooked object
    bobber_physics.position = line_info.end.with_z(950.);
    bobber_transform.translation = line_info.end.with_z(950.);
}

fn animate_fishing_line (
    rod: Query<&FishingRod, With<FishingRod>>,
    hooked_fish: Query<(&Species, &PhysicsObject), (With<Fish>, With<Hooked>)>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut bobber: Query<&PhysicsObject, With<Lure>>,
    state: Res<State<FishingState>>
) {
    let rod_info = rod.single();
    let mut line_info = line.single_mut();

    line_info.start = rod_info.tip_pos;

    match state.get() {
        FishingState::Idle => {
            line_info.end = line_info.start;
        },
        FishingState::ReelingHooked => {
            let (fish_species, fish_physics) = hooked_fish.single();
            let fish_offset = fish_species.hook_pos.rotate(Vec2::from_angle(fish_physics.rotation.z));
            let fish_pos = fish_physics.position + fish_offset.extend(0.);
            line_info.end = fish_pos;
        },
        FishingState::ReelingUnhooked => {
            let bobber_physics = bobber.single_mut();
            line_info.end = bobber_physics.position;
        },
        _ => {}
    }
}

fn reset_interface (
    mut power_bar: Query<&mut PowerBar>,
    mut line: Query<&mut FishingLine, With<FishingLine>>,
    mut splash: Query<&mut TextureAtlas, With<Splash>>,
    mut bobber: Query<(&mut PhysicsObject, &mut Visibility), With<Lure>>
) {
    let mut power_bar_info = power_bar.single_mut();
    let mut line_info = line.single_mut();
    let mut splash_texture = splash.single_mut();
    let (mut bobber_physics, mut bobber_visibility) = bobber.single_mut();
    
    line_info.length = 0.;
    line_info.start = Vec3::ZERO;
    line_info.end = Vec3::ZERO;
    bobber_physics.velocity = Vec3::ZERO;
    bobber_physics.forces = Forces::default();
    *bobber_visibility = Visibility::Hidden;
    splash_texture.index = 0;
    power_bar_info.power = 0.;
}

pub fn move_physics_objects (
    mut objects: Query<(&PhysicsObject, &mut Transform), With<PhysicsObject>>
) {
    for (physics_object, mut transform) in objects.iter_mut() {
        transform.translation = physics_object.position.with_z(transform.translation.z);
        transform.rotation = Quat::from_rotation_z(physics_object.rotation.z);
    }
}

fn draw_fishing_line (
    mut meshes: ResMut<Assets<Mesh>>,
    mut line: Query<(&mut Transform, &mut Mesh2dHandle, &mut FishingLine), (With<FishingLine>, Without<FishingRod>)>,
) {
    let (mut line_transform, mut line_mesh, line_info) = line.single_mut();

    let pos_delta = line_info.end - line_info.start;
    let line_length = pos_delta.with_z(0.).length();

    let line_pos = (line_info.start + line_info.end) / 2.;
    let line_rotation =  f32::atan2(pos_delta.y, pos_delta.x) + PI / 2.;

    // Draw fishing line
    line_transform.translation = Vec3::new(line_pos.x, line_pos.y, line_transform.translation.z);
    line_transform.rotation = Quat::from_rotation_z(line_rotation);

    let width = if line_length == 0. { 0. } else { FishingLine::WIDTH };

    meshes.remove(line_mesh.id());
    *line_mesh = Mesh2dHandle(meshes.add(Rectangle::new(width, line_length)));
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
    objects: Query<(&PhysicsObject, &Fish), (With<PhysicsObject>, With<Fish>)>, 
    mut wave: Query<(&mut TextureAtlas, &mut Transform, &mut Visibility), With<Wave>>
) {
    let (mut wave_texture, mut wave_transform, mut wave_visibility) = wave.single_mut();
    
    // Currently only supports one object and only supports fish
    for (physics_object, fish) in objects.iter() {
        let magnitude = physics_object.forces.water.length();
    
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
        if fish.is_caught{
            *wave_visibility = Visibility::Visible;
        
            wave_transform.translation = physics_object.position.with_z(902.);
            wave_transform.rotation = Quat::from_rotation_z(f32::atan2(physics_object.forces.water.y, physics_object.forces.water.x) - PI / 2.);
        }
        return;
    }
}

