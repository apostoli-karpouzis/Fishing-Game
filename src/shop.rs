use crate::gameday::*;
use crate::interface::CurrentInterface;
use crate::inventory::*;
use crate::map::*;
use crate::player::*;
use crate::resources::PlayerReturnPos;
use bevy::prelude::*;

pub const SHOP_CENTER: Vec2 = Map::get_area_center(1, -2);
pub const SHOP_X: f32 = SHOP_CENTER.x;
pub const SHOP_Y: f32 = SHOP_CENTER.y;

#[derive(Component)]
struct ShopEntrance;

#[derive(Resource)]
pub struct HoverEntity(pub Entity);

#[derive(PartialEq, Clone)]
pub enum ItemType {
    ROD,
    LINE,
    LURE,
    COSMETIC,
}

#[derive(Component, Clone)]
pub struct ShopItem {
    pub name: &'static str,
    pub price: u32,
    pub is_bought: bool,
    pub index: usize,
    pub item_type: ItemType,
}

impl ShopItem {
    pub const fn new(
        name: &'static str,
        price: u32,
        is_bought: bool,
        index: usize,
        item_type: ItemType,
    ) -> Self {
        Self {
            name,
            price,
            is_bought,
            index,
            item_type,
        }
    }
}

#[derive(Resource)]
struct SelectedShopItem {
    index: usize,
}

#[derive(Component)]
struct SoldSprite;

pub struct ShopPlugin;
impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_shop, setup_player_inventory))
            .insert_resource(SelectedShopItem { index: 0 })
            .add_systems(
                Update,
                check_shop_entrance.run_if(in_state(CurrentInterface::Overworld)),
            )
            .add_systems(
                Update,
                (handle_purchase, update_selected_item, exit_shop)
                    .run_if(in_state(CurrentInterface::Shop)),
            )
            .add_systems(
                OnTransition {
                    exited: CurrentInterface::Overworld,
                    entered: CurrentInterface::Shop,
                },
                display_shop_items,
            );
    }
}

