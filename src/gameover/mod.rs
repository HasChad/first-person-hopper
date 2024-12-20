use bevy::{prelude::*, winit::WinitSettings};

mod gameover_ui;

use crate::GameState;
use gameover_ui::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(Update, home_button_system)
            .add_systems(Update, restart_button_system)
            .insert_resource(WinitSettings::desktop_app());
    }
}
