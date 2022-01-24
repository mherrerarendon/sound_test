use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, NUM_FUNDAMENTALS, SAMPLE_RATE},
    detectors::{fft_space::FftSpace, FundamentalDetector, TopFundamentals},
};
use num_traits::Zero;
use rustfft::{num_complex::Complex, FftPlanner};

const USE_COMPLEX_CEPSTRUM: bool = true;

pub struct CepstrumDetector {
    fft_space: FftSpace,
}

impl FundamentalDetector for CepstrumDetector {
    fn get_top_fundamentals(&mut self, signal: &[f64]) -> TopFundamentals {
        // assert_eq!(signal.len(), self.scratch.len());
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);
        let cepstrum = match USE_COMPLEX_CEPSTRUM {
            true => self.complex_spectrum(&mut planner),
            false => self.power_spectrum(&mut planner),
        };

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
        partials.into_iter().take(NUM_FUNDAMENTALS).collect()
    }
}

impl CepstrumDetector {
    pub fn new(fft_space_size: usize) -> Self {
        CepstrumDetector {
            fft_space: FftSpace::new(fft_space_size),
        }
    }

    fn complex_spectrum(&mut self, planner: &mut FftPlanner<f64>) -> Vec<f64> {
        self.fft_space = self
            .fft_space
            .freq_domain(true)
            .map(|(freq, phase)| Complex::new(freq.log(std::f64::consts::E), phase))
            .collect();
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);
        self.fft_space.space().iter().map(|f| f.re).collect()
    }

    fn power_spectrum(&mut self, planner: &mut FftPlanner<f64>) -> Vec<f64> {
        self.fft_space = self
            .fft_space
            .freq_domain(false)
            .map(|(freq, _)| Complex::new(freq.log(std::f64::consts::E), 0.0))
            .collect();
        let (fft_space, scratch) = self.fft_space.workspace();
        let forward_fft = planner.plan_fft_forward(fft_space.len());
        forward_fft.process_with_scratch(fft_space, scratch);

        let power_cepstrum: Vec<f64> = self
            .fft_space
            .freq_domain(false)
            .map(|(freq, _)| freq)
            .collect();
        power_cepstrum
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::*, detectors::cepstrum::USE_COMPLEX_CEPSTRUM, tuner::Tuner};
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
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(4000.0, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn tuner_c5() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;

        match USE_COMPLEX_CEPSTRUM {
            true => assert!(partials[0].freq.approx_eq(523.809, (0.02, 2))),

            // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
            false => assert!(partials[0].freq.approx_eq(3384.615, (0.02, 2))),
        }
        Ok(())
    }

    #[test]
    fn cello_open_a() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let partials = tuner.detect_pitch(&buffer)?;

        assert!(partials[0].freq.approx_eq(218.905, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_d() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_d.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;

        assert!(fft_peak[0].freq.approx_eq(146.666, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_g() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_g.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;
        match USE_COMPLEX_CEPSTRUM {
            true => assert!(fft_peak[0].freq.approx_eq(97.13, (0.02, 2))),
            false => assert!(fft_peak[0].freq.approx_eq(97.345, (0.02, 2))),
        }
        Ok(())
    }

    #[test]
    fn cello_open_c() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../../test_data/cello_open_c.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2, CEPSTRUM_ALGORITHM);
        let fft_peak = tuner.detect_pitch(&buffer)?;

        // This fails to detect the C note, which should be at around 64Hz
        match USE_COMPLEX_CEPSTRUM {
            true => assert!(fft_peak[0].freq.approx_eq(2933.333, (0.02, 2))),
            false => assert!(fft_peak[0].freq.approx_eq(3142.857, (0.02, 2))),
        }
        Ok(())
    }
}
