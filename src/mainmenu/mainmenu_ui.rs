#![allow(clippy::complexity)]

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::ingame::Scores;
use crate::AppState;
use crate::GameDifficultyState;

#[derive(Component)]
pub struct EasyButton;

#[derive(Component)]
pub struct MediumButton;

#[derive(Component)]
pub struct HardButton;

#[derive(Component)]
pub struct MainMenuEntity;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, scores: Res<Scores>) {
    //spawn full screen node bundle
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                // vertically center child text
                align_items: AlignItems::Center,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(MainMenuEntity)
        //title node bundle
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(30.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        // vertically center child text
                        align_items: AlignItems::End,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                })
                //title
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("sprites/title.png"),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        //button node bundle
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(50.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(75.0),
                        ..default()
                    },
                    ..default()
                })
                //spawn easy button
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(220.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(EasyButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("EASY: {}", scores.easy_hscore),
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.19, 0.76, 0.41),
                                },
                            ));
                        });

                    //spawn medium button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(220.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(MediumButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("MEDIUM: {}", scores.medium_hscore),
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.35, 0.67, 0.89),
                                },
                            ));
                        });

                    //spawn hard button
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(220.0),
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
                        .insert(HardButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("HARD: {}", scores.hard_hscore),
                                TextStyle {
                                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.88, 0.21, 0.20),
                                },
                            ));
                        });
                });
        });
}

pub fn easy_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EasyButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Easy)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn medium_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MediumButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Medium)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn hard_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HardButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Hard)))
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn entity_despawner(mut entities: Query<Entity, With<MainMenuEntity>>, mut commands: Commands) {
    info!("Main Menu Despawner Activated");

    //despawn everyting in InGame
    for entities_despawner in &mut entities {
        commands.entity(entities_despawner).despawn_recursive();
    }
}
