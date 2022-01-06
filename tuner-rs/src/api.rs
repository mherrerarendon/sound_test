use rustfft::{num_complex::Complex, FftPlanner};

pub fn marco(i: i32) -> anyhow::Result<i32> {
    Ok(i + 1)
}

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<i32> {
    let buffer16: Vec<Complex<f64>> = byte_buffer
        .chunks_exact(2)
        .into_iter()
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .enumerate()
        .map(|(i, a)| Complex {
            re: i / 44000,
            im: a / i16::MAX,
        })
        .collect();

    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Result, Value};

    #[test]
    fn fft_works() {
        let v: Value = serde_json::from_str(include_str!("sampleData.txt"))?;
        let buffer = v["data"].as_array().unwrap();
        let result = fft(buffer);
        assert!(result.is_ok());
    }
}
