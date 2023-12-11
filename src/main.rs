use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use camera::MainCameraPlugin;
use earth::EarthPlugin;
use enemy::dragon::DragonPlugin;
use enemy::ghost::GhostPlugin;
use enemy::knight::KnightPlugin;
use enemy::EnemyPlugin;
use gameover::GameOverPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use waveform::WaveformPlugin;
use weapon::bullet::BulletPlugin;
use weapon::orbit::OrbitPlugin;
use weapon::WeaponPlugin;

mod camera;
mod earth;
mod enemy;
mod gameover;
mod menu;
mod player;
mod score;
mod waveform;
mod weapon;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
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
        .add_plugins(OrbitPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(GameOverPlugin)
        .add_systems(Update, (menu, game))
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn menu(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

fn game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        game_state.set(GameState::Game);
    }
}

fn despawn_components<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
