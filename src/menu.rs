//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::prelude::*;

use super::AppState;

pub fn build_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(Update, (button_system).run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup);
}


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Play".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                game_state.set(AppState::Game)
            }
            Interaction::Hovered => {
                text.sections[0].value = "Play".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::GOLD;
            }
            Interaction::None => {
                text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/EvilEmpire.otf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}


fn cleanup(mut commands: Commands, query: Query<Entity, With<Node>>, cams: Query<Entity, With<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for ent in &cams {
        commands.entity(ent).despawn();
    }
}