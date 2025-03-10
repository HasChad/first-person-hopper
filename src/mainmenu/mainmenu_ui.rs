use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    ButtonQuery, CUSTOM_FONT, GameDifficultyState, GameState, HOVERED_BUTTON, NORMAL_BUTTON,
    PRESSED_BUTTON, ingame::Scores,
};

#[derive(Component)]
pub struct EasyButton;

#[derive(Component)]
pub struct NormalButton;

#[derive(Component)]
pub struct HardButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct MainMenuEntity;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    scores: Res<Scores>,
) {
    audio.play(asset_server.load("sounds/main_menu.ogg"));

    // full screen node
    commands
        .spawn((
            ImageNode::from(asset_server.load("sprites/menu_background.png")),
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
        // title node
        .with_children(|parent| {
            let title_size = 30. / 100.;
            parent
                .spawn(Node {
                    height: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,
                    ..default()
                })
                .with_child((
                    ImageNode::from(asset_server.load("sprites/title.png")),
                    Node {
                        width: Val::Px(2018. * title_size),
                        height: Val::Px(1060. * title_size),
                        ..default()
                    },
                ));
        })
        // button node
        .with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(50.0),
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                // easy button
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
                            EasyButton,
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

                    // normal button
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
                            NormalButton,
                        ))
                        .with_child((
                            Text::new(format!("NORMAL: {}", scores.normal_hscore)),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.35, 0.67, 0.89)),
                        ));

                    // hard button
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

                    // quit button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                            QuitButton,
                        ))
                        .with_child((
                            Text::new("QUIT"),
                            TextFont {
                                font: asset_server.load(CUSTOM_FONT),
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                });
        });
}

pub fn easy_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut button_query: Query<ButtonQuery, (Changed<Interaction>, With<EasyButton>)>,
) {
    for mut button in button_query.iter_mut() {
        match *button.interaction {
            Interaction::None => {
                *button.color = NORMAL_BUTTON.into();
                button.border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *button.color = HOVERED_BUTTON.into();
                button.border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *button.color = PRESSED_BUTTON.into();
                button.border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Easy);
            }
        }
    }
}

pub fn normal_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut button_query: Query<ButtonQuery, (Changed<Interaction>, With<NormalButton>)>,
) {
    for mut button in button_query.iter_mut() {
        match *button.interaction {
            Interaction::None => {
                *button.color = NORMAL_BUTTON.into();
                button.border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *button.color = HOVERED_BUTTON.into();
                button.border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *button.color = PRESSED_BUTTON.into();
                button.border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Normal);
            }
        }
    }
}

pub fn hard_button_system(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_difficulty_state: ResMut<NextState<GameDifficultyState>>,
    mut button_query: Query<ButtonQuery, (Changed<Interaction>, With<HardButton>)>,
) {
    for mut button in button_query.iter_mut() {
        match *button.interaction {
            Interaction::None => {
                *button.color = NORMAL_BUTTON.into();
                button.border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *button.color = HOVERED_BUTTON.into();
                button.border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *button.color = PRESSED_BUTTON.into();
                button.border_color.0 = Color::BLACK;
                next_game_state.set(GameState::InGame);
                next_difficulty_state.set(GameDifficultyState::Hard);
            }
        }
    }
}

pub fn quit_button_system(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut button_query: Query<ButtonQuery, (Changed<Interaction>, With<QuitButton>)>,
) {
    for mut button in button_query.iter_mut() {
        match *button.interaction {
            Interaction::None => {
                *button.color = NORMAL_BUTTON.into();
                button.border_color.0 = Color::BLACK;
            }
            Interaction::Hovered => {
                *button.color = HOVERED_BUTTON.into();
                button.border_color.0 = Color::WHITE;
                audio.play(asset_server.load("sounds/hover_button.ogg"));
            }
            Interaction::Pressed => {
                *button.color = PRESSED_BUTTON.into();
                button.border_color.0 = Color::BLACK;
                app_exit_events.send(AppExit::Success);
            }
        }
    }
}
