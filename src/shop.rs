use bevy::prelude::*;
use crate::gameday::*;
use crate::inventory::*;
use crate::map::*;
use crate::player::*;
use crate::resources::*;

#[derive(Component)]
struct ShopEntrance;

#[derive(Resource)]
pub struct HoverEntity(pub Entity);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShopingMode {
    #[default]
    Overworld,
    Shop
}

#[derive(PartialEq, Default, Clone)]
pub enum ItemType{
    #[default]
    LINE,
    LURE
}

impl ShopingMode{
    pub fn next(&self) -> Self {
        match self {
            ShopingMode::Overworld => ShopingMode::Shop,
            ShopingMode::Shop => ShopingMode::Overworld,
        }
    }
}

#[derive(Resource)]
pub struct ShopState {
    pub is_open: bool,
}

#[derive(Component, Default, Clone)]
pub struct ShopItem {
    pub name: &'static str,
    pub price: u32,
    pub is_bought: bool,
    pub index: usize,
    pub item_type: ItemType,
}

impl ShopItem {
    pub const fn new(name: &'static str, price: u32, is_bought: bool, index: usize, item_type: ItemType) -> Self {
        Self { name,  price, is_bought, index, item_type}
    }
}




#[derive(Resource)]
struct SelectedShopItem {
    index: usize,
}

#[derive(Component)]
struct SoldSprite;

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
            items: Vec::from([ShopItem::new("Bobber", 0, true, 0, ItemType::LURE), 
            ShopItem::new("Monofilament Fishing Line", 0, true, 0, ItemType::LINE)]),
            lures: Vec::from([ShopItem::new("Bobber", 0, true, 0, ItemType::LURE)]),
            lines: Vec::from([ShopItem::new("Monofilament Fishing Line", 0, true, 0, ItemType::LINE)]),
            lure_index: 0,
            line_index: 0,
        },
));
}
fn spawn_shop(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let shop = asset_server.load("tiles/shop.png");
    commands.spawn((
        SpriteBundle {
            texture: shop,
            transform: Transform {
                translation: Vec3::new(1024., 1., 1.),
                ..default()
            },
            ..default()
        },
        Tile::SHOP,
        Collision,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(1024., -96., 0.)),
        ShopEntrance,
        Tile::new("shopEntrance", true, Vec2::new(64., 64.))
    ));
    commands.spawn(SpriteBundle{
        texture: asset_server.load("shop/inventory.png"),
        transform: Transform::from_xyz(3000., 3000., 1.),
       ..default()
    });
    commands.spawn((
        ShopItem {
            name: "Swim Bait",
            price: 50,
            is_bought: false,
            index: 2,
            item_type: ItemType::LURE,
        },
    ));
    commands.spawn(
        ShopItem{
            name: "Frog Bait",
            price: 20,
            is_bought: false,
            index: 1,
            item_type: ItemType::LURE,
        }
    );
    commands.spawn(
        ShopItem{
            name: "Surf Fishing Rod",
            is_bought: false,
            price: 150,
            ..default()
        },
    );
    commands.spawn(
        ShopItem{
            name: "Braided Fishing Line",
            is_bought: false,
            price: 50,
            item_type: ItemType::LINE,
            ..default()
        }
    );
    commands.spawn(
        ShopItem{
            name: "FluoroCarbon Fishing Line",
            is_bought: false,
            price: 25,
            item_type: ItemType::LINE,
            ..default()
        }
    );
    commands.spawn(
        ShopItem{
            name: "Fish",
            price: 500,
            is_bought: false,
            ..default()
        }
    );

    let hover_texture = asset_server.load("shop/hover.png");
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

    
    let swim_bait_texture = asset_server.load("lures/swim_bait.png");
    let frog_bait_texture = asset_server.load("lures/frog_bait.png");
    let surf_rod_texture = asset_server.load("rods/surf.png");
    let monofil_texture = asset_server.load("lines/monofilament.png");
    let braided_line_texture = asset_server.load("lines/braided.png");
    let fish_texture = asset_server.load("fish/bass.png");
    let sold_texture: Handle<Image> = asset_server.load("shop/sold.png");

    //slot positions
    let slot_positions = [
        Vec3::new(2620., 2820., 2.),
        Vec3::new(3000., 2820., 2.),
        Vec3::new(3400., 2820., 2.),
        Vec3::new(2620., 3100., 2.),
        Vec3::new(3000., 3100., 2.),
        Vec3::new(3400., 3100., 2.),
    ];


    let font: Handle<Font> = asset_server.load("fonts/pixel.ttf");


    for (i, (entity, item)) in shop_items.iter().enumerate() {
        if let Some(&position) = slot_positions.get(i) {
            let mut position_sold = position;
            position_sold.z += 1 as f32;
            position_sold.y += 30 as f32;
            let texture = match item.name {
                "Swim Bait" => swim_bait_texture.clone(),
                "Frog Bait" => frog_bait_texture.clone(),
                "Surf Fishing Rod" => surf_rod_texture.clone(),
                "FluoroCarbon Fishing Line" => monofil_texture.clone(),
                "Braided Fishing Line" => braided_line_texture.clone(),
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

                    parent.spawn(SpriteBundle{
                        texture: sold_texture.clone(),
                        transform: Transform::from_translation(position_sold),
                        visibility: Visibility::Visible,
                        ..Default::default()
                    });
                    
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            item.name,
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

            commands.spawn((
                SpriteBundle {
                texture: sold_texture.clone(),
                transform: Transform::from_translation(position_sold),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            SoldSprite,
            ));
        } else {
            println!("No available slots");
        }
        }
    }



fn check_shop_entrance(
    mut player_query: Query<(&mut Transform, &mut PlayerDirection, &mut Location, &Animation, &mut InputStack), With<Player>>,
    entrance_query: Query<(&Transform, &Tile), (With<ShopEntrance>, Without<Player>, Without<Camera>)>,
    time_of_day: Res<GameDayTimer>,
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
    mut sold_spite: Query<&mut Visibility, With<SoldSprite>>,
) {
    if !shop_state.is_open {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) { // Use Enter key to purchase
        println!("Attempting to purchase");
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            let mut items: Vec<_> = shop_items.iter().collect();
            // let mut sold_sprites:Vec<_> = sold_spite.iter().collect();
            if let Some(mut shop_item) = shop_items.iter_mut().nth(selected_item.index) {
                if inventory.coins >= shop_item.price && !shop_item.is_bought{
                    inventory.coins -= shop_item.price;
                    inventory.items.push(shop_item.clone());
                    if shop_item.item_type == ItemType::LURE {
                        inventory.lures.push(shop_item.clone());
                    }
                    else if shop_item.item_type == ItemType::LINE{
                        inventory.lines.push(shop_item.clone());
                    }
                    shop_item.is_bought = true;

                    if let Some(mut sold_sprite_visibility) = sold_spite.iter_mut().nth(selected_item.index) {
                        *sold_sprite_visibility  = Visibility::Visible;
                    }
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

pub fn shop_open(shop_state: Res<ShopState>) -> bool {
    shop_state.is_open
}

