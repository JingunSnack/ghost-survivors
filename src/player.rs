use bevy::prelude::*;

use crate::earth::EARTH_RADIUS;

const PLAYER_RADIUS: f32 = EARTH_RADIUS / 20.0;
const PLAYER_ROTATION_SPEED: f32 = 1.0;
const PLAYER_MOVEMENT_SPEED: f32 = 0.5;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, movement);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: PLAYER_RADIUS,
                    subdivisions: (PLAYER_RADIUS / 2.0) as usize,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.9, 0.9, 0.1),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, EARTH_RADIUS + PLAYER_RADIUS, 0.0),
            ..default()
        },
        Player,
    ));

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: PLAYER_RADIUS,
                subdivisions: (PLAYER_RADIUS / 2.0) as usize,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.7, 0.1),
            ..default()
        }),
        transform: Transform::from_xyz(EARTH_RADIUS + PLAYER_RADIUS, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: PLAYER_RADIUS,
                subdivisions: (PLAYER_RADIUS / 2.0) as usize,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.7, 0.1),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, EARTH_RADIUS + PLAYER_RADIUS),
        ..default()
    });
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if !keyboard_input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
        return;
    }

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let up = player_transform.translation.normalize(); // Up direction from the center of the Earth to the player
        if keyboard_input.pressed(KeyCode::A) {
            let rotation = Quat::from_axis_angle(up, PLAYER_ROTATION_SPEED * time.delta_seconds());
            player_transform.rotate(rotation);
        }
        if keyboard_input.pressed(KeyCode::D) {
            let rotation = Quat::from_axis_angle(up, -PLAYER_ROTATION_SPEED * time.delta_seconds());
            player_transform.rotate(rotation);
        }

        let movement_axis = player_transform.forward().cross(up).normalize(); // Axis to rotate around for movement
        if keyboard_input.pressed(KeyCode::W) {
            let rotation =
                Quat::from_axis_angle(movement_axis, -PLAYER_MOVEMENT_SPEED * time.delta_seconds());
            player_transform.rotate(rotation);
            player_transform.translation = rotation.mul_vec3(player_transform.translation);
        }
        if keyboard_input.pressed(KeyCode::S) {
            let rotation =
                Quat::from_axis_angle(movement_axis, PLAYER_MOVEMENT_SPEED * time.delta_seconds());
            player_transform.rotate(rotation);
            player_transform.translation = rotation.mul_vec3(player_transform.translation);
        }
    }
}
