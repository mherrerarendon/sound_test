use crate::{api::Partial, harmonic_partials::HarmonicPartials, TunerError};
use rustfft::{num_complex::Complex, FftPlanner};

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

    pub fn fft(&mut self, byte_buffer: Vec<u8>) -> Result<Vec<Partial>, TunerError> {
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
        let harmonics = HarmonicPartials::new(30, &absolute_values);

        Ok(harmonics.harmonic_partials().iter().cloned().collect())
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
    fn noise() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/noise.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.fft(buffer)?;
        assert!(partials[0].freq.approx_eq(40.28, (0.02, 2)));
        assert!(partials[1].freq.approx_eq(120.849, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn tuner_c5() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.fft(buffer)?;
        // assert!(partials[0].freq.approx_eq(523.68, (0.02, 2)));
        // assert_eq!(partials.len(), 1);
        Ok(())
    }

    #[test]
    fn cello_open_a() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.fft(buffer)?;
        assert!(partials[0].freq.approx_eq(220.21, (0.02, 2)));
        assert!(partials[1].freq.approx_eq(440.43, (0.02, 2)));
        assert!(partials[2].freq.approx_eq(880.86, (0.02, 2)));
        assert_eq!(partials.len(), 3);
        Ok(())
    }

    #[test]
    fn cello_open_d() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_d.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        // assert!(fft_peak[0].freq.approx_eq(146.83, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_g() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_g.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        assert!(fft_peak[0].freq.approx_eq(96.68, (0.02, 2)));
        assert!(fft_peak[1].freq.approx_eq(193.36, (0.02, 2)));
        assert!(fft_peak[2].freq.approx_eq(290.04, (0.02, 2)));
        assert!(fft_peak[3].freq.approx_eq(386.72, (0.02, 2)));
        assert_eq!(fft_peak.len(), 4);
        Ok(())
    }

    #[test]
    fn cello_open_c() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_c.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        assert!(fft_peak[0].freq.approx_eq(64.45, (0.02, 2)));
        assert!(fft_peak[1].freq.approx_eq(128.91, (0.02, 2)));
        assert!(fft_peak[2].freq.approx_eq(257.81, (0.02, 2)));
        assert_eq!(fft_peak.len(), 8);
        Ok(())
    }

    #[test]
    fn cello_open_c_out_of_tune() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_c_out_of_tune.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        // assert!(fft_peak[0].freq.approx_eq(65.41, (0.02, 2)));
        Ok(())
    }
}
