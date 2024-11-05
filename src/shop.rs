use std::path::is_separator;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::{
    map::{Collision, Tile}, resources, Animation, InputStack, Location, Player, PlayerDirection, PLAYER_HEIGHT, PLAYER_WIDTH,
};
use crate::resources::*;

#[derive(Component)]
struct ShopEntrance;

#[derive(Resource)]
pub struct HoverEntity(pub Entity);

#[derive(Component)]
struct ShopItem {
    name: String,
    price: u32,
    is_bought: bool,
}

#[derive(Resource)]
struct SelectedShopItem {
    index: usize,
}

#[derive(Resource)]
struct OriginalCameraPosition(Vec3);

pub struct ShopPlugin;
impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_shop, setup_player_inventory))
        .add_systems(Update, (check_shop_entrance, handle_purchase, update_selected_item, exit_shop, display_shop_items))
        .insert_resource(SelectedShopItem {index: 0})
        .insert_resource(OriginalCameraPosition(Vec3::ZERO));

    }
}

fn setup_player_inventory(mut commands: Commands) {
    commands.spawn((
        PlayerInventory{
            coins: 1000,
            items: Vec::new(),
        },
));
}
fn spawn_shop(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let shop = asset_server.load("shop.png");
    commands.spawn((
        SpriteBundle {
            texture: shop,
            transform: Transform {
                translation: Vec3::new(1024., 1., 1.),
                ..default()
            },
            ..default()
        },
        Tile::Shop,
        Collision,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(1024., -96., 0.)),
        ShopEntrance,
        Tile::new("shopEntrance", true, Vec2::new(64., 64.))
    ));
    commands.spawn(SpriteBundle{
        texture: asset_server.load("inventory.png"),
        transform: Transform::from_xyz(3000., 3000., 1.),
       ..default()
    });
    commands.spawn((
        ShopItem {
            name: "Fishing Rod".to_string(),
            price: 50,
            is_bought: true
        },
    ));
    commands.spawn((
        ShopItem{
            name: "Lure".to_string(),
            price: 20,
            is_bought: false
        }
    ));
    commands.spawn((
        ShopItem{
            name: "Rocks".to_string(),
            price: 1000,
            is_bought: false
        },
    ));
    commands.spawn((
        ShopItem{
            name: "Tree".to_string(),
            price: 20,
            is_bought: false
        }
    ));
    commands.spawn((
        ShopItem{
            name: "Water".to_string(),
            price: 240,
            is_bought: false
        }
    ));
    commands.spawn((
        ShopItem{
            name: "Fish".to_string(),
            price: 500,
            is_bought: false
        }
    ));

    let hover_texture = asset_server.load("hover.png");
    let hover_entity = commands.spawn(SpriteBundle {
        texture: hover_texture,
        transform: Transform {
            translation: Vec3::new(2620., 2860., 3.0), 
            ..Default::default()
        },
        ..Default::default()
    }).id();

    
    commands.insert_resource(HoverEntity(hover_entity));
    
}

fn display_shop_items(
    mut commands: Commands,
    shop_items: Query<(Entity, &ShopItem)>,
    asset_server: Res<AssetServer>,
    shop_state: Res<ShopState>,
) {
    if !shop_state.is_open {
        return;
    }

    
    let fishing_rod_texture = asset_server.load("fishingRod.png");
    let lure_texture = asset_server.load("pixil-frame-0 (64).png");
    let rocks_texture = asset_server.load("Rocks.png");
    let tree_texture = asset_server.load("tiles/tree.png");
    let water_texture = asset_server.load("tiles/water.png");
    let fish_texture = asset_server.load("fish/bass.png");

    //slot positions
    let slot_positions = [
        Vec3::new(2620., 2820., 2.),
        Vec3::new(3000., 2820., 2.),
        Vec3::new(3400., 2820., 2.),
        Vec3::new(2620., 3100., 2.),
        Vec3::new(3000., 3100., 2.),
        Vec3::new(3400., 3100., 2.),
    ];


    let font: Handle<Font> = asset_server.load("pixel.ttf");


    for (i, (entity, item)) in shop_items.iter().enumerate() {
        if let Some(&position) = slot_positions.get(i) {
            let texture = match item.name.as_str() {
                "Fishing Rod" => fishing_rod_texture.clone(),
                "Lure" => lure_texture.clone(),
                "Rocks" => rocks_texture.clone(),
                "Tree" => tree_texture.clone(),
                "Water" => water_texture.clone(),
                "Fish" => fish_texture.clone(),
                _ => {
                    println!("No texture found for item: {}", item.name);
                    continue;
                }
            };

            commands.entity(entity)
                .insert(SpriteBundle {
                    texture,
                    transform: Transform::from_translation(position),
                    ..Default::default()
                })
                .with_children(|parent| {
                    
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            item.name.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform::from_translation(Vec3::new(0.0, 160.0, 1.0)),
                        ..Default::default()
                    });

                    
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("${}", item.price),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform::from_translation(Vec3::new(0.0, -90.0, 1.0)),
                        ..Default::default()
                    });
                });
        } else {
            println!("No available slots");
        }
    }
}


