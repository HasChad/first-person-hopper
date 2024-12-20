// #![windows_subsystem = "windows"] //to disable console

use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::prelude::*;

pub mod gameover;
pub mod ingame;
pub mod mainmenu;

use gameover::GameOverPlugin;
use ingame::InGamePlugin;
use mainmenu::MainMenuPlugin;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const CUSTOM_FONT: &str = "fonts/NotoSans-Medium.ttf";

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
    Medium,
    Hard,
}

fn main() {
    App::new()
        //plugins
        .add_plugins(
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
                .build(),
        )
        .add_plugins(AudioPlugin)
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        //systems
        .add_systems(Startup, setup)
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
    //setup camera with debug-render.
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/menu_background.png")),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

pub fn entity_despawner(
    mut commands: Commands,
    mut entity_query: Query<Entity, Without<Camera2d>>,
) {
    for entity in &mut entity_query {
        commands.entity(entity).despawn_recursive();
    }
}
