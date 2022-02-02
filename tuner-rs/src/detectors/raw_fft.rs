use crate::{
    api::Partial,
    constants::*,
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use rustfft::FftPlanner;

pub struct RawFftDetector {
    fft_space: FftSpace,
}

impl FundamentalDetector for RawFftDetector {
    fn detect_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(
            signal
                .iter()
                .zip(apodize::hanning_iter(signal.len()))
                .map(|(&x, y)| x * y),
        );

        let (fft_space, scratch) = self.fft_space.workspace();
        fft.process_with_scratch(fft_space, scratch);
        self.spectrum()
            .iter()
            .reduce(|accum, item| if item.1 > accum.1 { item } else { accum })
            .map(|item| Partial {
                freq: item.0 as f64 * SAMPLE_RATE / self.fft_space.len() as f64,
                intensity: item.1,
            })
            .ok_or(anyhow::anyhow!("Failed to detect fundamental with raw fft"))
    }

    fn spectrum(&self) -> Vec<(usize, f64)> {
        let lower_limit = (MIN_FREQ * self.fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        let upper_limit = (MAX_FREQ * self.fft_space.len() as f64 / SAMPLE_RATE).round() as usize;
        self.fft_space
            .freq_domain(true)
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .map(|(i, (amplitude, _))| (i, amplitude))
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        RAW_FFT_ALGORITHM
    }
}

impl RawFftDetector {
    pub fn new(fft_space_size: usize) -> Self {
        RawFftDetector {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_raw_fft() -> anyhow::Result<()> {
        let mut detector = RawFftDetector::new(TEST_FFT_SPACE_SIZE);

        test_fundamental_freq(&mut detector, "tuner_c5.json", 523.681)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 218.872)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.362)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 96.679)?;

        // Fails to detect open C, which should be around 64 Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 128.906)?;
        Ok(())
    }
}
