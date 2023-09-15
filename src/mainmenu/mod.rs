use bevy::prelude::*;

mod mainmenu_ui;

use crate::AppState;
use mainmenu_ui::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(
                Update,
                (easy_button_system, medium_button_system, hard_button_system)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), entity_despawner);
    }
}
