use bevy::prelude::*;

mod gameover_ui;

use crate::{despawn_screen, GameState};
use gameover_ui::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(Update, (home_button_system, restart_button_system))
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_screen::<GameOverEntity>,
            );
    }
}
