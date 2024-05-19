//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{app::AppExit, prelude::*};

use crate::player::{Player, PlayerTool};

use super::{AppState, RESOLUTION_X, RESOLUTION_Y};

pub fn build_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(OnEnter(AppState::Game), hud_setup)
        .add_systems(Update, (play_button_system).run_if(in_state(AppState::Menu)))
        .add_systems(Update, (quit_button_system).run_if(in_state(AppState::Menu)))
        .add_systems(Update, (hud_update).run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Menu), cleanup);
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
enum HUDButtonAction {
    Tiller,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Music,
    SoundEffects,
}

#[derive(Component)]
struct ResourceCounter;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn play_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut text_query: Query<&mut Text, With<PlayButton>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    // for (interaction, mut color, mut border_color, children) in &mut interaction_query
    for (interaction, mut color, mut border_color, _children) in interaction_query.iter_mut() {
        let mut text = text_query.get_single_mut().unwrap();
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

fn quit_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut text_query: Query<&mut Text, With<QuitButton>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    // for (interaction, mut color, mut border_color, children) in &mut interaction_query
    for (interaction, mut color, mut border_color, _children) in interaction_query.iter_mut() {
        let mut text = text_query.get_single_mut().unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Quit".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                app_exit_events.send(AppExit);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Quit".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::GOLD;
            }
            Interaction::None => {
                text.sections[0].value = "Quit".to_string();
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
                flex_direction: FlexDirection::Column, // Set flex direction to Column
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Add the title text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "SolFarmer",
                    TextStyle {
                        font: asset_server.load("fonts/Disolve_light.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)), // Add margin to create space below the title
                    ..default()
                },
                ..default()
            });
            parent
                .spawn((PlayButton, ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)), // Add bottom margin for spacing
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }))
                .with_children(|parent| {
                    parent.spawn((PlayButton, TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((QuitButton, ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)), // Add bottom margin for spacing
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }))
                .with_children(|parent| {
                    parent.spawn((QuitButton, TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/BebasNeue-Regular.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )));
                });
        });
}

fn hud_setup(mut commands: Commands, assets: Res<AssetServer>) {
    // Spawn the parent node that will hold the row of buttons
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center, // Center the buttons horizontally
            align_items: AlignItems::FlexEnd, // Align items to the end (bottom)
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let buttons = [
            ("Toil", 0),
            ("U1", 1),
            ("U2", 2),
            ("U3", 3),
            ("U4", 4),
        ];

        for (label, index) in buttons.iter() {
            let mut button = parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(10.0), // Add bottom margin for spacing from the edge
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            });

            match index {
                0 => { button.insert(HUDButtonAction::Tiller); }
                1 => { button.insert(HUDButtonAction::Unit1); }
                2 => { button.insert(HUDButtonAction::Unit2); }
                3 => { button.insert(HUDButtonAction::Unit3); }
                4 => { button.insert(HUDButtonAction::Unit4); }
                _ => {}
            }

            button.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    *label,
                    TextStyle {
                        font: assets.load("fonts/BebasNeue-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    });

    // Spawn the parent node that will hold the row of buttons in the bottom right corner
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute, // Position the node absolutely
            right: Val::Px(-RESOLUTION_X + 140.), // Align the right edge with the screen's right edge
            bottom: Val::Px(-RESOLUTION_Y + 65.), // Align the bottom edge with the screen's bottom edge
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        let buttons = [
            ("Music", 0),
            ("SFX", 1),
        ];

        for (label, index) in buttons.iter() {
            let mut button = parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(10.0), // Add bottom margin for spacing from the edge
                    },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            });

            match index {
                0 => { button.insert(HUDButtonAction::Music); }
                1 => { button.insert(HUDButtonAction::SoundEffects); }
                _ => {}
            }

            button.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    *label,
                    TextStyle {
                        font: assets.load("fonts/BebasNeue-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }
    });

    // Spawn the resource counter
    commands.spawn((ResourceCounter, NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute, // Position the node absolutely
            right: Val::Px(-50.), // Align the right edge with the screen's right edge
            bottom: Val::Px(-RESOLUTION_Y + 65.), // Align the bottom edge with the screen's bottom edge
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        
    

        let mut button = parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(10.0), // Add bottom margin for spacing from the edge
                },
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        });

        button.with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(assets.load("images/Hoe3.png")),
            ));
            parent.spawn((ResourceCounter, TextBundle::from_section(
                "1",
                TextStyle {
                    font: assets.load("fonts/BebasNeue-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )));
        });
    });
}

fn hud_update (
    mut tiller_interact_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        &HUDButtonAction
    ), (Changed<Interaction>, With<HUDButtonAction>)>,
    mut player_tool: Query<&mut PlayerTool, With<Player>>,
    mut resource_text_query: Query<&mut Text, With<ResourceCounter>>,
) {
    // Handle Main 5 Buttons
    for (interaction, mut color, mut border_color, _children, button_action) in tiller_interact_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                // Logic

                if let Ok(mut tool) = player_tool.get_single_mut() {
                    *tool = match button_action {
                        HUDButtonAction::Tiller => PlayerTool::Tiller,
                        HUDButtonAction::Unit1 => PlayerTool::Planter,
                        HUDButtonAction::Unit2 => PlayerTool::Planter,
                        HUDButtonAction::Unit3 => PlayerTool::Planter,
                        HUDButtonAction::Unit4 => PlayerTool::Planter,
                        _ => tool.clone(),
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::GOLD;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Node>>, cams: Query<Entity, With<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for ent in &cams {
        commands.entity(ent).despawn();
    }
}