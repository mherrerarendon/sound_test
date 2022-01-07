use crate::{api::FftPeak, TunerError};
use rustfft::{num_complex::Complex, FftPlanner};

const SAMPLE_RATE: f32 = 44000.0;

pub struct Tuner {
    fft_planner: FftPlanner<f32>,
    optimized_num_samples: usize,
    scratch: Vec<Complex<f32>>,
}

impl Tuner {
    pub fn new(num_samples: usize) -> Self {
        let optimized_num_samples = Self::optimized_num_samples(num_samples);
        Self {
            fft_planner: FftPlanner::new(),
            optimized_num_samples,
            scratch: vec![Complex::new(0.0, 0.0); optimized_num_samples],
        }
    }

    fn optimized_num_samples(num_samples: usize) -> usize {
        let mut optimized_sum_samples = (2 as usize).pow(14);
        loop {
            if optimized_sum_samples > num_samples {
                optimized_sum_samples /= 2;
            } else {
                break optimized_sum_samples;
            }
        }
    }

    fn ratio(&self) -> f32 {
        SAMPLE_RATE / self.optimized_num_samples as f32
    }

    pub fn fft(&mut self, byte_buffer: Vec<u8>) -> Result<FftPeak, TunerError> {
        if self.optimized_num_samples > byte_buffer.len() / 2 {
            return Err(TunerError::FftFailed);
        }

        let mut samples: Vec<Complex<f32>> = byte_buffer
            .chunks_exact(2)
            .take(self.optimized_num_samples)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]))
            .map(|a| Complex {
                re: a as f32,
                im: 0.0f32,
            })
            .collect();

        let fft = self.fft_planner.plan_fft_forward(samples.len());

        fft.process_with_scratch(&mut samples, &mut self.scratch);
        let absolute_values: Vec<(usize, f32)> = samples
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let sum = a.re.powf(2.0) + a.im.powf(2.0);
                (i, (sum as f32).sqrt())
            })
            .collect();
        let highest_freq_amp = absolute_values
            .iter()
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .ok_or(TunerError::FftFailed)?;

        Ok(FftPeak {
            freq: (highest_freq_amp.0 as f32 * self.ratio()),
            intensity: highest_freq_amp.1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_cmp::ApproxEq;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn fft_works() -> anyhow::Result<()> {
        let mut sample_data: SampleData = serde_json::from_str(include_str!("sampleData.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        assert!(fft_peak.freq.approx_eq(120.849, (0.02, 2)));
        Ok(())
    }
}
