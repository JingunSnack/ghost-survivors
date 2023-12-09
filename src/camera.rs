use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 400.0;

#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
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
