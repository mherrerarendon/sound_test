use flutter_rust_bridge::StreamSink;

use crate::tuner::{
    tuner_change_algorithm, tuner_detect_pitch_with_buffer, tuner_init, tuner_init_stream,
    tuner_new_audio_data,
};

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
    tuner_change_algorithm(&algorithm)
}

pub fn init_tuner(algorithm: String) -> anyhow::Result<()> {
    tuner_init(&algorithm);
    Ok(())
}

pub fn detect_pitch_with_buffer(byte_buffer: Vec<u8>) -> anyhow::Result<Option<Partial>> {
    tuner_detect_pitch_with_buffer(&byte_buffer)
}

pub fn init_stream(sink: StreamSink<Partial>) -> anyhow::Result<()> {
    tuner_init_stream(sink)
}

pub fn new_audio_data(byte_buffer: Vec<u8>) -> anyhow::Result<()> {
    tuner_new_audio_data(&byte_buffer)
}
