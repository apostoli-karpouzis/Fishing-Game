use crate::fishing_view::*;
use crate::interface::*;
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

pub fn fishing_button_system(
    input: Res<ButtonInput<KeyCode>>,   
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), (With<FishingButton>,)>,
    mut visibility_query: Query<&mut Visibility, With<FishingButton>>, 
    mut text_query: Query<&mut Text>, 
    mut start_fishing_animation: ResMut<StartFishingAnimation>,
    mut fishing_timer: ResMut<FishingAnimationDuration>,
    mut next_state: ResMut<NextState<CurrentInterface>>,  
    state: Res<State<CurrentInterface>>,  
) {
   
    for (mut color, mut border_color, children) in &mut button_query {
        let mut visibility = visibility_query.single_mut();
        let mut text = text_query.get_mut(children[0]).unwrap();

        
        if *visibility == Visibility::Visible || state.eq(&CurrentInterface::Fishing) {

            
            if state.eq(&CurrentInterface::Overworld) {
                text.sections[0].value = "Throw Rod(X)".to_string(); 
            } else if state.eq(&CurrentInterface::Fishing) {
                text.sections[0].value = "Exit(ESC)".to_string(); 
            }

            
            if input.pressed(KeyCode::KeyX) && state.eq(&CurrentInterface::Overworld) {
                *color = HOVERED_BUTTON.into();  
                border_color.0 = Color::WHITE;
            } else {
                *color = NORMAL_BUTTON.into();  
                border_color.0 = Color::BLACK;
            }

            
            if input.just_pressed(KeyCode::KeyX) && state.eq(&CurrentInterface::Overworld) {
                *color = PRESSED_BUTTON.into();  
                start_fishing_animation.active = true;
                start_fishing_animation.button_control_active = false;
                fishing_timer.0.reset();

                
                next_state.set(CurrentInterface::Fishing);
                println!("Switching to fishing mode");

            
            } else if input.just_pressed(KeyCode::Escape) && state.eq(&CurrentInterface::Fishing) {
                println!("Exiting fishing mode");
                *color = NORMAL_BUTTON.into();  
                start_fishing_animation.active = false;
                start_fishing_animation.button_control_active = true;
                *visibility = Visibility::Visible;  

                
                next_state.set(CurrentInterface::Overworld);
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

            
            });
}