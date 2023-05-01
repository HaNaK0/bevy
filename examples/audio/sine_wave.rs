//! Shows how to use the sine wave 

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut assets: ResMut<Assets<SineWave>>, audio: Res<Audio<SineWave>>) {
    // Create a SineWave that play a C
    let sine_wave = assets.add(SineWave::new(440.0));

    // Start playing the SineWave
    audio.play(sine_wave);
}