pub fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f64)
        .collect()
}
