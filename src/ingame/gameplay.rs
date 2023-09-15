#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_cursor::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use crate::ingame::Ball;
use crate::ingame::CursorCrosshair;
use crate::ingame::EndGameTimer;
use crate::ingame::InGameEntity;
use crate::ingame::Scores;
use crate::ingame::M4;

#[derive(Event)]
pub struct JumpBallEvent;

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
    mut scores: ResMut<Scores>,
    mut ball: Query<(&mut ExternalImpulse, &mut Velocity), With<Ball>>,
    mut event_reader: EventReader<JumpBallEvent>,
) {
    //jump ball if collide eachother
    for (mut ball_impulse, mut ball_velocity) in &mut ball {
        for _event in event_reader.iter() {
            scores.current_score += 1;
            info!("{}", scores.current_score);
            ball_velocity.linvel.y = 0.0;
            ball_velocity.linvel.x = 0.0;
            ball_velocity.angvel = 0.0;
            ball_impulse.impulse.y = alea::f32_in_range(500000.0, 900000.0);
            ball_impulse.impulse.x = alea::f32_in_range(-500000.0, 500000.0);
            ball_impulse.torque_impulse = alea::f32_in_range(-10000000.0, 10000000.0);
        }
    }
}

pub fn ball_contact_checker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rapier_context: Res<RapierContext>,
    ball: Query<Entity, With<Ball>>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    mut m4: Query<&mut M4>,
    input: Res<Input<MouseButton>>,
    mut event_writer: EventWriter<JumpBallEvent>,
) {
    let ball_entity = ball.single();
    let cross_entity = crosshair.single();
    let mut m4_props = m4.single_mut();

    if input.just_pressed(MouseButton::Left) && m4_props.okay_to_shoot {
        //m4 sound play and 0 rate of fire
        m4_props.okay_to_shoot = false;
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds\\M16.ogg"),
            ..default()
        });

        //check jump ball collide
        if rapier_context.intersection_pair(ball_entity, cross_entity) == Some(true) {
            event_writer.send(JumpBallEvent);
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds\\click.ogg"),
                ..default()
            });
        }
    }
}

pub fn entity_despawner(
    mut timer: Query<&mut EndGameTimer>,
    mut entities: Query<Entity, With<InGameEntity>>,
    mut commands: Commands,
    ball: Query<(&Transform, &Ball)>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut scores: ResMut<Scores>,
) {
    if ball.single().0.translation.y < -420.0 {
        let ball_x = ball.single().1;

        match ball_x {
            Ball::Easy => {
                scores.high_score = scores.easy_hscore;

                if scores.current_score > scores.easy_hscore {
                    scores.easy_hscore = scores.current_score;
                    scores.high_score = scores.current_score
                }
            }
            Ball::Medium => {
                scores.high_score = scores.medium_hscore;

                if scores.current_score > scores.medium_hscore {
                    scores.medium_hscore = scores.current_score;
                    scores.high_score = scores.current_score
                }
            }
            Ball::Hard => {
                scores.high_score = scores.hard_hscore;

                if scores.current_score > scores.hard_hscore {
                    scores.hard_hscore = scores.current_score;
                    scores.high_score = scores.current_score
                }
            }
        }

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
