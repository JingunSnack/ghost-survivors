use bevy::prelude::*;

use crate::player::Player;
use crate::weapon::Weapon;

const BULLET_LIFESPAN: f32 = 10.0;
const BULLET_MOVEMENT_SPEED: f32 = 2.5;
const BULLET_DAMAGE: f32 = 100.0;

#[derive(Component)]
pub struct Bullet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn.run_if(bevy::time::common_conditions::on_timer(
                std::time::Duration::from_secs(1),
            )),
        )
        .add_systems(Update, translate);
    }
}

fn spawn(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
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
                    base_color: Color::rgba(0.9, 0.9, 0.1, 0.5),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),

                transform: Transform::from_translation(player_transform.translation)
                    .looking_at(player_transform.forward(), player_transform.translation),
                ..default()
            },
            Weapon {
                elapesed_time: 0.0,
                lifespan: BULLET_LIFESPAN,
                damage: BULLET_DAMAGE,
            },
            Bullet,
        ));
    }
}

fn translate(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut bullet_transform in &mut bullet_query {
        let up = bullet_transform.translation.normalize();
        let movement_axis = bullet_transform.forward().cross(up).normalize(); // Axis to rotate around for movement
        let rotation =
            Quat::from_axis_angle(movement_axis, -BULLET_MOVEMENT_SPEED * time.delta_seconds());
        bullet_transform.rotate(rotation);
        bullet_transform.translation = rotation.mul_vec3(bullet_transform.translation);
    }
}
