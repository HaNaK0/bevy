//! Shows how to use the sine wave 

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut assets: ResMut<Assets<SineWave>>, audio: Res<Audio<SineWave>>) {
    let sine_wave = assets.add(SineWave::new(440.0));

    audio.play(sine_wave);
}