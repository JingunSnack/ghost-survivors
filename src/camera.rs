use bevy::prelude::*;

use crate::player::Player;

const CAMERA_DISTANCE: f32 = 300.0;

#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, follow_player);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                CAMERA_DISTANCE / 3.0_f32.sqrt(),
                CAMERA_DISTANCE / 3.0_f32.sqrt(),
                CAMERA_DISTANCE / 3.0_f32.sqrt(),
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

fn follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let camera_up = player_transform.translation.normalize();

            camera_transform.translation =
                camera_up * CAMERA_DISTANCE - player_transform.forward() * 20.0;

            camera_transform.look_at(player_transform.translation, camera_up);
        }
    }
}
