use crate::{
    api::Partial,
    constants::*,
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use rustfft::{num_complex::Complex, FftPlanner};

pub struct PowerCepstrum {
    fft_space: FftSpace,
}

impl FundamentalDetector for PowerCepstrum {
    fn detect_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);
        self.fft_space
            .map(|f| Complex::new(f.norm_sqr().log(std::f64::consts::E), 0.0));
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        let cepstrum = self.spectrum();
        let mut partials: Vec<Partial> = cepstrum
            .iter()
            .map(|(quefrency, amplitude)| Partial {
                freq: SAMPLE_RATE / *quefrency as f64,
                intensity: *amplitude,
            })
            .collect();
        // let test = partials.iter().map(|p| p.intensity).collect::<Vec<f64>>();
        partials.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
        partials
            .into_iter()
            .next()
            .ok_or(anyhow::anyhow!("No partials found"))
    }

    fn spectrum(&self) -> Vec<(usize, f64)> {
        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;

        self.fft_space
            .freq_domain(false)
            .map(|(amplitude, _)| amplitude)
            .enumerate()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .collect()
    }

    #[cfg(test)]
    fn name(&self) -> &'static str {
        POWER_CEPSTRUM_ALGORITHM
    }
}

impl PowerCepstrum {
    pub fn new(fft_space_size: usize) -> Self {
        PowerCepstrum {
            fft_space: FftSpace::new(fft_space_size),
        }
    }

    // fn dummy<'a, I: IntoIterator<Item = Complex<f64>>>(&'a self) -> I {
    //     // Frequency = SAMPLE_RATE / quefrency
    //     // With this in mind we can ignore the extremes of the power cepstrum
    //     // https://en.wikipedia.org/wiki/Cepstrum
    //     let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
    //     let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;

    //     self.fft_space.space()
    //     // .freq_domain(false)
    //     // .map(|(freq, _)| freq)
    //     // .map(|f| f.norm_sqr())
    //     // .skip(lower_limit)
    //     // .take(upper_limit - lower_limit)
    //     // .enumerate()
    //     // .map(|(quefrency, intensity)| Partial {
    //     //     freq: SAMPLE_RATE / (quefrency as f64 + lower_limit as f64) as f64,
    //     //     intensity,
    //     // })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_complex() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::new(TEST_FFT_SPACE_SIZE);
        test_fundamental_freq(&mut detector, "noise.json", 4000.0)?;

        // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 3384.615)?;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 218.905)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.666)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.345)?;

        // This fails to detect the C note, which should be at around 64Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 2933.333)?;
        Ok(())
    }
}
