use bevy::prelude::*;

use crate::enemy::{Enemy, HitByWeapon};

pub mod bullet;

const HIT_BY_WEAPON_LIFESPAN: f32 = 0.25;

#[derive(Component)]
pub struct Weapon {
    elapesed_time: f32,
    lifespan: f32,
    damage: f32,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hit).add_systems(Update, despawn);
    }
}

fn hit(
    mut commands: Commands,
    weapon_query: Query<(&Weapon, &Transform), With<Weapon>>,
    mut enemy_query: Query<
        (Entity, &Transform, &mut Enemy),
        (With<Enemy>, Without<Weapon>, Without<HitByWeapon>),
    >,
) {
    for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
        for (weapon, weapon_transform) in &weapon_query {
            if enemy_transform
                .translation
                .distance(weapon_transform.translation)
                < enemy.radius
            {
                enemy.health -= weapon.damage;

                commands.entity(enemy_entity).insert(HitByWeapon {
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
    mut weapon_query: Query<(Entity, &mut Weapon), With<Weapon>>,
    time: Res<Time>,
) {
    for (entity, mut weapon) in &mut weapon_query {
        weapon.elapesed_time += time.delta_seconds();
        if weapon.elapesed_time >= weapon.lifespan {
            commands.entity(entity).despawn_recursive();
        }
    }
}
