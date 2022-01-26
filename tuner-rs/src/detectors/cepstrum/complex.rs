use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, SAMPLE_RATE},
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use rustfft::{num_complex::Complex, FftPlanner};

pub struct ComplexCepstrum {
    fft_space: FftSpace,
}

impl FundamentalDetector for ComplexCepstrum {
    fn get_top_fundamentals(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);
        self.fft_space
            .map(|f| Complex::new(f.norm().log(std::f64::consts::E), f.arg()));
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);
        let cepstrum: Vec<f64> = self.fft_space.space().iter().map(|f| f.re).collect();

        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        let mut partials: Vec<Partial> = cepstrum
            .iter()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .enumerate()
            .map(|(quefrency, intensity)| Partial {
                freq: SAMPLE_RATE / (quefrency as f64 + lower_limit as f64) as f64,
                intensity: *intensity,
            })
            .collect();
        partials.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
        partials
            .into_iter()
            .next()
            .ok_or(anyhow::anyhow!("No partials found"))
    }
}

impl ComplexCepstrum {
    pub fn new(fft_space_size: usize) -> Self {
        ComplexCepstrum {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::*, tuner::Tuner};
    use float_cmp::ApproxEq;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn noise() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/noise.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;
        assert!(partial.freq.approx_eq(2000.0, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn tuner_c5() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;

        assert!(partial.freq.approx_eq(523.809, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_a() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;

        assert!(partial.freq.approx_eq(218.905, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_d() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/cello_open_d.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;

        assert!(partial.freq.approx_eq(146.666, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_g() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/cello_open_g.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;
        assert!(partial.freq.approx_eq(97.13, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_c() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../../test_data/cello_open_c.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, COMPLEX_CEPSTRUM_ALGORITHM);
        let partial = tuner.detect_pitch(&buffer)?;

        // This fails to detect the C note, which should be at around 64Hz
        assert!(partial.freq.approx_eq(2588.235, (0.02, 2)));
        Ok(())
    }
}
