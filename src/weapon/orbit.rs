use bevy::prelude::*;

use crate::player::Player;
use crate::weapon::Weapon;
use crate::GameState;

const ORBIT_DAMAGE: f32 = 100.0;
const ORBIT_RADIUS: f32 = 25.0;
const ORBIT_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Orbit {
    angle: f32,
}

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, translate.run_if(in_state(GameState::Game)));
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
                    radius: 5.0,
                    subdivisions: 2,
                })
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.1, 0.9, 0.9, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(100000.0, 100000.0, 100000.0),
            ..default()
        },
        Orbit { angle: 0.0 },
        Weapon {
            elapesed_time: 0.0,
            lifespan: -1.0,
            damage: ORBIT_DAMAGE,
        },
    ));
}

fn translate(
    mut orbit_query: Query<(&mut Transform, &mut Orbit), With<Orbit>>,
    player_query: Query<&Transform, (With<Player>, Without<Orbit>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut orbit_transform, mut orbit) in &mut orbit_query {
            orbit.angle += ORBIT_SPEED * time.delta_seconds();
            orbit.angle %= 2.0 * std::f32::consts::PI;

            // Calculate the up direction (from the center of the Earth to the player)
            let up = player_transform.translation.normalize();

            // Calculate the right direction (perpendicular to the up direction)
            let right = if up.y.abs() != 1.0 {
                Vec3::Y.cross(up).normalize()
            } else {
                Vec3::X.cross(up).normalize()
            };

            // Calculate the forward direction (perpendicular to both right and up)
            let forward = right.cross(up);

            // Determine the orbit position using polar coordinates on the perpendicular plane
            let orbit_pos = right * ORBIT_RADIUS * orbit.angle.cos()
                + forward * ORBIT_RADIUS * orbit.angle.sin();

            // Set the orbiting object's position relative to the player
            orbit_transform.translation = player_transform.translation + orbit_pos;
        }
    }
}
