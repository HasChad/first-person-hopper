use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ingame::Animation;
use crate::ingame::AnimationState;
use crate::ingame::ContactAnimationEvent;
use crate::ingame::CursorCrosshair;
use crate::ingame::InGameEntity;
use crate::ingame::M4AnimationEvent;
use crate::ingame::M4;

#[derive(Component)]
pub struct BulletCase {
    lifetime: Timer,
}

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
                    asset_server.load("sprites/contact_sheet.png"),
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

pub fn fire_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
    m4_pos: Query<&Transform, With<M4>>,
    mut m4_event_reader: EventReader<M4AnimationEvent>,
) {
    for _event in m4_event_reader.iter() {
        //fire effect spawner
        commands
            // Spawn a bevy sprite-sheet
            .spawn(SpriteSheetBundle {
                texture_atlas: textures.add(TextureAtlas::from_grid(
                    asset_server.load("sprites/fire_sheet.png"),
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
            .insert(BulletCase {
                lifetime: Timer::from_seconds(0.2, TimerMode::Once),
            })
            .insert(AnimationState::default())
            .insert(InGameEntity);

        //bullet case spawner
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("sprites/bullet_case.png"),
                transform: Transform::from_xyz(
                    m4_pos.single().translation.x,
                    m4_pos.single().translation.y + 200.0,
                    -1.0,
                ),
                ..default()
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Velocity {
                linvel: Vec2::new(alea::f32_in_range(4500.0, 5500.0), 1000.0),
                angvel: alea::f32_in_range(-15.0, -5.0),
            });
    }
}

pub fn fire_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation), With<BulletCase>>,
) {
    for (mut anim_state, mut texture, animation) in query.iter_mut() {
        // Update the state
        anim_state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = anim_state.frame_index();
    }
}

pub fn bullet_case_despawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut casing: Query<(Entity, &mut BulletCase)>,
    time: Res<Time>,
) {
    for (casing_entity, mut casing_timer) in &mut casing {
        casing_timer.lifetime.tick(time.delta());

        if casing_timer.lifetime.finished() {
            commands.entity(casing_entity).despawn();
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/casing.ogg"),
                ..default()
            });
        }
    }
}
