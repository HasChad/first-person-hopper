use bevy::prelude::*;

use crate::{ingame::Scores, CUSTOM_FONT};

#[derive(Component)]
pub struct ScoreText;

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            height: Val::Percent(10.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::default(),
        TextFont {
            font: asset_server.load(CUSTOM_FONT),
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        ScoreText,
    ));
}

pub fn ui_update(
    mut writer: TextUiWriter,
    scores: ResMut<Scores>,
    entity: Single<Entity, With<ScoreText>>,
) {
    *writer.text(*entity, 0) = format!("{}", scores.current_score.to_string());
}
