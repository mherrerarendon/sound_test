use crate::tuner::{tuner_detect_pitch, tuner_init, tuner_set_algorithm};

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

pub fn detect_pitch(byte_buffer: Vec<u8>) -> anyhow::Result<Partial> {
    tuner_detect_pitch(&byte_buffer)
}

pub fn set_algorithm(algorithm: String) -> anyhow::Result<()> {
    tuner_set_algorithm(&algorithm)?;
    Ok(())
}

pub fn init_tuner(algorithm: String, num_samples: u32) -> anyhow::Result<()> {
    tuner_init(&algorithm, num_samples as usize);
    Ok(())
}
