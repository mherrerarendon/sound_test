use crate::tuner::{tuner_change_algorithm, tuner_detect_pitch_with_buffer, tuner_init};

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
