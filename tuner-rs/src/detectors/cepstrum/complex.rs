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
    fn get_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
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
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_complex() -> anyhow::Result<()> {
        let mut detector = ComplexCepstrum::new(TEST_FFT_SPACE_SIZE);
        test_fundamental_freq(&mut detector, "noise.json", 2000.0)?;
        test_fundamental_freq(&mut detector, "tuner_c5.json", 523.809)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 218.905)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.666)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.13)?;

        // This fails to detect the C note, which should be at around 64Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 2588.235)?;
        Ok(())
    }
}
