use bevy_reflect::TypeUuid;
use rodio::source;

use crate::Decodable;

/// An audio source that generates an infinite sine wave by exposing an interface to rodios sine wave source
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "95e5b43a-e254-4232-b6e4-fc9db29dd6ce"]
pub struct SineWave {
    freq: f32,
}

impl SineWave {
    /// Create a new SineWave struct
    pub fn new(freq: f32) -> Self {
        SineWave { freq }
    }
}

impl Decodable for SineWave {
    type DecoderItem = f32;
    type Decoder = source::SineWave;   

    fn decoder(&self) -> Self::Decoder {
        source::SineWave::new(self.freq)
    }
}
