use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

const TITLE: &str = "gameMap";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

//how big is each tile
const TILE_SIZE: u32 = 64;

#[derive(Component)]
struct GrassTile;

fn main() {
    App::new()
    //making resources
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
        .run();
    //set up of systems
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    //get the texture atlas ready
) {
    //loading in the birds sprite sheet
//
//      LOADING IN THE SPRITE SHEETS AND SETTING UP THE BACKGROUND FOR THE SCROLLING ROOMS. 
// 
//
    let grass_sheet_handle = asset_server.load("ground_sheet.png");
    let grass_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE),
    6, 5, None, None);
                //IF THE SPRITE SHEET FOR BACKGROUND IS MADE LARGER, THIS NEEDS TO GROW

    let grass_layout_len = grass_layout.textures.len();
    println!("grasslayout.len {}", grass_layout_len);
    let grass_layout_handle = texture_atlases.add(grass_layout);

    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - (TILE_SIZE as f32) / 2.;
    let y_bound = WIN_H / 2. - (TILE_SIZE as f32) / 2.;
    println!("window w {}", (-WIN_H));
//SO FAR I HAVE IT CORRECTLY Printing out The sprite sheet, need to make it go up and look more times. 
    
    let mut j = 0.;
    //this is the offset
    //let mut t = Vec3::new(-x_bound, y_bound, 0.);
    
    println!("y bound is {}", y_bound);
    while (j as f32) * (TILE_SIZE as f32) < WIN_H{
        //println!("rinning j");
        let mut i = 0;
        let mut t = Vec3::new(-x_bound, (TILE_SIZE as f32*j)+(-y_bound), 0.);
        println!("spawning at {}", (TILE_SIZE as f32*j)+y_bound);
        while (i as f32) * (TILE_SIZE as f32) < WIN_W {
            //println!("rinning i");
            //IF THE SPRITE SHEET FOR BACKGROUND IS MADE LARGER, THIS NEEDS TO GROW
            let mut random_index = rng.gen_range(0..29);
            if random_index % 2 != 0{
                random_index-=1;
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
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
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
                    index: (random_index+1),
                    layout: grass_layout_handle.clone(),
                },
                GrassTile,
            ));
            //
            //do this twice uhhhhhh....

            i += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
            println!("{}", t);
        }
        j += 1.0;
        
    }
}
