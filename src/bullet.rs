use bevy::prelude::*;

use crate::enemy::{Enemy, HitByBullet};
use crate::player::Player;

const HIT_BY_WEAPON_LIFESPAN: f32 = 0.25;
const BULLET_LIFESPAN: f32 = 10.0;
const BULLET_MOVEMENT_SPEED: f32 = 2.5;
const BULLET_DAMAGE: f32 = 100.0;

#[derive(Component)]
pub struct Bullet {
    elapesed_time: f32,
    lifespan: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn.run_if(bevy::time::common_conditions::on_timer(
                std::time::Duration::from_secs(1),
            )),
        )
        .add_systems(Update, hit)
        .add_systems(Update, translate)
        .add_systems(Update, despawn);
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
            Bullet {
                elapesed_time: 0.0,
                lifespan: BULLET_LIFESPAN,
            },
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

fn hit(
    mut commands: Commands,
    weapon_query: Query<&Transform, With<Bullet>>,
    mut enemy_query: Query<
        (Entity, &Transform, &mut Enemy),
        (With<Enemy>, Without<Bullet>, Without<HitByBullet>),
    >,
) {
    for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
        for weapon_transform in &weapon_query {
            if enemy_transform
                .translation
                .distance(weapon_transform.translation)
                < enemy.radius
            {
                let direction = enemy_transform.translation - weapon_transform.translation;
                enemy.health -= BULLET_DAMAGE;

                commands.entity(enemy_entity).insert(HitByBullet {
                    bullet_direction: direction,
                    elapsed_time: 0.0,
                    lifespan: HIT_BY_WEAPON_LIFESPAN,
                    is_lethal: enemy.health <= 0.0,
                });
                break;
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in &mut bullet_query {
        bullet.elapesed_time += time.delta_seconds();
        if bullet.elapesed_time >= bullet.lifespan {
            commands.entity(entity).despawn_recursive();
        }
    }
}
