use bevy::prelude::*;

use crate::earth::EARTH_RADIUS;

const PLAYER_RADIUS: f32 = 20.0;
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

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/player.glb#Scene0"),
            transform: Transform::from_xyz(0.0, EARTH_RADIUS + PLAYER_RADIUS, 0.0)
                .with_scale(Vec3::splat(10.0)),
            ..default()
        },
        Player,
    ));
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
