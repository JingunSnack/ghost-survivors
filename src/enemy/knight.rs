use bevy::prelude::*;
use rand::Rng;

use crate::earth::EARTH_RADIUS;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::waveform::Waveform;
use crate::GameState;

const MIN_DISTANCE_FROM_PLAYER: f32 = 50.0;
const MAX_DISTANCE_FROM_PLAYER: f32 = 100.0;
const SPAWN_INTERVAL: u64 = 10;

const KNIGHT_HEALTH: f32 = 2000.0;
const KNIGHT_MOVEMENT_SPEED: f32 = 30.0;
const KNIGHT_RADIUS: f32 = 50.0;
const KNIGHT_SCORE: u32 = 2000;

#[derive(Component)]
pub struct Knight;

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
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
                scene: asset_server.load("models/knight.glb#Scene0"),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(KNIGHT_RADIUS)),
                ..default()
            },
            Knight,
            Enemy {
                health: KNIGHT_HEALTH,
                speed: KNIGHT_MOVEMENT_SPEED,
                radius: KNIGHT_RADIUS,
                score: KNIGHT_SCORE,
            },
            Waveform { accumulator: 0.0 },
        ));
    }
}
