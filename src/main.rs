#![windows_subsystem = "windows"] //to disable console

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    window::{PrimaryWindow, WindowMode},
    winit::WinitWindows,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::*;
use winit::window::Icon;

pub mod gameover;
pub mod ingame;
pub mod mainmenu;

use gameover::GameOverPlugin;
use ingame::InGamePlugin;
use mainmenu::MainMenuPlugin;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameDifficultyState {
    Easy,
    #[default]
    Medium,
    Hard,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "First Person Hopper".into(),
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        resizable: false,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
            AudioPlugin,
        ))
        .add_systems(Startup, (setup, setup_window))
        .add_state::<AppState>()
        .add_state::<GameDifficultyState>()
        .add_plugins(InGamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameOverPlugin)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //setup camera with debug-render.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/menu_background.png"),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..default()
    });
}

pub fn setup_window(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/game_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}
