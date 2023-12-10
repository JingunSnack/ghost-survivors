use bevy::prelude::*;
use rand::Rng;

use crate::earth::EARTH_RADIUS;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::waveform::Waveform;

const MIN_DISTANCE_FROM_PLAYER: f32 = 50.0;
const MAX_DISTANCE_FROM_PLAYER: f32 = 100.0;
const SPAWN_INTERVAL: u64 = 1;

const GHOST_HEALTH: f32 = 100.0;
const GHOST_MOVEMENT_SPEED: f32 = 20.0;
const GHOST_RADIUS: f32 = 20.0;

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
        );
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

        for _ in 0..5 {
            let mut position: Vec3;
            loop {
                let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let phi = rng.gen_range(0.0..std::f32::consts::PI);
                let x = phi.sin() * theta.cos();
                let y = phi.sin() * theta.sin();
                let z = phi.cos();

                position = Vec3::new(x, y, z) * (EARTH_RADIUS + GHOST_RADIUS);

                if MAX_DISTANCE_FROM_PLAYER >= position.distance(player_position)
                    && position.distance(player_position) >= MIN_DISTANCE_FROM_PLAYER
                {
                    break;
                }
            }

            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/ghost.glb#Scene0"),
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(GHOST_RADIUS)),
                    ..default()
                },
                Ghost,
                Enemy {
                    health: GHOST_HEALTH,
                    speed: GHOST_MOVEMENT_SPEED,
                    radius: GHOST_RADIUS,
                },
                Waveform { accumulator: 0.0 },
            ));
        }
    }
}
