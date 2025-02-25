use avian2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_kira_audio::prelude::*;
use std::f32::consts::PI;

use super::{AnimationConfig, Scores};
use crate::{GameDifficultyState, SCREEN_WIDTH};

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct CursorCrosshair;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct M4 {
    pub lifetime: Timer,
    pub okay_to_shoot: bool,
}

#[derive(Component)]
pub struct EndGameTimer {
    pub lifetime: Timer,
}

pub fn ingame_setup(
    audio: Res<Audio>,
    mut commands: Commands,
    game_difficulty_state: Res<State<GameDifficultyState>>,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
    mut scores: ResMut<Scores>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    scores.current_score = 0;

    audio.play(asset_server.load("sounds/start.ogg"));

    // lock and hide crosshair
    //window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Confined;

    // end game timer creation
    commands.spawn((
        EndGameTimer {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
        },
        InGameEntity,
    ));

    // background spawn
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/background.png")),
        Transform::from_xyz(0.0, 0.0, -9.0),
        InGameEntity,
    ));

    // spawn m4 with animation props
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1550, 720), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 4, 30);

    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("sprites/m4_sheet.png"),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_config.first_sprite_index,
            },
        ),
        animation_config,
        M4 {
            lifetime: Timer::from_seconds(0.2, TimerMode::Once),
            okay_to_shoot: true,
        },
        InGameEntity,
    ));

    // crosshair
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/crosshair.png")),
        Collider::circle(5.0),
        Sensor,
        CursorCrosshair,
        InGameEntity,
    ));

    // left wall
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/wall.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: false,
                tile_y: true,
                stretch_value: 1.0,
            },
            custom_size: Some(Vec2::new(200.0, 10000.0)),
            ..default()
        },
        Transform::from_xyz(-SCREEN_WIDTH / 2.0, 0.0, -6.0),
        RigidBody::Static,
        Collider::rectangle(200.0, 100000.0),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        InGameEntity,
    ));

    // right wall
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/wall.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: false,
                tile_y: true,
                stretch_value: 1.0,
            },
            custom_size: Some(Vec2::new(200.0, 10000.0)),
            ..default()
        },
        Transform::from_xyz(SCREEN_WIDTH / 2.0, 0.0, -6.0).with_rotation(Quat::from_rotation_y(PI)),
        RigidBody::Static,
        Collider::rectangle(200.0, 100000.0),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        InGameEntity,
    ));

    // ball
    match *game_difficulty_state.get() {
        GameDifficultyState::Easy => {
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/easy_ball.png")),
                Transform::from_xyz(0.0, 0.0, -6.0),
                TransformInterpolation,
                RigidBody::Dynamic,
                RigidBodyDisabled,
                Collider::circle(50.0),
                Mass(1.0),
                GravityScale(150.0),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombine::Average,
                },
                LinearVelocity::default(),
                AngularVelocity::default(),
                ExternalImpulse::default(),
                ExternalAngularImpulse::default(),
                Ball,
                InGameEntity,
            ));
        }
        GameDifficultyState::Normal => {
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/normal_ball.png")),
                Transform::from_xyz(0.0, 0.0, -6.0),
                TransformInterpolation,
                RigidBody::Dynamic,
                RigidBodyDisabled,
                Collider::circle(50.0),
                Mass(1.0),
                GravityScale(220.0),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombine::Average,
                },
                LinearVelocity::default(),
                AngularVelocity::default(),
                ExternalImpulse::default(),
                ExternalAngularImpulse::default(),
                Ball,
                InGameEntity,
            ));
        }
        GameDifficultyState::Hard => {
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/hard_ball.png")),
                Transform::from_xyz(0.0, 0.0, -6.0),
                TransformInterpolation,
                RigidBody::Dynamic,
                RigidBodyDisabled,
                Collider::circle(25.0),
                Mass(1.1),
                GravityScale(230.0),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombine::Average,
                },
                LinearVelocity::default(),
                AngularVelocity::default(),
                ExternalImpulse::default(),
                ExternalAngularImpulse::default(),
                Ball,
                InGameEntity,
            ));
        }
    }
}
