use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Game), spawn)
            .add_systems(Update, update.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn)
            .add_systems(OnExit(GameState::GameOver), reset);
    }
}

fn spawn(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            ScoreUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Score: {}", score.value),
                    TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
                ScoreText,
            ));
        });
}

fn despawn(mut commands: Commands, score_ui_query: Query<Entity, With<ScoreUI>>) {
    for score_ui_entity in &score_ui_query {
        commands.entity(score_ui_entity).despawn_recursive();
    }
}

fn update(score: Res<Score>, mut score_query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut score_text) = score_query.get_single_mut() {
            score_text.sections[0].value = format!("Score: {}", score.value);
        }
    }
}

fn reset(mut score: ResMut<Score>) {
    score.value = 0;
}
