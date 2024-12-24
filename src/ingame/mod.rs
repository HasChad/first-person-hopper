use bevy::prelude::*;

pub mod animations;
pub mod gameplay;
pub mod ingame_ui;
pub mod spawn;

use animations::*;
use gameplay::*;
use ingame_ui::*;
use spawn::*;

use crate::despawn_screen;
use crate::GameState;

#[derive(Resource)]
pub struct Scores {
    pub current_score: i32,
    pub high_score: i32,
    pub easy_hscore: i32,
    pub medium_hscore: i32,
    pub hard_hscore: i32,
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (setup, ui_setup))
            .add_systems(
                Update,
                (
                    // gameplay
                    pause,
                    cursor_position,
                    m4_shooting,
                    m4_firerate_timer,
                    ball_jump,
                    ball_contact_checker,
                    gameover_controller,
                    score_saver,
                    // animation
                    contact_spawn,
                    m4_sprite_animator,
                    contact_sprite_animator,
                    // ui
                    score_text_updater,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_screen::<InGameEntity>)
            .add_event::<HitEvent>()
            .add_event::<ShootingEvent>()
            .insert_resource(Scores {
                current_score: 0,
                high_score: 0,
                easy_hscore: 0,
                medium_hscore: 0,
                hard_hscore: 0,
            });
    }
}
