use crate::tuner::Tuner;

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

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<Vec<Partial>> {
    let mut tuner = Tuner::new(byte_buffer.len() / 2, "marco");
    Ok(tuner.detect_pitch(byte_buffer)?)
}
