#![allow(clippy::too_many_arguments)]

use avian2d::prelude::*;
use bevy::{prelude::*, utils::info, window::CursorGrabMode};
use bevy_kira_audio::prelude::*;
use rand::Rng;

use super::{Ball, CursorCrosshair, EndGameTimer, Scores, M4};
use crate::{GameDifficultyState, GameState};

#[derive(Event)]
pub struct ShootingEvent;

#[derive(Event)]
pub struct HitEvent;

pub fn cursor_position(
    mut crosshair_pos: Single<&mut Transform, With<CursorCrosshair>>,
    mut m4_pos: Single<&mut Transform, (With<M4>, Without<CursorCrosshair>)>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    crosshair_pos.translation.x = cursor_pos.x;
    crosshair_pos.translation.y = cursor_pos.y;

    m4_pos.translation.x = cursor_pos.x + 350.0;
    m4_pos.translation.y = cursor_pos.y - 400.0;
}

pub fn ball_contact_checker(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    ball: Query<Entity, With<Ball>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    mut m4: Query<&mut M4>,
    mut hit_event_writer: EventWriter<HitEvent>,
    mut shooting_event_writer: EventWriter<ShootingEvent>,
    mut collision_event_reader: EventReader<Collision>,
) {
    let ball_entity = ball.single();
    let cross_entity = crosshair.single();
    let mut m4_props = m4.single_mut();

    if mouse_input.just_pressed(MouseButton::Left) && m4_props.okay_to_shoot {
        m4_props.okay_to_shoot = false;
        shooting_event_writer.send(ShootingEvent);

        audio.play(asset_server.load("sounds/M4.ogg"));

        for Collision(contacts) in collision_event_reader.read() {
            if contacts.entity1 == ball_entity && contacts.entity2 == cross_entity {
                hit_event_writer.send(HitEvent);

                info("nice");
            }
        }
    }
}

pub fn m4_shooting() {}

pub fn ball_jump(
    mut scores: ResMut<Scores>,
    mut ball: Query<
        (
            &mut ExternalImpulse,
            &mut ExternalTorque,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        With<Ball>,
    >,
    mut event_reader: EventReader<HitEvent>,
) {
    for _event in event_reader.read() {
        for (mut ball_impulse, mut ball_torque, mut ball_vel, mut ball_ang_vel) in &mut ball {
            let mut rng = rand::thread_rng();

            scores.current_score += 1;

            ball_vel.0 = Vec2::ZERO;
            ball_ang_vel.0 = 0.0;
            ball_impulse.apply_impulse(Vec2::new(
                rng.gen_range(-500000.0..500000.0),
                rng.gen_range(500000.0..900000.0),
            ));
            ball_torque.apply_torque(rng.gen_range(-10000000.0..10000000.0));
        }
    }
}

pub fn m4_firerate_timer(mut m4_timer: Single<&mut M4>, time: Res<Time>) {
    if !m4_timer.okay_to_shoot {
        m4_timer.lifetime.tick(time.delta());

        if m4_timer.lifetime.finished() {
            m4_timer.okay_to_shoot = true;
            m4_timer.lifetime = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }
}

pub fn gameover_controller(
    time: Res<Time>,
    mut timer: Query<&mut EndGameTimer>,
    ball: Query<&Transform, With<Ball>>,
    mut window: Single<&mut Window>,
    mut next_gamestate: ResMut<NextState<GameState>>,
) {
    if ball.single().translation.y < -420.0 {
        let mut end_game_timer = timer.single_mut();

        end_game_timer.lifetime.tick(time.delta());

        if end_game_timer.lifetime.finished() {
            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = CursorGrabMode::None;

            next_gamestate.set(GameState::GameOver);
        }
    }
}

pub fn score_writer(
    mut scores: ResMut<Scores>,
    game_difficulty_state: Res<State<GameDifficultyState>>,
) {
    match *game_difficulty_state.get() {
        GameDifficultyState::Easy => {
            scores.high_score = scores.easy_hscore;

            if scores.current_score > scores.easy_hscore {
                scores.easy_hscore = scores.current_score;
                scores.high_score = scores.current_score
            }
        }
        GameDifficultyState::Medium => {
            scores.high_score = scores.medium_hscore;

            if scores.current_score > scores.medium_hscore {
                scores.medium_hscore = scores.current_score;
                scores.high_score = scores.current_score
            }
        }
        GameDifficultyState::Hard => {
            scores.high_score = scores.hard_hscore;

            if scores.current_score > scores.hard_hscore {
                scores.hard_hscore = scores.current_score;
                scores.high_score = scores.current_score
            }
        }
    }
}
