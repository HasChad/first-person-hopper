use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
struct HomeButton;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct GameOverEntity;

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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("gameover menu activated");
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites\\menu_background.png"),
            transform: Transform::from_xyz(0.0, 0.0, -4.0),
            ..default()
        })
        .insert(GameOverEntity);

    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .insert(GameOverEntity)
        //spawn easy button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
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
                .insert(HomeButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "MAIN MENU",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            //spawn medium button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
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
                .insert(RestartButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RESTART",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn home_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HomeButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn restart_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn entity_despawner(mut entities: Query<Entity, With<GameOverEntity>>, mut commands: Commands) {
    info!("Main Menu Despawner Activated");

    //despawn everyting in InGame
    for entities_despawner in &mut entities {
        commands.entity(entities_despawner).despawn_recursive();
    }
}
