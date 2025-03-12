// #![windows_subsystem = "windows"] //disable console on windows

use avian2d::prelude::*;
use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::query::QueryData,
    prelude::*,
    render::camera::ScalingMode,
    window::WindowMode,
};
use bevy_kira_audio::prelude::*;

pub mod gameover;
pub mod ingame;
pub mod mainmenu;

use gameover::GameOverPlugin;
use ingame::InGamePlugin;
use mainmenu::MainMenuPlugin;

const SCREEN_WIDTH: f32 = 1920.0;
// const SCREEN_HEIGHT: f32 = 1080.0;
const CUSTOM_FONT: &str = "fonts/Gunplay.ttf";
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::WHITE;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ButtonQuery {
    interaction: &'static Interaction,
    color: &'static mut BackgroundColor,
    border_color: &'static mut BorderColor,
}

#[derive(Component)]
pub struct FpsText;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameDifficultyState {
    #[default]
    Easy,
    Normal,
    Hard,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "First Person Hopper".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        mode: WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        //plugins
        .add_plugins(AudioPlugin)
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        //systems
        .add_systems(Startup, setup)
        .add_systems(Update, fps_text_updater)
        //states
        .init_state::<GameState>()
        .init_state::<GameDifficultyState>()
        //mod plugins
        .add_plugins(InGamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameOverPlugin)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal {
                viewport_width: SCREEN_WIDTH,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // version text
    commands.spawn((
        Text::new("v2.0alpha_test"),
        TextFont {
            font: asset_server.load(CUSTOM_FONT),
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));

    // FPS UI
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::End,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            Name::new("UI - FPSCounter"),
            FpsText,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(GOLD.into()),
        ));
}

pub fn fps_text_updater(
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
    entity: Single<Entity, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            *writer.text(*entity, 1) = format!("{value:.0}");
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
