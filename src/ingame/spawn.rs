use avian2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_kira_audio::prelude::*;

use super::Scores;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Component)]
pub struct CursorCrosshair;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
pub enum Ball {
    Easy,
    Medium,
    Hard,
}

#[derive(Component)]
pub struct M4 {
    pub lifetime: Timer,
    pub okay_to_shoot: bool,
}

#[derive(Component)]
pub struct EndGameTimer {
    pub lifetime: Timer,
}

pub fn setup(
    audio: Res<Audio>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
    mut scores: ResMut<Scores>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    scores.current_score = 0;

    audio.play(asset_server.load("sounds/start.ogg"));

    //lock and hide crosshair
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Confined;

    //end game timer creation
    commands.spawn(EndGameTimer {
        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
    });

    //background spawn
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/background.png")),
        Transform::from_xyz(0.0, 0.0, -9.0),
    ));

    //spawn m4 with animation props
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1550, 720), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 4 };

    commands
        // Spawn a bevy sprite-sheet
        .spawn((
            Sprite::from_atlas_image(
                asset_server.load("sprites/m4_sheet.png"),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            M4 {
                lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                okay_to_shoot: true,
            },
        ));

    //crosshair and collision spawn
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/crosshair.png")),
        Collider::circle(5.0),
        Sensor,
        CursorCrosshair,
    ));

    //right wall
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/wall.png")),
        Transform::from_xyz(SCREEN_WIDTH / 2.0, 0.0, -6.0),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        Collider::rectangle(100.0, SCREEN_HEIGHT / 2.0 + 500.0),
    ));

    //left wall
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/wall.png")),
        Transform::from_xyz(-SCREEN_WIDTH / 2.0, 0.0, -6.0),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        Collider::rectangle(100.0, SCREEN_HEIGHT / 2.0 + 500.0),
    ));
}

pub fn game_difficulty_easy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/easy_ball.png")),
        Transform::from_xyz(0.0, 0.0, -6.0),
        RigidBody::Dynamic,
        Collider::circle(50.0),
        Mass(0.1),
        GravityScale(17.0),
        Sleeping,
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Average,
        },
        LinearVelocity::default(),
        AngularVelocity::default(),
        ExternalImpulse::default(),
        ExternalTorque::default(),
        Ball::Easy,
    ));
}

pub fn game_difficulty_medium(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/medium_ball.png")),
        Transform::from_xyz(0.0, 0.0, -6.0),
        RigidBody::Dynamic,
        Collider::circle(50.0),
        Mass(0.1),
        GravityScale(30.0),
        Sleeping,
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Average,
        },
        LinearVelocity::default(),
        ExternalImpulse::default(),
        AngularVelocity::default(),
        ExternalTorque::default(),
        Ball::Medium,
    ));
}

pub fn game_difficulty_hard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/hard_ball.png")),
        Transform::from_xyz(0.0, 0.0, -6.0),
        RigidBody::Dynamic,
        Collider::circle(25.0),
        Mass(0.4),
        GravityScale(24.0),
        Sleeping,
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Average,
        },
        LinearVelocity::default(),
        ExternalImpulse::default(),
        AngularVelocity::default(),
        ExternalTorque::default(),
        Ball::Hard,
    ));
}
