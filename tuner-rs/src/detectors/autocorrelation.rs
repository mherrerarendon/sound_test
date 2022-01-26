use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, NUM_FUNDAMENTALS, SAMPLE_RATE},
    detectors::{fft_space::FftSpace, FundamentalDetector, TopFundamentals},
};
use rustfft::FftPlanner;

pub struct AutocorrelationDetector {
    fft_space: FftSpace,
}

impl FundamentalDetector for AutocorrelationDetector {
    fn get_top_fundamentals(&mut self, signal: &[f64]) -> TopFundamentals {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);

        self.fft_space.map(|f| f * f.conj());
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        self.fft_space
            .space()
            .iter()
            .enumerate()
            .map(|(idx, f)| (idx, f.re))
            .skip_while(|(_, intensity)| *intensity > 0.0)
            .skip_while(|(_, intensity)| *intensity < 0.0)
            .take_while(|(_, intensity)| *intensity > 0.0)
            .reduce(|acc, (idx, intensity)| {
                if intensity > acc.1 {
                    (idx, intensity)
                } else {
                    acc
                }
            })
            .map(|(idx, intensity)| {
                TopFundamentals::new(Partial {
                    freq: SAMPLE_RATE / idx as f64,
                    intensity: intensity / self.fft_space.space()[0].re,
                })
            })
            .unwrap()
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
            serde_json::from_str(include_str!("../../test_data/noise.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(119.891, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn tuner_c5() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;

        // Fails to detect C5, which should be at around 523 Hz
        assert!(partials[0].freq.approx_eq(263.473, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_a() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;

        assert!(partials[0].freq.approx_eq(220.0, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_d() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_d.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;

        assert!(fft_peak[0].freq.approx_eq(146.666, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_g() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_g.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;
        assert!(fft_peak[0].freq.approx_eq(97.345, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_c() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_c.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, AUTOCORRELATION_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;

        // Fails to detect an open C on a cello, which should be at around 64 Hz
        assert!(fft_peak[0].freq.approx_eq(141.479, (0.02, 2)));

        Ok(())
    }
}
