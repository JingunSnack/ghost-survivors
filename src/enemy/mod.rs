use bevy::prelude::*;

use crate::earth::EARTH_RADIUS;
use crate::player::Player;

pub mod dragon;
pub mod ghost;
pub mod knight;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct HitByWeapon {
    pub elapsed_time: f32,
    pub lifespan: f32,
    pub is_lethal: bool,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player)
            .add_systems(Update, hit_by_weapon)
            .add_systems(Update, despawn_when_hit_by_weapon.after(hit_by_weapon));
    }
}

fn follow_player(
    mut enemy_query: Query<(&Enemy, &mut Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy, mut enemy_transform) in &mut enemy_query {
            let direction =
                (player_transform.translation - enemy_transform.translation).normalize();

            let enemy_position =
                enemy_transform.translation + direction * enemy.speed * time.delta_seconds();

            if enemy_position.distance(Vec3::ZERO) >= EARTH_RADIUS - enemy.radius / 2.0 {
                enemy_transform.translation = enemy_position;
            }

            enemy_transform.look_at(player_transform.translation, player_transform.translation)
        }
    }
}

fn hit_by_weapon(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Visibility), With<HitByWeapon>>,
) {
    for (mut transform, mut visibility) in query.iter_mut() {
        let forward = transform.forward();
        transform.translation += forward * -2.0 * time.delta_seconds();

        match *visibility {
            Visibility::Hidden => *visibility = Visibility::Visible,
            Visibility::Visible => *visibility = Visibility::Hidden,
            Visibility::Inherited => *visibility = Visibility::Visible,
        }
    }
}

fn despawn_when_hit_by_weapon(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility, &mut HitByWeapon), With<HitByWeapon>>,
    time: Res<Time>,
) {
    for (entity, mut visibility, mut hit_by_weapon) in &mut query {
        hit_by_weapon.elapsed_time += time.delta_seconds();
        if hit_by_weapon.elapsed_time >= hit_by_weapon.lifespan {
            commands.entity(entity).remove::<HitByWeapon>();
            *visibility = Visibility::Visible;
            if hit_by_weapon.is_lethal {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
