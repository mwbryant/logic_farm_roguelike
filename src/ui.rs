use bevy::prelude::*;

use crate::{GameState, Money};
#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct MainMenu;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_start_button)
            .add_systems(OnEnter(GameState::Gameplay), spawn_game_ui)
            .add_systems(
                Update,
                check_start_button.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), despawn_start_button)
            .add_systems(Update, update_money_ui);
    }
}

fn spawn_start_button(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(10.0),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLUE.into(),
                        ..default()
                    },
                    MainMenu,
                    Name::new("Start Button"),
                ))
                .with_children(|commands| {
                    commands.spawn((TextBundle {
                        text: Text::from_section(
                            "Start Game",
                            TextStyle {
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        ..default()
                    },));
                });
        });
}

fn check_start_button(
    start_button: Query<&Interaction, With<MainMenu>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let button = start_button.single();
    match button {
        Interaction::Pressed => next_state.set(GameState::Gameplay),
        Interaction::None | Interaction::Hovered => {}
    }
}

fn despawn_start_button(mut commands: Commands, start_menu: Query<Entity, With<MainMenu>>) {
    for entity in &start_menu {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                MoneyText,
            ));
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: ${:?}", money.0);
    }
}
