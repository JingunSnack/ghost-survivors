use bevy::prelude::*;

use crate::{score::Score, GameState};

#[derive(Component)]
pub struct GameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn)
            .add_systems(OnExit(GameState::GameOver), despawn);
    }
}

fn spawn(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    margin: UiRect {
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            GameOver,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("Score: {}\nPress Enter to Play Again", score.value),
                    TextStyle {
                        font_size: 64.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}

fn despawn(mut commands: Commands, game_over_query: Query<Entity, With<GameOver>>) {
    for entity in &game_over_query {
        commands.entity(entity).despawn_recursive();
    }
}
