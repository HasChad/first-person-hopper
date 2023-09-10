use bevy::{prelude::*, window::CursorGrabMode};
use bevy_cursor::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;
use crate::GameDifficultyState;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

#[derive(Component)]
struct InGameEntity;

#[derive(Component)]
struct CursorCrosshair;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct M4;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // ! .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(CursorInfoPlugin)
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    setup,
                    game_diffuculty_easy.run_if(in_state(GameDifficultyState::Easy)),
                    game_diffuculty_hard.run_if(in_state(GameDifficultyState::Hard)),
                ),
            )
            .add_systems(
                Update,
                (cursor_position, ball_movement, entity_despawner)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn game_diffuculty_hard(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Hard ball created");

    //normal jump-ball spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\ball.png"),
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)))
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
        .insert(Ball)
        .insert(InGameEntity);
}

fn game_diffuculty_easy(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Normal ball created");

    //normal jump-ball spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\ball.png"),
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)))
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
        .insert(Ball)
        .insert(InGameEntity);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: Query<&mut Window>) {
    info!("Game Started");

    //lock and hide crosshair
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;

    //spawn m4
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\m4.png"),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(M4)
        .insert(InGameEntity);

    //crosshair and collision spawn
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\crosshair.png"),
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
            texture: asset_server.load("sprites\\wall.png"),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(100.0, SCREEN_HEIGHT / 2.0 + 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            SCREEN_WIDTH / 2.0,
            0.0,
            -1.0,
        )))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(InGameEntity);

    //left wall
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\wall.png"),
            ..default()
        })
        .insert(Collider::cuboid(100.0, SCREEN_HEIGHT / 2.0 + 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -SCREEN_WIDTH / 2.0,
            0.0,
            -1.0,
        )))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(InGameEntity);
}

fn cursor_position(
    q_windows: Res<CursorInfo>,
    mut crosshair: Query<&mut Transform, With<CursorCrosshair>>,
    mut m4: Query<&mut Transform, (With<M4>, Without<CursorCrosshair>)>,
) {
    let Some(position) = q_windows.position() else {
        return; //info!("out of window");
    };

    let mut m4_position = m4.single_mut();

    for mut crosshair_pos in &mut crosshair {
        //crosshair position
        crosshair_pos.translation.x = position.x;
        crosshair_pos.translation.y = position.y;
        //m4 position relative to cursor position
        m4_position.translation.x = position.x + 350.0;
        m4_position.translation.y = position.y - 400.0;
    }
}

fn ball_movement(
    mut commands: Commands,
    mut ball: Query<(Entity, &mut ExternalImpulse, &mut Velocity), With<Ball>>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
) {
    let entity_ball = ball.single().0;
    let entity_cross = crosshair.single();

    if input.just_pressed(MouseButton::Left) {
        //sound play
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds\\M16.ogg"),
            ..default()
        });

        //jump ball if collide eachother
        for (_, mut ball_impulse, mut ball_velocity) in &mut ball {
            if rapier_context.intersection_pair(entity_ball, entity_cross) == Some(true) {
                ball_velocity.linvel.y = 0.0;
                ball_velocity.linvel.x = 0.0;
                ball_velocity.angvel = 0.0;
                ball_impulse.impulse.y = alea::f32_in_range(500000.0, 900000.0);
                ball_impulse.impulse.x = alea::f32_in_range(-500000.0, 500000.0);
                ball_impulse.torque_impulse = alea::f32_in_range(-10000000.0, 10000000.0);
            }
        }
    }
}

fn entity_despawner(
    mut entities: Query<Entity, With<InGameEntity>>,
    mut commands: Commands,
    ball: Query<&Transform, With<Ball>>,
    mut windows: Query<&mut Window>,
) {
    if ball.single().translation.y < -420.0 {
        info!("Despawner Activated");

        //enable cursor
        let mut window = windows.single_mut();
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;

        //change state
        commands.insert_resource(NextState(Some(AppState::MainMenu)));

        //despawn everyting in InGame
        for entities_despawner in &mut entities {
            commands.entity(entities_despawner).despawn();
        }
    }
}
