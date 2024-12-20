use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

use crate::ingame::CursorCrosshair;
use crate::ingame::HitEvent;
use crate::ingame::M4;

#[derive(Component)]
pub struct BulletCase {
    lifetime: Timer,
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub fn sprite_animator(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn contact_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut contact_event_reader: EventReader<HitEvent>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for _event in contact_event_reader.read() {
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 5, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_config_1 = AnimationConfig::new(1, 6, 10);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/contact_sheet.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_config_1.first_sprite_index,
                }),
                ..default()
            },
            Transform::from_xyz(
                cursor_pos.single().translation.x,
                cursor_pos.single().translation.y,
                -2.0,
            ),
        ));
    }
}

/*
pub fn fire_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
    m4_pos: Query<&Transform, With<M4>>,
    mut m4_event_reader: EventReader<M4AnimationEvent>,
) {
    for _event in m4_event_reader.iter() {
        let mut rng = rand::thread_rng();

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
                linvel: Vec2::new(rng.gen_range(4500.0..5500.0), 1000.0),
                angvel: rng.gen_range(-15.0..-5.0),
            });
    }
}



pub fn bullet_case_despawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut casing: Query<(Entity, &mut BulletCase)>,
    time: Res<Time>,
) {
    for (casing_entity, mut casing_timer) in &mut casing {
        casing_timer.lifetime.tick(time.delta());

        if casing_timer.lifetime.finished() {
            commands.entity(casing_entity).despawn();
            audio.play(asset_server.load("sounds/casing.ogg"));
        }
    }
}

*/
