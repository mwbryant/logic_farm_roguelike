use bevy::prelude::*;

use crate::Money;

pub struct GameUI;

#[derive(Component)]
pub struct MoneyText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_money_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text::from_section("Money!", TextStyle::default()),
            ..default()
        },
        MoneyText,
    ));
}

fn update_money_ui(mut texts: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: ${:?}", money.0);
    }
}
