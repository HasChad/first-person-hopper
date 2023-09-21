use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier2d::prelude::*;

use crate::ingame::Animation;
use crate::ingame::AnimationState;
use crate::ingame::Scores;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct CursorCrosshair;

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

pub fn game_difficulty_easy(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Easy ball created");

    //normal jump-ball spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/easy_ball.png"),
            sprite: Sprite {
                color: Color::rgb(0.40, 1.5, 0.8),
                ..default()
            },
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -6.0)))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(17.0))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Average,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(Ball::Easy)
        .insert(InGameEntity);
}

pub fn game_difficulty_medium(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Medium ball created");

    //normal jump-ball spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/medium_ball.png"),
            sprite: Sprite {
                color: Color::rgb(0.7, 1.4, 1.8),
                ..default()
            },
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -6.0)))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(30.0))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Average,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(Ball::Medium)
        .insert(InGameEntity);
}

pub fn game_difficulty_hard(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Hard ball created");

    //normal jump-ball spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/hard_ball.png"),
            sprite: Sprite {
                color: Color::rgb(1.8, 0.40, 0.40),
                ..default()
            },
            ..default()
        })
        .insert(Collider::ball(25.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -6.0)))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(24.0))
        .insert(ColliderMassProperties::Density(0.4))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Average,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(Ball::Hard)
        .insert(InGameEntity);
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
    mut scores: ResMut<Scores>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    info!("Game Started");

    scores.current_score = 0;

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/start.ogg"),
        ..default()
    });

    //lock and hide crosshair
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;

    //end game timer creation
    commands
        .spawn(EndGameTimer {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .insert(InGameEntity);

    //background spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/background.png"),
            transform: Transform::from_xyz(0.0, 0.0, -9.0),
            ..default()
        })
        .insert(InGameEntity);

    //spawn m4 with animation props
    commands
        // Spawn a bevy sprite-sheet
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("sprites/m4_sheet.png"),
                Vec2::new(1550.0, 720.0),
                5,
                1,
                None,
                None,
            )),
            ..default()
        })
        //Create and insert an animation
        .insert(Animation(benimator::Animation::from_indices(
            0..=4,
            benimator::FrameRate::from_fps(30.0),
        )))
        // Insert the state
        .insert(AnimationState::default())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(M4 {
            lifetime: Timer::from_seconds(0.2, TimerMode::Once),
            okay_to_shoot: true,
        })
        .insert(InGameEntity);

    //crosshair and collision spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/crosshair.png"),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Collider::ball(5.0))
        .insert(Sensor)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CursorCrosshair)
        .insert(InGameEntity);

    //spawn side walls
    //right wall
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/wall.png"),
            sprite: Sprite {
                color: Color::rgba(5.0, 5.0, 5.0, 0.1),
                flip_x: true,
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(100.0, SCREEN_HEIGHT / 2.0 + 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            SCREEN_WIDTH / 2.0,
            0.0,
            -6.0,
        )))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(InGameEntity);

    //left wall
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/wall.png"),
            sprite: Sprite {
                color: Color::rgba(5.0, 5.0, 5.0, 0.1),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(100.0, SCREEN_HEIGHT / 2.0 + 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -SCREEN_WIDTH / 2.0,
            0.0,
            -6.0,
        )))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(InGameEntity);
}
