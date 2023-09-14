use bevy::{prelude::*, window::CursorGrabMode};
use bevy_cursor::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use crate::ingame::Ball;
use crate::ingame::CursorCrosshair;
use crate::ingame::EndGameTimer;
use crate::ingame::InGameEntity;
use crate::ingame::M4;

pub fn m4_shooting(mut m4: Query<&mut M4>, time: Res<Time>) {
    let mut m4_props = m4.single_mut();

    if !m4_props.okay_to_shoot {
        m4_props.lifetime.tick(time.delta());

        if m4_props.lifetime.finished() {
            m4_props.okay_to_shoot = true;
            m4_props.lifetime = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }
}

pub fn cursor_position(
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

pub fn ball_movement(
    mut commands: Commands,
    mut ball: Query<(Entity, &mut ExternalImpulse, &mut Velocity), With<Ball>>,
    mut m4: Query<&mut M4>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
) {
    let entity_ball = ball.single().0;
    let entity_cross = crosshair.single();
    let mut m4_props = m4.single_mut();

    if input.just_pressed(MouseButton::Left) && m4_props.okay_to_shoot {
        //sound play
        m4_props.okay_to_shoot = false;
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds\\M16.ogg"),
            ..default()
        });

        //jump ball if collide eachother
        for (_, mut ball_impulse, mut ball_velocity) in &mut ball {
            if rapier_context.intersection_pair(entity_ball, entity_cross) == Some(true) {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds\\click.ogg"),
                    ..default()
                });
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

pub fn entity_despawner(
    mut timer: Query<&mut EndGameTimer>,
    mut entities: Query<Entity, With<InGameEntity>>,
    mut commands: Commands,
    ball: Query<&Transform, With<Ball>>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
) {
    if ball.single().translation.y < -420.0 {
        info!("timer created");

        let mut end_game_timer = timer.single_mut();

        end_game_timer.lifetime.tick(time.delta());

        if end_game_timer.lifetime.finished() {
            info!("Despawner Activated");

            //enable cursor
            let mut window = windows.single_mut();
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            //change state
            commands.insert_resource(NextState(Some(AppState::GameOver)));

            //despawn everyting in InGame
            for entities_despawner in &mut entities {
                commands.entity(entities_despawner).despawn();
            }
        }
    }
}
