use crate::{
    api::Partial,
    constants::*,
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use fitting::gaussian::fit;
use rustfft::FftPlanner;

pub struct AutocorrelationDetector {
    fft_space: FftSpace,
}

impl FundamentalDetector for AutocorrelationDetector {
    fn detect_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);

        self.fft_space.map(|f| f * f.conj());
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        let peak: Vec<(usize, f64)> = self
            .spectrum()
            .into_iter()
            .skip_while(|(_, intensity)| *intensity > 0.0)
            .skip_while(|(_, intensity)| *intensity < 0.0)
            .take_while(|(_, intensity)| *intensity > 0.0)
            .collect();
        let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = peak
            .iter()
            .map(|i| (i.0 as f64, i.1 / self.fft_space.space()[0].re))
            .unzip();

        // mu, sigma, a
        let (mu, _, a) = fit(x_vals.into(), y_vals.into())?;

        Ok(Partial {
            freq: SAMPLE_RATE / mu,
            intensity: a,
        })
    }

    fn spectrum(&self) -> Vec<(usize, f64)> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        self.fft_space
            .space()
            .iter()
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .map(|(idx, f)| (idx, f.re))
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        AUTOCORRELATION_ALGORITHM
    }
}

impl AutocorrelationDetector {
    pub fn new(fft_space_size: usize) -> Self {
        AutocorrelationDetector {
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
        let mut detector = AutocorrelationDetector::new(TEST_FFT_SPACE_SIZE);
        test_fundamental_freq(&mut detector, "noise.json", 119.997)?;

        // Fails to detect C5, which whould be at around 523 Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 263.919)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 219.634)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.717)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.985)?;

        // This fails to detect the C note, which should be at around 64Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 129.536)?;
        Ok(())
    }
}
