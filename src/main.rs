use bevy::prelude::*;

use bullet::BulletPlugin;
use camera::MainCameraPlugin;
use earth::EarthPlugin;
use enemy::dragon::DragonPlugin;
use enemy::ghost::GhostPlugin;
use enemy::knight::KnightPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod bullet;
mod camera;
mod earth;
mod enemy;
mod player;

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
        .add_plugins(BulletPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
