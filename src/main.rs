use bevy::prelude::*;

mod bullet;
mod camera;
mod earth;
mod ghost;
mod player;

use crate::bullet::BulletPlugin;
use crate::camera::MainCameraPlugin;
use crate::earth::EarthPlugin;
use crate::ghost::GhostPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(MainCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EarthPlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(BulletPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
