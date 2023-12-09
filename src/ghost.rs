use bevy::prelude::*;
use rand::Rng;

use crate::earth::EARTH_RADIUS;
use crate::player::Player;

const GHOST_RADIUS: f32 = 30.0;
const GHOST_MOVEMENT_SPEED: f32 = 20.0;
const MIN_DISTANCE_FROM_PLAYER: f32 = 100.0;
const SPAWN_INTERVAL: u64 = 1;

#[derive(Component)]
pub struct Ghost;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn.run_if(bevy::time::common_conditions::on_timer(
                std::time::Duration::from_secs(SPAWN_INTERVAL),
            )),
        )
        .add_systems(Update, follow_player);
    }
}

fn spawn(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.translation;
        let mut rng = rand::thread_rng();

        let mut position: Vec3;
        loop {
            let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();

            position = Vec3::new(x, y, z) * (EARTH_RADIUS + GHOST_RADIUS);

            if position.distance(player_position) >= MIN_DISTANCE_FROM_PLAYER {
                break;
            }
        }

        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/ghost.glb#Scene0"),
                transform: Transform::from_translation(position).with_scale(Vec3::splat(20.0)),
                ..default()
            },
            Ghost,
        ));
    }
}

fn follow_player(
    mut ghost_query: Query<&mut Transform, With<Ghost>>,
    player_query: Query<&Transform, (With<Player>, Without<Ghost>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut ghost_transform in &mut ghost_query {
            let direction =
                (player_transform.translation - ghost_transform.translation).normalize();

            let ghost_position = ghost_transform.translation
                + direction * GHOST_MOVEMENT_SPEED * time.delta_seconds();

            if ghost_position.distance(Vec3::ZERO) >= EARTH_RADIUS {
                ghost_transform.translation = ghost_position;
            }

            ghost_transform.look_at(player_transform.translation, player_transform.translation)
        }
    }
}
