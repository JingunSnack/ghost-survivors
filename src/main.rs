use bevy::prelude::*;

use camera::MainCameraPlugin;
use earth::EarthPlugin;
use enemy::dragon::DragonPlugin;
use enemy::ghost::GhostPlugin;
use enemy::knight::KnightPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use waveform::WaveformPlugin;
use weapon::bullet::BulletPlugin;
use weapon::WeaponPlugin;

mod camera;
mod earth;
mod enemy;
mod player;
mod waveform;
mod weapon;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(MainCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EarthPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(KnightPlugin)
        .add_plugins(DragonPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(WaveformPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}
