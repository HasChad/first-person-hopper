use avian2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_kira_audio::prelude::*;
use rand::random_range;

use super::{Ball, CursorCrosshair, EndGameTimer, M4, Scores};
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
    let cursor_position = windows.single().cursor_position().unwrap();
    let Ok(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    crosshair_pos.translation.x = cursor_pos.x;
    crosshair_pos.translation.y = cursor_pos.y;

    m4_pos.translation.x = cursor_pos.x + 350.0;
    m4_pos.translation.y = cursor_pos.y - 400.0;
}

pub fn m4_shooting(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut m4_props: Single<&mut M4>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut shooting_event_writer: EventWriter<ShootingEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) && m4_props.okay_to_shoot {
        m4_props.okay_to_shoot = false;
        audio.play(asset_server.load("sounds/M4.ogg"));
        shooting_event_writer.send(ShootingEvent);
    }
}

pub fn m4_firerate_timer(mut m4_timer: Single<&mut M4>, time: Res<Time>) {
    if !m4_timer.okay_to_shoot {
        m4_timer.lifetime.tick(time.delta());

        if m4_timer.lifetime.finished() {
            m4_timer.okay_to_shoot = true;
            m4_timer.lifetime.reset();
        }
    }
}

pub fn enable_ball_physics(
    mut commands: Commands,
    scores: Res<Scores>,
    query: Query<Entity, (With<RigidBody>, With<Ball>)>,
) {
    if scores.current_score == 1 {
        for entity in &query {
            commands.entity(entity).remove::<RigidBodyDisabled>();
        }
    }
}

pub fn ball_contact_checker(
    ball: Query<Entity, With<Ball>>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    mut hit_event_writer: EventWriter<HitEvent>,
    mut shooting_event_reader: EventReader<ShootingEvent>,
    mut collision_event_reader: EventReader<Collision>,
) {
    let ball_entity = ball.single();
    let cross_entity = crosshair.single();

    for _event in shooting_event_reader.read() {
        for Collision(contacts) in collision_event_reader.read() {
            // info!("ent1 = {}", contacts.entity1);
            // info!("ent2 = {}", contacts.entity2);
            if contacts.entity1 == ball_entity && contacts.entity2 == cross_entity {
                hit_event_writer.send(HitEvent);
                break;
            }
        }
    }
}

pub fn ball_jump(
    mut scores: ResMut<Scores>,
    mut ball: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
        ),
        With<Ball>,
    >,
    mut event_reader: EventReader<HitEvent>,
) {
    for _event in event_reader.read() {
        for (mut ball_vel, mut ball_ang_vel, mut ball_imp, mut ball_ang_imp) in &mut ball {
            scores.current_score += 1;

            ball_vel.0 = Vec2::ZERO;
            ball_ang_vel.0 = 0.0;
            ball_imp.apply_impulse(Vec2::new(
                random_range(-400.0..400.0),
                random_range(700.0..1000.0),
            ));
            ball_ang_imp.apply_impulse(random_range(-5000.0..5000.0));
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

pub fn score_saver(
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
        GameDifficultyState::Normal => {
            scores.high_score = scores.normal_hscore;

            if scores.current_score > scores.normal_hscore {
                scores.normal_hscore = scores.current_score;
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
