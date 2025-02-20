use bevy::prelude::*;

use super::InGameEntity;
use crate::{ingame::Scores, CUSTOM_FONT};

#[derive(Component)]
pub struct ScoreText;

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
        InGameEntity,
    ));
}

pub fn score_text_updater(
    mut writer: TextUiWriter,
    scores: ResMut<Scores>,
    entity: Single<Entity, With<ScoreText>>,
) {
    *writer.text(*entity, 0) = scores.current_score.to_string();
}
