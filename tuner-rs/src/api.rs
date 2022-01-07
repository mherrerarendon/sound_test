use crate::tuner::Tuner;

#[derive(Debug, Clone)]
pub struct FftPeak {
    pub freq: f32,
    pub intensity: f32,
}

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<FftPeak> {
    let mut tuner = Tuner::new(byte_buffer.len() / 2);
    Ok(tuner.fft(byte_buffer)?)
    // Ok(fft_peak.freq.round() as i32)
}
