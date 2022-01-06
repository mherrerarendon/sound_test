use rustfft::{num_complex::Complex, FftPlanner};

pub fn marco(i: i32) -> anyhow::Result<i32> {
    Ok(i + 1)
}

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<i32> {
    let buffer16: Vec<Complex<i32>> = byte_buffer
        .chunks_exact(2)
        .into_iter()
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .enumerate()
        .map(|(i, a)| Complex {
            re: i as i32,
            im: a as i32,
        })
        .collect();

    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_json::{Result, Value};

    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn fft_works() -> anyhow::Result<()> {
        let mut sample_data: SampleData = serde_json::from_str(include_str!("sampleData.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let result = fft(buffer);
        assert!(result.is_ok());
        Ok(())
    }
}
