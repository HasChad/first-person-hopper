#![allow(clippy::complexity)]

use bevy::prelude::*;

use crate::AppState;

use crate::ingame::Scores;

#[derive(Component)]
pub struct HomeButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct GameOverEntity;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, scores: Res<Scores>) {
    info!("GameOver menu activated");

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/gameover_sound.ogg"),
        ..default()
    });

    //create full screen node bundle
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .insert(GameOverEntity)
        //create upper "game over" title node bundle
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(75.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::End,
                        row_gap: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                //create yellow background
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(800.0),
                                height: Val::Px(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: Color::YELLOW.into(),
                            ..default()
                        })
                        //"game over" text
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "GAME OVER",
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 120.0,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                })
                //current score: create black background
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(550.0),
                                height: Val::Px(75.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("SCORE: {}", scores.current_score),
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 70.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                })
                //high score: create black background
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(550.0),
                                height: Val::Px(75.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("HIGH SCORE: {}", scores.high_score),
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 70.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        })
        //create "main menu" and "restart" buttons
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(50.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                })
                //spawn "main menu" button
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
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
                        .insert(HomeButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "MAIN MENU",
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    //spawn restart button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
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
                        .insert(RestartButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "RESTART",
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

pub fn home_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HomeButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/hover_button.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn restart_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/hover_button.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn entity_despawner(mut entities: Query<Entity, With<GameOverEntity>>, mut commands: Commands) {
    info!("Main Menu Despawner Activated");

    //despawn everyting in InGame
    for entities_despawner in &mut entities {
        commands.entity(entities_despawner).despawn_recursive();
    }
}
