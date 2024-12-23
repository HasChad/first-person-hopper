use bevy::{ecs::system::RunSystemOnce, prelude::*};
use bevy_kira_audio::prelude::*;

use crate::{ingame::Scores, DespawnEvent, GameDifficultyState, GameState, CUSTOM_FONT};

#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct EasyButton;

#[derive(Component)]
pub struct MediumButton;

#[derive(Component)]
pub struct HardButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::WHITE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, scores: Res<Scores>) {
    //spawn full screen node bundle
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuEntity,
        ))
        //title node bundle
        .with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_child((
                    Text::new("FIRST PERSON HOPPER"),
                    TextFont {
                        font: asset_server.load(CUSTOM_FONT),
                        font_size: 100.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        })
        //button node bundle
        .with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(50.0),
                    ..default()
                })
                //spawn easy button
                .with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                            HardButton,
                        ))
                        .with_child((
                            Text::new(format!("EASY: {}", scores.easy_hscore)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.19, 0.76, 0.41)),
                        ));

                    //spawn medium button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                            MediumButton,
                        ))
                        .with_child((
                            Text::new(format!("MEDIUM: {}", scores.medium_hscore)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.35, 0.67, 0.89)),
                        ));

                    //spawn hard button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(220.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                            HardButton,
                        ))
                        .with_child((
                            Text::new(format!("HARD: {}", scores.hard_hscore)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.88, 0.21, 0.20)),
                        ));
                });
        });
}

pub fn easy_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EasyButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Easy);
            }
        }
    }
}

pub fn medium_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MediumButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Medium);
            }
        }
    }
}

pub fn hard_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HardButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Hard);
            }
        }
    }
}
