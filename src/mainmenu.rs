use bevy::prelude::*;

use crate::AppState;
use crate::GameDifficultyState;

#[derive(Component)]
struct EasyButton;

#[derive(Component)]
struct MediumButton;

#[derive(Component)]
struct HardButton;

#[derive(Component)]
struct MainMenuEntity;

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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\menu_background.png"),
            transform: Transform::from_xyz(0.0, 0.0, -4.0),
            ..default()
        })
        .insert(MainMenuEntity);
    //spawn full screen node bundle
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                // vertically center child text
                align_items: AlignItems::Center,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .insert(MainMenuEntity)
        //spawn easy button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(EasyButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "EASY",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.19, 0.76, 0.41),
                            ..default()
                        },
                    ));
                });

            //spawn medium button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MediumButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "MEDIUM",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.35, 0.67, 0.89),
                            ..default()
                        },
                    ));
                });

            //spawn hard button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(HardButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "HARD",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.88, 0.21, 0.20),
                            ..default()
                        },
                    ));
                });
        });
}

fn easy_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EasyButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Easy)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds\\hover_button.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn medium_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MediumButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Medium)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds\\hover_button.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn hard_button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HardButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                commands.insert_resource(NextState(Some(GameDifficultyState::Hard)))
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds\\hover_button.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn entity_despawner(mut entities: Query<Entity, With<MainMenuEntity>>, mut commands: Commands) {
    info!("Main Menu Despawner Activated");

    //despawn everyting in InGame
    for entities_despawner in &mut entities {
        commands.entity(entities_despawner).despawn_recursive();
    }
}
