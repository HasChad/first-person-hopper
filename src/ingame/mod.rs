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

#[derive(Resource)]
pub struct Scores {
    pub current_score: i32,
    pub high_score: i32,
    pub easy_hscore: i32,
    pub medium_hscore: i32,
    pub hard_hscore: i32,
}

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
pub struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            // ! .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(CursorInfoPlugin)
            .add_event::<JumpBallEvent>()
            .add_event::<ContactAnimationEvent>()
            .add_event::<M4AnimationEvent>()
            .insert_resource(PlayAnimation(false))
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
                    game_difficulty_easy.run_if(in_state(GameDifficultyState::Easy)),
                    game_difficulty_medium.run_if(in_state(GameDifficultyState::Medium)),
                    game_difficulty_hard.run_if(in_state(GameDifficultyState::Hard)),
                ),
            )
            .add_systems(
                Update,
                (
                    contact_spawn,
                    contact_animation,
                    fire_spawn,
                    fire_animation,
                    m4_animation,
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
