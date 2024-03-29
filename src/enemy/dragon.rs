use bevy::prelude::*;
use rand::Rng;

use crate::earth::EARTH_RADIUS;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::waveform::Waveform;
use crate::GameState;

const MIN_DISTANCE_FROM_PLAYER: f32 = 50.0;
const MAX_DISTANCE_FROM_PLAYER: f32 = 100.0;
const SPAWN_INTERVAL: u64 = 30;

const DRAGON_HEALTH: f32 = 5000.0;
const DRAGON_MOVEMENT_SPEED: f32 = 50.0;
const DRAGON_RADIUS: f32 = 50.0;
const DRAGON_SCORE: u32 = 5000;

#[derive(Component)]
pub struct Dragon;

pub struct DragonPlugin;

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn.run_if(in_state(GameState::Game).and_then(
                bevy::time::common_conditions::on_timer(std::time::Duration::from_secs(
                    SPAWN_INTERVAL,
                )),
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

        let mut position: Vec3;
        loop {
            let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();

            position = Vec3::new(x, y, z) * EARTH_RADIUS;

            if MAX_DISTANCE_FROM_PLAYER >= position.distance(player_position)
                && position.distance(player_position) >= MIN_DISTANCE_FROM_PLAYER
            {
                break;
            }
        }

        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/dragon.glb#Scene0"),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(DRAGON_RADIUS)),
                ..default()
            },
            Dragon,
            Enemy {
                health: DRAGON_HEALTH,
                speed: DRAGON_MOVEMENT_SPEED,
                radius: DRAGON_RADIUS,
                score: DRAGON_SCORE,
            },
            Waveform { accumulator: 0.0 },
        ));
    }
}
