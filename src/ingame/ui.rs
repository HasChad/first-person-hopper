use bevy::prelude::*;

use crate::ingame::InGameEntity;

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
        //score text
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/NotoSans-Medium.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn ui_update() {}
