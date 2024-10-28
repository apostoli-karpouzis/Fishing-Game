use bevy::{input::keyboard::KeyboardInput, prelude::*};
use crate::{
    map::{Collision, Tile}, resources, Animation, InputStack, Location, Player, PlayerDirection, PLAYER_HEIGHT, PLAYER_WIDTH,
};
use crate::resources::*;

#[derive(Component)]
struct ShopEntrance;

#[derive(Component)]
struct ShopItem {
    name: String,
    price: u32,
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
        .add_systems(Update, (check_shop_entrance, handle_purchase, update_selected_item, exit_shop))
        .insert_resource(SelectedShopItem {index: 0})
        .insert_resource(OriginalCameraPosition(Vec3::ZERO));

    }
}

fn setup_player_inventory(mut commands: Commands) {
    commands.spawn((
        PlayerInventory{
            coins: 0,
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
        },
    ));
    commands.spawn((
        ShopItem{
            name: "Bait Box".to_string(),
            price: 20,
        }
    ));
    commands.spawn((
        ShopItem{
            name: "Sunglasses".to_string(),
            price: 100,
        },
    ));
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
    shop_items: Query<(Entity, &ShopItem)>,
    mut player_inventory: Query<&mut PlayerInventory>,
    selected_item: Res<SelectedShopItem>,
) {
    if !shop_state.is_open {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) { // Use Enter key to purchase
        println!("Attempting to purchase");
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            let items: Vec<_> = shop_items.iter().collect();
            if let Some((item_entity, shop_item)) = items.get(selected_item.index) {
                if inventory.coins >= shop_item.price {
                    inventory.coins -= shop_item.price;
                    inventory.items.push(shop_item.name.clone());
                    println!("Purchased: {}", shop_item.name);
                } else {
                    println!("Not enough coins to purchase {}", shop_item.name);
                }
            }
        }
    }
}

fn update_selected_item(
    input: Res<ButtonInput<KeyCode>>,
    mut selected_item:ResMut<SelectedShopItem>,
    shop_items: Query<&ShopItem>,
    shop_state: Res<ShopState>,
){
    if !shop_state.is_open {
        return;
    }

    let item_count = shop_items.iter().count();

    if input.just_pressed(KeyCode::ArrowUp){
        selected_item.index = (selected_item.index + item_count - 1) % item_count;
        println!("Selected: {}", selected_item.index);
    }
    if input.just_pressed(KeyCode::ArrowDown){
        selected_item.index = (selected_item.index + 1) % item_count;
        println!("Selected: {}", selected_item.index);
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


