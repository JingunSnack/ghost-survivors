use bevy::prelude::*;

mod camera;

use crate::camera::MainCameraPlugin;

const EARTH_RADIUS: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(MainCameraPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: EARTH_RADIUS,
                subdivisions: 50,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.1, 0.7),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
