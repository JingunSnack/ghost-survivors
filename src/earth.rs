use bevy::prelude::*;

pub const EARTH_RADIUS: f32 = 100.0;

#[derive(Component)]
pub struct Earth;

pub struct EarthPlugin;

impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
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
                    radius: EARTH_RADIUS,
                    subdivisions: (EARTH_RADIUS / 2.0) as usize,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.7),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Earth,
    ));
}
