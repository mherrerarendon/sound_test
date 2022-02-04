use flutter_rust_bridge::StreamSink;

use crate::tuner::{tuner_change_algorithm, tuner_init, tuner_new_audio_data};

#[derive(Debug, Clone)]
pub struct Partial {
    pub freq: f64,
    pub intensity: f64,
}

impl Default for Partial {
    fn default() -> Self {
        Self {
            freq: 0.0,
            intensity: 0.0,
        }
    }
}

pub fn change_algorithm(algorithm: String) -> anyhow::Result<()> {
    tuner_change_algorithm(&algorithm)?;
    Ok(())
}

pub fn init_tuner(sink: StreamSink<Partial>, algorithm: String) -> anyhow::Result<()> {
    tuner_init(sink, &algorithm);
    Ok(())
}

pub fn new_audio_data(byte_buffer: Vec<u8>) -> anyhow::Result<()> {
    tuner_new_audio_data(&byte_buffer)
}
