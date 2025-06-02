use avian2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode, winit::cursor::CursorIcon};
use bevy_kira_audio::prelude::*;
use rand::random_range;

use super::{animations::AnimationConfig, Ball, EndGameTimer, Gun, Scores};
use crate::{GameDifficultyState, GameState, SCREEN_HEIGHT};

#[derive(Event)]
pub struct ShootingEvent;

#[derive(Event)]
pub struct HitEvent;

pub fn cursor_position(
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut gun_pos: Single<&mut Transform, With<Gun>>,
) {
    let (camera, camera_transform) = *camera_query;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let cursor_pos = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .unwrap();

    gun_pos.translation.x = cursor_pos.x + 250.0;
    gun_pos.translation.y = cursor_pos.y - 250.0;
}

pub fn gun_shooting(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut gun_props: Query<(&mut Gun, &mut AnimationConfig)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut shooting_event_writer: EventWriter<ShootingEvent>,
) {
    for (mut gun_prop, mut anim_config) in gun_props.iter_mut() {
        if mouse_input.just_pressed(MouseButton::Left) && gun_prop.okay_to_shoot {
            gun_prop.okay_to_shoot = false;
            audio.play(asset_server.load("sounds/gun_shot.ogg"));
            shooting_event_writer.write(ShootingEvent);

            anim_config.play = true;
        }
    }
}

pub fn gun_firerate_timer(mut gun_timer: Single<&mut Gun>, time: Res<Time>) {
    if !gun_timer.okay_to_shoot {
        gun_timer.lifetime.tick(time.delta());

        if gun_timer.lifetime.finished() {
            gun_timer.okay_to_shoot = true;
            gun_timer.lifetime.reset();
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
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ball_entity: Single<Entity, With<Ball>>,
    mut scores: ResMut<Scores>,
    mut hit_event_writer: EventWriter<HitEvent>,
    mut shooting_event_reader: EventReader<ShootingEvent>,
    spatial_query: SpatialQuery,
) {
    for _event in shooting_event_reader.read() {
        let (camera, camera_transform) = *camera_query;
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let cursor_pos = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap();

        let intersections =
            spatial_query.point_intersections(cursor_pos, &SpatialQueryFilter::default());

        for entity in intersections.iter() {
            if *entity == *ball_entity {
                hit_event_writer.write(HitEvent);
                scores.current_score += 1;
            }
        }
    }
}

pub fn ball_jump(
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
            ball_vel.0 = Vec2::ZERO;
            ball_ang_vel.0 = 0.0;
            ball_imp.apply_impulse(Vec2::new(
                random_range(-600.0..600.0),
                random_range(700.0..1200.0),
            ));
            ball_ang_imp.apply_impulse(random_range(-5000.0..5000.0));
        }
    }
}

pub fn gameover_controller(
    time: Res<Time>,
    mut end_game_timer: Single<&mut EndGameTimer>,
    ball: Single<&Transform, With<Ball>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
    window_entity: Single<Entity, With<Window>>,
    mut next_gamestate: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if ball.translation.y < SCREEN_HEIGHT / -2.0 {
        end_game_timer.lifetime.tick(time.delta());

        if end_game_timer.lifetime.finished() {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            commands
                .entity(*window_entity)
                .insert(CursorIcon::System(bevy::window::SystemCursorIcon::Default));

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
