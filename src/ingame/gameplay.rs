#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::ingame::Animation;
use crate::ingame::AnimationState;
use crate::ingame::Ball;
use crate::ingame::CursorCrosshair;
use crate::ingame::EndGameTimer;
use crate::ingame::InGameEntity;
use crate::ingame::Scores;
use crate::ingame::M4;
use crate::AppState;

#[derive(Resource)]
pub struct PlayAnimation(pub bool);

#[derive(Event)]
pub struct JumpBallEvent;

#[derive(Event)]
pub struct ContactAnimationEvent;

#[derive(Event)]
pub struct M4AnimationEvent;

pub fn cursor_position(
    mut crosshair: Query<&mut Transform, With<CursorCrosshair>>,
    mut m4: Query<&mut Transform, (With<M4>, Without<CursorCrosshair>)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    // alternative mouse position finder
    let (camera, camera_transform) = camera_q.single();

    let Some(mouse_position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    else {
        return;
    };

    let mut m4_position = m4.single_mut();

    for mut crosshair_pos in &mut crosshair {
        //crosshair position
        crosshair_pos.translation.x = mouse_position.x;
        crosshair_pos.translation.y = mouse_position.y;
        //m4 position relative to cursor position
        m4_position.translation.x = mouse_position.x + 350.0;
        m4_position.translation.y = mouse_position.y - 400.0;
    }
}

pub fn ball_movement(
    mut scores: ResMut<Scores>,
    mut ball: Query<(&mut ExternalImpulse, &mut Velocity), With<Ball>>,
    mut event_reader: EventReader<JumpBallEvent>,
) {
    //jump ball if collide eachother
    for _event in event_reader.iter() {
        for (mut ball_impulse, mut ball_velocity) in &mut ball {
            let mut rng = rand::thread_rng();

            scores.current_score += 1;
            info!("{}", scores.current_score);
            ball_velocity.linvel.y = 0.0;
            ball_velocity.linvel.x = 0.0;
            ball_velocity.angvel = 0.0;
            ball_impulse.impulse.y = rng.gen_range(500000.0..900000.0);
            ball_impulse.impulse.x = rng.gen_range(-500000.0..500000.0);
            ball_impulse.torque_impulse = rng.gen_range(-10000000.0..10000000.0);
        }
    }
}

pub fn ball_contact_checker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    ball: Query<Entity, With<Ball>>,
    crosshair: Query<Entity, With<CursorCrosshair>>,
    mut m4: Query<&mut M4>,
    mut play_animation: ResMut<PlayAnimation>,
    mut event_writer: EventWriter<JumpBallEvent>,
    mut contact_event_writer: EventWriter<ContactAnimationEvent>,
    mut m4_animation_event: EventWriter<M4AnimationEvent>,
) {
    let ball_entity = ball.single();
    let cross_entity = crosshair.single();
    let mut m4_props = m4.single_mut();

    if input.just_pressed(MouseButton::Left) && m4_props.okay_to_shoot {
        //m4 sound play and 0 rate of fire
        m4_props.okay_to_shoot = false;
        play_animation.0 = true;
        m4_animation_event.send(M4AnimationEvent);
        info!("{:?}", play_animation.0);
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/M4.ogg"),
            ..default()
        });

        //check jump ball collide
        if rapier_context.intersection_pair(ball_entity, cross_entity) == Some(true) {
            event_writer.send(JumpBallEvent);
            contact_event_writer.send(ContactAnimationEvent);
        }
    }
}

pub fn m4_firerate_timer(mut m4: Query<&mut M4>, time: Res<Time>) {
    let mut m4_timer = m4.single_mut();

    if !m4_timer.okay_to_shoot {
        m4_timer.lifetime.tick(time.delta());

        if m4_timer.lifetime.finished() {
            m4_timer.okay_to_shoot = true;
            m4_timer.lifetime = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }
}

pub fn m4_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation), With<M4>>,
    mut play_animation: ResMut<PlayAnimation>,
) {
    if play_animation.0 {
        for (mut anim_state, mut texture, animation) in query.iter_mut() {
            // Update the state
            anim_state.update(animation, time.delta());

            // Update the texture atlas
            texture.index = anim_state.frame_index();

            if anim_state.frame_index() == 4 {
                play_animation.0 = false;
                anim_state.reset();
            }
        }
    }
}

pub fn gameover_controller(
    mut commands: Commands,
    mut timer: Query<&mut EndGameTimer>,
    ball: Query<&Transform, With<Ball>>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
) {
    if ball.single().translation.y < -420.0 {
        let mut end_game_timer = timer.single_mut();

        end_game_timer.lifetime.tick(time.delta());

        if end_game_timer.lifetime.finished() {
            //enable cursor
            let mut window = windows.single_mut();
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            //change state
            commands.insert_resource(NextState(Some(AppState::GameOver)));
        }
    }
}

pub fn entity_despawner(
    mut commands: Commands,
    ball: Query<&Ball>,
    mut entities: Query<Entity, With<InGameEntity>>,
    mut scores: ResMut<Scores>,
) {
    let ball_diff = ball.single();

    match ball_diff {
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

    //despawn everyting in InGame
    for entities_despawner in &mut entities {
        commands.entity(entities_despawner).despawn();
    }
}
