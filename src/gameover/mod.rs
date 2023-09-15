use bevy::prelude::*;

mod gameover_ui;

use crate::AppState;
use gameover_ui::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup)
            .add_systems(
                Update,
                (home_button_system, restart_button_system).run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), entity_despawner);
    }
}
