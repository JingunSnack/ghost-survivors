use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn)
            .add_systems(OnExit(GameState::Menu), despawn);
    }
}

fn spawn(mut commands: Commands) {
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
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Press Enter to Play",
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

fn despawn(mut commands: Commands, menu_query: Query<Entity, With<Menu>>) {
    for menu_entity in &menu_query {
        commands.entity(menu_entity).despawn_recursive();
    }
}
