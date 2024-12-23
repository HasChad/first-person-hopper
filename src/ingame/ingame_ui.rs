use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{ingame::Scores, CUSTOM_FONT};

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct FpsText;

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::default(),
        TextFont {
            font: asset_server.load(CUSTOM_FONT),
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Start,
            ..default()
        },
        ScoreText,
    ));

    //MARK: FPS UI
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::End,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            Name::new("UI - FPSCounter"),
            FpsText,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(GOLD.into()),
        ));
}

pub fn score_text_updater(
    mut writer: TextUiWriter,
    scores: ResMut<Scores>,
    entity: Single<Entity, With<ScoreText>>,
) {
    *writer.text(*entity, 0) = format!("{}", scores.current_score.to_string());
}

pub fn fps_text_updater(
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
    entity: Single<Entity, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            *writer.text(*entity, 1) = format!("{value:.0}");
        }
    }
}
