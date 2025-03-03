use bevy::prelude::*;

mod mainmenu_ui;

use crate::{GameState, despawn_screen};
use mainmenu_ui::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                (
                    easy_button_system,
                    normal_button_system,
                    hard_button_system,
                    quit_button_system,
                ),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_screen::<MainMenuEntity>,
            );
    }
}
