use crate::tuner::Tuner;

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<i32> {
    let mut tuner = Tuner::new(byte_buffer.len() / 2);
    let fft_peak = tuner.fft(byte_buffer)?;
    Ok(fft_peak.freq.round() as i32)
}
