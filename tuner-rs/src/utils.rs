pub fn audio_buffer_to_signal(byte_buffer: &[u8]) -> Vec<f64> {
    byte_buffer
        .chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f64)
        .collect()
}

pub fn calc_optimized_fft_space_size(num_samples: usize) -> usize {
    let mut optimized_sum_samples = (2usize).pow(10);
    loop {
        if optimized_sum_samples < num_samples {
            optimized_sum_samples *= 2;
        } else {
            break optimized_sum_samples;
        }
    }
}
