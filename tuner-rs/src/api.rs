use anyhow::anyhow;
use rustfft::{num_complex::Complex, FftPlanner};

pub fn marco(i: i32) -> anyhow::Result<i32> {
    Ok(i + 1)
}

pub fn fft(byte_buffer: Vec<u8>) -> anyhow::Result<i32> {
    let mut num_samples = 16384;
    let num_samples_take = loop {
        if num_samples > byte_buffer.len() / 2 {
            num_samples /= 2;
        } else {
            break num_samples;
        }
    };
    let sample_rate = 44000.0f32;
    let ratio = sample_rate / num_samples_take as f32;
    let mut buffer16: Vec<Complex<f32>> = byte_buffer
        .chunks_exact(2)
        .take(num_samples_take)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .map(|a| Complex {
            re: a as f32,
            im: 0.0f32,
        })
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer16.len());

    fft.process(&mut buffer16);
    let freq_domain: Vec<(usize, f32)> = buffer16
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let sum = a.re.powf(2.0) + a.im.powf(2.0);
            (i, (sum as f32).sqrt())
        })
        .collect();
    let highest_freq_amp = freq_domain
        .iter()
        .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
        .ok_or(anyhow!("fft failed"))?;

    Ok((highest_freq_amp.0 as f32 * ratio).round() as i32)
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