fn setup_player_inventory(mut commands: Commands) {
    commands.spawn((PlayerInventory {
        coins: 1000,
        items: Vec::from([
            ShopItem::new("Default Rod", 0, true, 0, ItemType::ROD),
            ShopItem::new("Bobber", 0, true, 0, ItemType::LURE),
            ShopItem::new("Monofilament Fishing Line", 0, true, 0, ItemType::LINE),
        ]),
        rods: Vec::from([ShopItem::new("Default Rod", 0, true, 0, ItemType::ROD)]),
        lures: Vec::from([ShopItem::new("Bobber", 0, true, 0, ItemType::LURE)]),
        lines: Vec::from([ShopItem::new(
            "Monofilament Fishing Line",
            0,
            true,
            0,
            ItemType::LINE,
        )]),
        cosmetics: Vec::new(),
        rod_index: 0,
        lure_index: 0,
        line_index: 0,
    },));
}
fn spawn_shop(asset_server: Res<AssetServer>, mut commands: Commands) {
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
        Tile::new("shopEntrance", true, Vec2::new(64., 64.)),
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("shop/inventory.png"),
        transform: Transform::from_xyz(SHOP_X, SHOP_Y, 1.),
        ..default()
    });
    commands.spawn((ShopItem {
        name: "Swim Bait",
        price: 50,
        is_bought: false,
        index: 2,
        item_type: ItemType::LURE,
    },));
    commands.spawn(ShopItem {
        name: "Frog Bait",
        price: 20,
        is_bought: false,
        index: 1,
        item_type: ItemType::LURE,
    });
    commands.spawn(ShopItem {
        name: "Surf Rod",
        is_bought: false,
        price: 150,
        index: 3,
        item_type: ItemType::ROD,
    });
    commands.spawn(ShopItem {
        name: "Braided Fishing Line",
        is_bought: false,
        price: 50,
        index: 0,
        item_type: ItemType::LINE,
    });
    commands.spawn(ShopItem {
        name: "FluoroCarbon Fishing Line",
        is_bought: false,
        price: 25,
        index: 0,
        item_type: ItemType::LINE,
    });
    commands.spawn(ShopItem {
        name: "Polarized Sun Glasses",
        is_bought: false,
        price: 100,
        index: 0,
        item_type: ItemType::COSMETIC,
    });

    let hover_texture = asset_server.load("shop/hover.png");
    let hover_entity = commands
        .spawn(SpriteBundle {
            texture: hover_texture,
            transform: Transform {
                translation: Vec3::new(SHOP_X - 380., SHOP_Y - 140., 3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.insert_resource(HoverEntity(hover_entity));
}

fn display_shop_items(
    mut commands: Commands,
    shop_items: Query<(Entity, &ShopItem)>,
    asset_server: Res<AssetServer>,
) {
    let swim_bait_texture = asset_server.load("lures/swim_bait.png");
    let frog_bait_texture = asset_server.load("lures/frog_bait.png");
    let surf_rod_texture = asset_server.load("rods/surf.png");
    let monofil_texture = asset_server.load("lines/monofilament.png");
    let braided_line_texture = asset_server.load("lines/braided.png");
    let glasses_texture = asset_server.load("shop/polarized_glasses.png");
    let sold_texture: Handle<Image> = asset_server.load("shop/sold.png");

    //slot positions
    let slot_positions = [
        Vec3::new(SHOP_X - 380., SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X, SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X + 400., SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X - 380., SHOP_Y + 100., 2.),
        Vec3::new(SHOP_X, SHOP_Y + 100., 2.),
        Vec3::new(SHOP_X + 400., SHOP_Y + 100., 2.),
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
                "Surf Rod" => surf_rod_texture.clone(),
                "FluoroCarbon Fishing Line" => monofil_texture.clone(),
                "Braided Fishing Line" => braided_line_texture.clone(),
                "Polarized Sun Glasses" => glasses_texture.clone(),
                _ => {
                    println!("No texture found for item: {}", item.name);
                    continue;
                }
            };

            commands
                .entity(entity)
                .insert(SpriteBundle {
                    texture,
                    transform: Transform::from_translation(position),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(SpriteBundle {
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
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerDirection,
            &mut Location,
            &Animation,
            &mut InputStack,
        ),
        With<Player>,
    >,
    entrance_query: Query<
        (&Transform, &Tile),
        (With<ShopEntrance>, Without<Player>, Without<Camera>),
    >,
    time_of_day: Res<GameDayTimer>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>, Without<ShopEntrance>)>,
    mut original_camera_pos: ResMut<PlayerReturnPos>,
    mut next_interface: ResMut<NextState<CurrentInterface>>,
) {
    let (mut pt, mut pd, mut pl, _pa, mut pi) = player_query.single_mut();
    let (e_tran, e_tile) = entrance_query.single();
    if pt.translation.y - PLAYER_HEIGHT / 2. > e_tran.translation.y + e_tile.hitbox.y / 2.
        || pt.translation.y + PLAYER_HEIGHT / 2. < e_tran.translation.y - e_tile.hitbox.y / 2.
        || pt.translation.x + PLAYER_WIDTH / 2. < e_tran.translation.x - e_tile.hitbox.x / 2.
        || pt.translation.x - PLAYER_WIDTH / 2. > e_tran.translation.x + e_tile.hitbox.x / 2.
    {
        return;
    } else {
        if *pd == PlayerDirection::Back && time_of_day.hour < 21 {
            let mut camera = camera_query.single_mut();
            original_camera_pos.position = camera.translation;
            let new_position = Vec3::new(SHOP_X, SHOP_Y, camera.translation.z);
            camera.translation = new_position;
            next_interface.set(CurrentInterface::Shop);
            println!("Shop open");
        }
    }
}

fn handle_purchase(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shop_items: Query<&mut ShopItem>,
    mut player_inventory: Query<&mut PlayerInventory>,
    selected_item: Res<SelectedShopItem>,
    mut sold_spite: Query<&mut Visibility, With<SoldSprite>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        // Use Enter key to purchase
        println!("Attempting to purchase");
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            // let mut sold_sprites:Vec<_> = sold_spite.iter().collect();
            if let Some(mut shop_item) = shop_items.iter_mut().nth(selected_item.index) {
                if inventory.coins >= shop_item.price && !shop_item.is_bought {
                    inventory.coins -= shop_item.price;
                    inventory.items.push(shop_item.clone());

                    let category = match shop_item.item_type {
                        ItemType::ROD => &mut inventory.rods,
                        ItemType::LURE => &mut inventory.lures,
                        ItemType::LINE => &mut inventory.lines,
                        ItemType::COSMETIC => &mut inventory.cosmetics,
                    };

                    category.push(shop_item.clone());

                    shop_item.is_bought = true;

                    if let Some(mut sold_sprite_visibility) =
                        sold_spite.iter_mut().nth(selected_item.index)
                    {
                        *sold_sprite_visibility = Visibility::Visible;
                    }
                    println!("Purchased: {}", shop_item.name);
                } else if inventory.coins < shop_item.price {
                    println!("Not enough coins to purchase {}", shop_item.name);
                } else {
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
    hover_entity: Res<HoverEntity>,
    mut hover_query: Query<&mut Transform>,
) {
    let cols = 3;
    let rows = 2;

    let current_row = selected_item.index / cols;
    let current_col = selected_item.index % cols;

    if input.just_pressed(KeyCode::ArrowUp) {
        let new_row = if current_row == 0 {
            rows - 1
        } else {
            current_row - 1
        };
        selected_item.index = new_row * cols + current_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let new_row = if current_row == rows - 1 {
            0
        } else {
            current_row + 1
        };
        selected_item.index = new_row * cols + current_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowLeft) {
        let new_col = if current_col == 0 {
            cols - 1
        } else {
            current_col - 1
        };
        selected_item.index = current_row * cols + new_col;
        println!("Selected: {}", selected_item.index);
    }

    if input.just_pressed(KeyCode::ArrowRight) {
        let new_col = if current_col == cols - 1 {
            0
        } else {
            current_col + 1
        };
        selected_item.index = current_row * cols + new_col;
        println!("Selected: {}", selected_item.index);
    }

    // Define slot positions
    let slot_positions = [
        Vec3::new(SHOP_X - 380., SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X, SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X + 400., SHOP_Y - 180., 2.),
        Vec3::new(SHOP_X - 380., SHOP_Y + 100., 2.),
        Vec3::new(SHOP_X, SHOP_Y + 100., 2.),
        Vec3::new(SHOP_X + 400., SHOP_Y + 100., 2.),
    ];

    if let Some(&position) = slot_positions.get(selected_item.index) {
        if let Ok(mut hover_transform) = hover_query.get_mut(hover_entity.0) {
            let adjusted_x = if position == Vec3::new(SHOP_X, SHOP_Y - 180., 2.)
                || position == Vec3::new(SHOP_X, SHOP_Y + 100., 2.)
            {
                position.x + 2.0
            } else {
                position.x - 10.0
            };

            hover_transform.translation =
                Vec3::new(adjusted_x, position.y + 42.0, position.z + 1.0);
        }
    }
}

fn exit_shop(
    input: Res<ButtonInput<KeyCode>>,
    mut next_interface: ResMut<NextState<CurrentInterface>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    original_camera_pos: Res<PlayerReturnPos>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerDirection,
            &mut Location,
            &Animation,
            &mut InputStack,
        ),
        (With<Player>, Without<Camera>),
    >,
) {
    if input.just_pressed(KeyCode::Escape) {
        let mut camera = camera_query.single_mut();
        let (mut pt, mut pd, _pl, _pa, _pi) = player_query.single_mut();
        pt.translation = Vec3::new(1024., -180., 901.);
        *pd = PlayerDirection::Front;
        camera.translation = original_camera_pos.position;
        next_interface.set(CurrentInterface::Overworld);
        println!("Shop closed");
    }
}
