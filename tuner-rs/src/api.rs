use crate::tuner::Tuner;

#[derive(Debug, Clone)]
pub struct FftComponent {
    pub freq: f32,
    pub intensity: f32,
}

impl Default for FftComponent {
    fn default() -> Self {
        Self {
            freq: 0.0,
            intensity: 0.0,
        }
    }
}

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<FftComponent> {
    let mut tuner = Tuner::new(byte_buffer.len() / 2);
    Ok(tuner.fft(byte_buffer)?)
}
