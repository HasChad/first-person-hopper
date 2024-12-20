use bevy::prelude::*;

mod mainmenu_ui;

use mainmenu_ui::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                (easy_button_system, medium_button_system, hard_button_system),
            );
    }
}
