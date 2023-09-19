#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_cursor::prelude::*;
use bevy_rapier2d::prelude::*;

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

#[derive(Component)]
pub struct FireComponent;

pub fn contact_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut contact_event_reader: EventReader<ContactAnimationEvent>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
) {
    for _event in contact_event_reader.iter() {
        commands
            // Spawn a bevy sprite-sheet
            .spawn(SpriteSheetBundle {
                texture_atlas: textures.add(TextureAtlas::from_grid(
                    asset_server.load("sprites\\contact_sheet.png"),
                    Vec2::new(48.0, 48.0),
                    5,
                    1,
                    None,
                    None,
                )),
                transform: Transform::from_xyz(
                    cursor_pos.single().translation.x,
                    cursor_pos.single().translation.y,
                    -2.0,
                ),
                ..default()
            })
            //Create and insert an animation
            .insert(Animation(benimator::Animation::once(
                benimator::Animation::from_indices(0..=4, benimator::FrameRate::from_fps(16.0)),
            )))
            // Insert the state
            .insert(AnimationState::default());
    }
}

pub fn contact_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation), Without<M4>>,
) {
    for (mut anim_state, mut texture, animation) in query.iter_mut() {
        // Update the state
        anim_state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = anim_state.frame_index();
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

pub fn fire_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
    mut m4_event_reader: EventReader<M4AnimationEvent>,
) {
    for _event in m4_event_reader.iter() {
        commands
            // Spawn a bevy sprite-sheet
            .spawn(SpriteSheetBundle {
                texture_atlas: textures.add(TextureAtlas::from_grid(
                    asset_server.load("sprites\\fire_sheet.png"),
                    Vec2::new(432.0, 80.0),
                    1,
                    3,
                    None,
                    None,
                )),
                transform: Transform::from_xyz(
                    cursor_pos.single().translation.x + 150.0,
                    cursor_pos.single().translation.y - 100.0,
                    -1.0,
                ),
                sprite: TextureAtlasSprite {
                    color: Color::rgb(5.0, 5.0, 0.0),
                    ..default()
                },
                ..default()
            })
            //Create and insert an animation
            .insert(Animation(benimator::Animation::once(
                benimator::Animation::from_indices(0..=2, benimator::FrameRate::from_fps(24.0)),
            )))
            // Insert the state
            .insert(FireComponent)
            .insert(AnimationState::default());
    }
}

pub fn fire_animation(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationState, &mut TextureAtlasSprite, &Animation),
        With<FireComponent>,
    >,
) {
    for (mut anim_state, mut texture, animation) in query.iter_mut() {
        // Update the state
        anim_state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = anim_state.frame_index();
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
    for _event in event_reader.iter() {
        for (mut ball_impulse, mut ball_velocity) in &mut ball {
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
    mut contact_event_writer: EventWriter<ContactAnimationEvent>,
    mut play_animation: ResMut<PlayAnimation>,
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
            source: asset_server.load("sounds\\M16.ogg"),
            ..default()
        });

        //check jump ball collide
        if rapier_context.intersection_pair(ball_entity, cross_entity) == Some(true) {
            event_writer.send(JumpBallEvent);
            contact_event_writer.send(ContactAnimationEvent);
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds\\click.ogg"),
                ..default()
            });
        }
    }
}

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
