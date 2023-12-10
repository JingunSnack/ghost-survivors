use bevy::prelude::*;

#[derive(Component)]
pub struct Waveform {
    pub accumulator: f32,
}

pub const WAVEFORM_AMPLITUDE: f32 = 10.0;
pub const WAVEFORM_SPEED: f32 = 3.0;

pub struct WaveformPlugin;

impl Plugin for WaveformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, waveform);
    }
}

fn waveform(
    mut waveform_query: Query<(&mut Transform, &mut Waveform), With<Waveform>>,
    time: Res<Time>,
) {
    for (mut transform, mut waveform) in &mut waveform_query {
        waveform.accumulator += time.delta_seconds();
        let floating_value = WAVEFORM_AMPLITUDE * (waveform.accumulator * WAVEFORM_SPEED).sin();
        transform.translation.y += floating_value * time.delta_seconds();
    }
}
