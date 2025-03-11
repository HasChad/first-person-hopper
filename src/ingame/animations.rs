use std::time::Duration;

use bevy::prelude::*;

use crate::ingame::CursorCrosshair;
use crate::ingame::Gun;
use crate::ingame::HitEvent;

use super::ShootingEvent;

#[derive(Component)]
pub struct BulletCase {
    lifetime: Timer,
}

#[derive(Component)]
pub struct MuzzleFlash {
    lifetime: Timer,
}

#[derive(Component)]
pub struct ContactSprite;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub play: bool,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, play: bool) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            play,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub fn sprite_animator(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        if config.play {
            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.last_sprite_index {
                        atlas.index = config.first_sprite_index;
                        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                        config.play = false;
                    } else {
                        atlas.index += 1;
                        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                    }
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
        let animation_config = AnimationConfig::new(0, 4, 50, true);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/contact_sheet.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_config.first_sprite_index,
                }),
                ..default()
            },
            Transform::from_xyz(
                cursor_pos.single().translation.x,
                cursor_pos.single().translation.y,
                -2.0,
            ),
            ContactSprite,
            animation_config,
        ));
    }
}

/*
pub fn fire_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    cursor_pos: Query<&Transform, With<CursorCrosshair>>,
    gun_pos: Query<&Transform, With<GUN>>,
    mut gun_event_reader: EventReader<GUNAnimationEvent>,
) {
    for _event in gun_event_reader.iter() {
        commands
            .spawn((SpriteSheetBundle {
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
            },
            InGameEntity,
            ));
    }
}
*/

pub fn bullet_case_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gun_pos: Single<&Transform, With<Gun>>,
    mut shooting_event_reader: EventReader<ShootingEvent>,
) {
    for _event in shooting_event_reader.read() {
        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/bullet_case.png")),
            Transform::from_xyz(gun_pos.translation.x, gun_pos.translation.y + 200.0, -1.0),
            BulletCase {
                lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/muzzle_flash.png")),
            Transform::from_xyz(
                gun_pos.translation.x - 120.0,
                gun_pos.translation.y + 180.0,
                -1.0,
            ),
            MuzzleFlash {
                lifetime: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ));
    }
}

pub fn bullet_case_controller(
    mut commands: Commands,
    mut casing: Query<(Entity, &mut Transform, &mut BulletCase)>,
    time: Res<Time>,
) {
    for (casing_entity, mut casing_transform, mut casing_timer) in &mut casing {
        casing_timer.lifetime.tick(time.delta());
        casing_transform.translation += Vec3::new(5000.0, 1000.0, 0.0) * time.delta_secs();
        casing_transform.rotate_z(-30.0 * time.delta_secs());

        if casing_timer.lifetime.finished() {
            commands.entity(casing_entity).despawn();
        }
    }
}

pub fn muzzle_flash_controller(
    mut commands: Commands,
    mut casing: Query<(Entity, &mut Sprite, &mut MuzzleFlash)>,
    time: Res<Time>,
) {
    for (casing_entity, mut casing_sprite, mut casing_timer) in &mut casing {
        casing_timer.lifetime.tick(time.delta());

        casing_sprite.color =
            Color::srgba(1.0, 1.0, 1.0, casing_timer.lifetime.remaining_secs() / 0.1);

        if casing_timer.lifetime.finished() {
            commands.entity(casing_entity).despawn();
        }
    }
}
