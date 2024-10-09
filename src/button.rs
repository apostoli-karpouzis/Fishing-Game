use bevy::prelude::*;
use crate::resources::*;
use crate::fishingView::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ButtonVisible(pub bool);


pub fn button_system(
    input: Res<ButtonInput<KeyCode>>,   
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), (With<Button>,)>,
    mut visibility_query: Query<&mut Visibility, With<Button>>, 
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
                text.sections[0].value = "Exit(X)".to_string(); 
            }

            
            if input.pressed(KeyCode::KeyX) {
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

            
            } else if input.just_pressed(KeyCode::KeyX) && state.eq(&FishingMode::Fishing) {
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



pub fn spawn_button(commands: &mut Commands, asset_server: Res<AssetServer>) {
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
                .spawn(ButtonBundle {
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
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Throw Rod(X)",
                        TextStyle {
                            font: asset_server.load("pixel.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

