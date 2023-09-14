use bevy::prelude::*;

use crate::ingame::InGameEntity;
use crate::ingame::SCORE;

#[derive(Component)]
pub struct ScoreText;

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(10.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .insert(InGameEntity)
        .insert(ScoreText)
        //score text
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                ScoreText,
            ));
        });
}

pub fn ui_update(mut texts: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut texts {
        info!("nice work");
        text.sections[0].value = unsafe { SCORE.to_string() };
    }
}