fn check_shop_entrance(
    mut player_query: Query<(&mut Transform, &mut PlayerDirection, &mut Location, &Animation, &mut InputStack), With<Player>>,
    entrance_query: Query<(&Transform, &Tile), (With<ShopEntrance>, Without<Player>, Without<Camera>)>,
    time_of_day: Res<resources::GameDayTimer>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>, Without<ShopEntrance>)> ,
    mut shop_state: ResMut<ShopState>,
    mut original_camera_pos: ResMut<OriginalCameraPosition>,
){
    let (mut pt, mut pd, mut pl, _pa, mut pi ) = player_query.single_mut();
    let (e_tran,e_tile) = entrance_query.single();
    if pt.translation.y - PLAYER_HEIGHT / 2. > e_tran.translation.y + e_tile.hitbox.y / 2.
        || pt.translation.y + PLAYER_HEIGHT / 2. < e_tran.translation.y - e_tile.hitbox.y / 2. 
        || pt.translation.x + PLAYER_WIDTH / 2. < e_tran.translation.x - e_tile.hitbox.x / 2. 
        || pt.translation.x - PLAYER_WIDTH / 2. > e_tran.translation.x + e_tile.hitbox.x / 2.
    {
        return;
    }else{
        if *pd == PlayerDirection::Back 
        && time_of_day.hour < 21 && !shop_state.is_open{
            let mut camera = camera_query.single_mut();
            original_camera_pos.0 = camera.translation;
            println!("{}", original_camera_pos.0);
            let new_position = Vec3::new(3000.0, 3000.0, camera.translation.z);
            camera.translation = new_position;
            shop_state.is_open = true;
            println!("Shop open");
        }
    }
}

fn handle_purchase(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    shop_state: Res<ShopState>,
    mut shop_items: Query<&mut ShopItem>,
    mut player_inventory: Query<&mut PlayerInventory>,
    selected_item: Res<SelectedShopItem>,
) {
    if !shop_state.is_open {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) { // Use Enter key to purchase
        println!("Attempting to purchase");
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            let mut items: Vec<_> = shop_items.iter().collect();
            if let Some(mut shop_item) = shop_items.iter_mut().nth(selected_item.index) {
                if inventory.coins >= shop_item.price && !shop_item.is_bought{
                    inventory.coins -= shop_item.price;
                    inventory.items.push(shop_item.name.clone());
                    shop_item.is_bought = true;
                    println!("Purchased: {}", shop_item.name);
                } else if inventory.coins < shop_item.price{
                    println!("Not enough coins to purchase {}", shop_item.name);
                }
                else {
                    println!("Shop item has already been purchased");
                }
            }
        }
    }
}

fn update_selected_item(
    input: Res<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SelectedShopItem>,
    shop_items: Query<&ShopItem>,
    shop_state: Res<ShopState>,
    hover_entity: Res<HoverEntity>,
    mut hover_query: Query<&mut Transform>,
) {
    if !shop_state.is_open {
        return;
    }

    let cols = 3;
    let rows = 2;

    let current_row = selected_item.index / cols;
    let current_col = selected_item.index % cols;

    if input.just_pressed(KeyCode::ArrowUp) {
        let new_row = if current_row == 0 { rows - 1 } else { current_row - 1 };
        selected_item.index = new_row * cols + current_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let new_row = if current_row == rows - 1 { 0 } else { current_row + 1 };
        selected_item.index = new_row * cols + current_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowLeft) {
        let new_col = if current_col == 0 { cols - 1 } else { current_col - 1 };
        selected_item.index = current_row * cols + new_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowRight) {
        let new_col = if current_col == cols - 1 { 0 } else { current_col + 1 };
        selected_item.index = current_row * cols + new_col;
        println!("Selected: {}", selected_item.index);
    }

    // Define slot positions
    let slot_positions = [
        Vec3::new(2620., 2820., 2.),
        Vec3::new(3000., 2820., 2.),
        Vec3::new(3400., 2820., 2.),
        Vec3::new(2620., 3100., 2.),
        Vec3::new(3000., 3100., 2.),
        Vec3::new(3400., 3100., 2.),
    ];

    if let Some(&position) = slot_positions.get(selected_item.index) {
        if let Ok(mut hover_transform) = hover_query.get_mut(hover_entity.0) {
            let adjusted_x = if position == Vec3::new(3000., 2820., 2.) || position == Vec3::new(3000., 3100., 2.) {
                position.x + 2.0
            } else {
                position.x - 10.0
            };

            hover_transform.translation = Vec3::new(adjusted_x, position.y + 42.0, position.z + 1.0);
        }
    }
}

fn exit_shop (
    input: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    original_camera_pos: Res<OriginalCameraPosition>,
    mut player_query: Query<(&mut Transform, &mut PlayerDirection, &mut Location, &Animation, &mut InputStack), (With<Player>, Without<Camera>)>
){
    if input.just_pressed(KeyCode::Escape) && shop_state.is_open {
        let mut camera = camera_query.single_mut();
        let (mut pt,mut pd, _pl, _pa, _pi) = player_query.single_mut();
        pt.translation = Vec3::new(1024., -180., 1.);
        *pd = PlayerDirection::Front;
        camera.translation = original_camera_pos.0;
        shop_state.is_open = false;
        println!("Shop closed");
    }
}


