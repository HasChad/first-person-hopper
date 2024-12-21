use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{ingame::Scores, GameState, CUSTOM_FONT};

#[derive(Component)]
pub struct GameOverEntity;

#[derive(Component)]
pub struct HomeButton;

#[derive(Component)]
pub struct RestartButton;

const PRESSED_BUTTON: Color = Color::WHITE;
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    scores: Res<Scores>,
) {
    audio.play(asset_server.load("sounds/gameover_sound.ogg"));

    //create full screen node bundle
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(100.0),
                ..default()
            },
            GameOverEntity,
        ))
        //create upper "game over" title node bundle
        .with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(75.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    row_gap: Val::Px(25.0),
                    ..default()
                })
                // game over, yellow background
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(800.0),
                                height: Val::Px(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(255.0, 192.0, 0.0)),
                        ))
                        .with_child((
                            Text::new("GAME OVER"),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 120.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));
                })
                // current score, black background
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(550.0),
                                height: Val::Px(75.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::BLACK),
                        ))
                        .with_child((
                            Text::new(format!("SCORE: {}", scores.current_score)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 70.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                })
                // high score, black background
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(550.0),
                                height: Val::Px(75.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::BLACK),
                        ))
                        .with_child((
                            Text::new(format!("HIGH SCORE: {}", scores.current_score)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 70.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                });
        })
        // buttons
        .with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    //spawn main menu button
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
                            HomeButton,
                        ))
                        .with_child((
                            Text::new("MAIN MENU"),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));

                    // restart button
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
                            RestartButton,
                        ))
                        .with_child((
                            Text::new("RESTART"),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                });
        });
}

pub fn home_button_system(
    mut next_gamestate: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HomeButton>),
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
                next_gamestate.set(GameState::MainMenu);
            }
        }
    }
}

pub fn restart_button_system(
    mut next_gamestate: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<RestartButton>),
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
                next_gamestate.set(GameState::InGame);
            }
        }
    }
}
