use bevy::prelude::*;
use bevy_cursor::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod gameplay;
pub mod ingame_ui;
pub mod spawn;

use crate::AppState;
use crate::GameDifficultyState;
use gameplay::*;
use ingame_ui::*;
use spawn::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // ! .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(CursorInfoPlugin)
            .add_event::<JumpBallEvent>()
            .insert_resource(Scores {
                current_score: 0,
                high_score: 0,
                easy_hscore: 0,
                medium_hscore: 0,
                hard_hscore: 0,
            })
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    setup,
                    ui_setup,
                    game_difficulty_medium.run_if(in_state(GameDifficultyState::Medium)),
                    game_difficulty_hard.run_if(in_state(GameDifficultyState::Hard)),
                    game_difficulty_easy.run_if(in_state(GameDifficultyState::Easy)),
                ),
            )
            .add_systems(
                Update,
                (
                    cursor_position,
                    ball_movement,
                    ball_contact_checker,
                    entity_despawner,
                    m4_shooting,
                    ui_update,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
