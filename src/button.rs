use crate::resources::*;
use crate::fishing_view::*;
use crate::shop::*;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ButtonVisible(pub bool);

#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct FishingButton;

#[derive(Component)]
pub struct ShopingButton;

pub fn fishing_button_system(
    input: Res<ButtonInput<KeyCode>>,   
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), (With<FishingButton>,)>,
    mut visibility_query: Query<&mut Visibility, With<FishingButton>>, 
    mut text_query: Query<&mut Text>, 
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,
    mut next_state: ResMut<NextState<FishingMode>>,  
    state: Res<State<FishingMode>>,  
) {
   
    for (mut color, mut border_color, children) in &mut button_query {
        let mut visibility = visibility_query.single_mut();
        let mut text = text_query.get_mut(children[0]).unwrap();

        
        if *visibility == Visibility::Visible || state.eq(&FishingMode::Fishing) {

            
            if state.eq(&FishingMode::Overworld) {
                text.sections[0].value = "Throw Rod(X)".to_string(); 
            } else if state.eq(&FishingMode::Fishing) {
                text.sections[0].value = "Exit(ESC)".to_string(); 
            }

            
            if input.pressed(KeyCode::KeyX) && state.eq(&FishingMode::Overworld) {
                *color = HOVERED_BUTTON.into();  
                border_color.0 = Color::WHITE;
            } else {
                *color = NORMAL_BUTTON.into();  
                border_color.0 = Color::BLACK;
            }

            
            if input.just_pressed(KeyCode::KeyX) && state.eq(&FishingMode::Overworld) {
                *color = PRESSED_BUTTON.into();  
                start_fishing_animation.active = true;
                start_fishing_animation.button_control_active = false;
                fishing_timer.0.reset();

                
                next_state.set(FishingMode::Fishing);
                println!("Switching to fishing mode");

            
            } else if input.just_pressed(KeyCode::Escape) && state.eq(&FishingMode::Fishing) {
                println!("Exiting fishing mode");
                *color = NORMAL_BUTTON.into();  
                start_fishing_animation.active = false;
                start_fishing_animation.button_control_active = true;
                *visibility = Visibility::Visible;  

                
                next_state.set(FishingMode::Overworld);
                println!("Switching to overworld mode");
            }
        }
    }
}

pub fn shop_button_system(
    input: Res<ButtonInput<KeyCode>>,   
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), (With<ShopingButton>,)>,
    mut visibility_query: Query<&mut Visibility, With<ShopingButton>>, 
    mut text_query: Query<&mut Text>, 
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,
    mut next_state: ResMut<NextState<ShopingMode>>,  
    state: Res<State<ShopingMode>>,
) {
   
    for (mut color, mut border_color, children) in &mut button_query {
        let mut visibility = visibility_query.single_mut();
        let mut text = text_query.get_mut(children[0]).unwrap();

        
        if *visibility == Visibility::Visible || state.eq(&ShopingMode::Shop) {

            
            if state.eq(&ShopingMode::Overworld) {
                text.sections[0].value = "Shop(E)".to_string(); 
            } else if state.eq(&ShopingMode::Shop) {
                text.sections[0].value = "Exit(ESC)".to_string(); 
            }

            
            if input.pressed(KeyCode::KeyE) && state.eq(&ShopingMode::Overworld) {
                *color = HOVERED_BUTTON.into();  
                border_color.0 = Color::WHITE;
            } else {
                *color = NORMAL_BUTTON.into();  
                border_color.0 = Color::BLACK;
            }

            
            if input.just_pressed(KeyCode::KeyE) && state.eq(&ShopingMode::Overworld){
                *color = PRESSED_BUTTON.into();  
                start_fishing_animation.active = true;
                start_fishing_animation.button_control_active = false;
                fishing_timer.0.reset();

                
                next_state.set(ShopingMode::Shop);
                // println!("Switching to shoping mode");

            
            } else if input.just_pressed(KeyCode::Escape) && state.eq(&ShopingMode::Shop) {
                println!("Exiting shoping mode");
                *color = NORMAL_BUTTON.into();  
                start_fishing_animation.active = false;
                start_fishing_animation.button_control_active = true;
                *visibility = Visibility::Visible;  

                
                next_state.set(ShopingMode::Overworld);
                println!("Switching to overworld mode");
            }
        }
    }
}


pub fn spawn_fishing_button(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(98.0),
                    height: Val::Percent(98.0),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            ButtonVisible(false),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                FishingButton,
            ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Throw Rod(X)",
                        TextStyle {
                            font: asset_server.load("fonts/pixel.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                    .spawn((
                        ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ShopingButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Shop(E)",
                            TextStyle {
                                font: asset_server.load("fonts/pixel.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            });
}

// pub fn spawn_shop_button(commands: &mut Commands, asset_server: &Res<AssetServer>) {
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Percent(98.0),
//                     height: Val::Percent(98.0),
//                     align_items: AlignItems::End,
//                     justify_content: JustifyContent::Start,
                    
//                     ..default()
//                 },
//                 ..default()
//             },
//             ButtonVisible(false),
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn((
//                     ButtonBundle {
//                     style: Style {
//                         width: Val::Px(250.0),
//                         height: Val::Px(65.0),
//                         border: UiRect::all(Val::Px(5.0)),
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     border_color: BorderColor(Color::BLACK),
//                     border_radius: BorderRadius::MAX,
//                     background_color: NORMAL_BUTTON.into(),
//                     visibility: Visibility::Hidden,
//                     ..default()
//                 },
//                 ShopingButton,
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle::from_section(
//                         "Shop(E)",
//                         TextStyle {
//                             font: asset_server.load("pixel.ttf"),
//                             font_size: 40.0,
//                             color: Color::srgb(0.9, 0.9, 0.9),
//                         },
//                     ));
//                 });
//         });
// }