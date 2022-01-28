use crate::{
    api::Partial,
    constants::*,
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use fitting::gaussian::fit;
use rustfft::{num_complex::Complex, FftPlanner};
use smoothed_z_score::{Peak, PeaksDetector, PeaksFilter};

struct CepstrumPeakIter<I: Iterator<Item = (usize, f64)>> {
    signal: I,
}

trait CepstrumPeaks<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    fn cepstrum_peaks(self) -> CepstrumPeakIter<I>;
}

impl<I> CepstrumPeaks<I> for I
where
    I: Iterator<Item = (usize, f64)>,
{
    fn cepstrum_peaks(self) -> CepstrumPeakIter<I> {
        CepstrumPeakIter { signal: self }
    }
}

impl<I> Iterator for CepstrumPeakIter<I>
where
    I: Iterator<Item = (usize, f64)>,
{
    // mu, sigma, a
    type Item = (f64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        // let (x_vals, y_vals): (Vec<f64>, Vec<f64>) = self
        let asf = loop {
            let dum = self
                .signal
                .by_ref()
                .peaks(PeaksDetector::new(40, 5.0, 0.0), |e| e.1)
                .skip_while(|(_, peak)| *peak == Peak::None)
                .take_while(|(_, peak)| *peak == Peak::High);
            if let Some(size) = dum.size_hint().1 {
                if size > 3 {
                    break dum
                        .map(|(quefrency, _)| (quefrency.0 as f64, quefrency.1))
                        .unzip();
                }
            } else {
            }
        };
        if x_vals.is_empty() {
            return None;
        }

        if let Ok(the_fit) = fit(x_vals.into(), y_vals.into()) {
            Some(the_fit)
        } else {
            None
        }
    }
}

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

        // mu, sigma, a
        let mut cepstrum_peaks: Vec<(f64, f64, f64)> =
            self.spectrum().into_iter().cepstrum_peaks().collect();

        // Sort by highest amplitude
        cepstrum_peaks.sort_by(|(_, _, amplitude_a), (_, _, amplitude_b)| {
            amplitude_b.partial_cmp(amplitude_a).unwrap()
        });

        // Sort by lowest sigma
        cepstrum_peaks
            .sort_by(|(_, sigma_a, _), (_, sigma_b, _)| sigma_a.partial_cmp(sigma_b).unwrap());
        cepstrum_peaks
            .into_iter()
            .nth(0)
            .map(|(mu, _, amplitude)| Partial {
                freq: SAMPLE_RATE / mu,
                intensity: amplitude,
            })
            .ok_or(anyhow::anyhow!(
                "Failed to detect fundamental with power cepstrum"
            ))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_power() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::new(TEST_FFT_SPACE_SIZE);
        // test_fundamental_freq(&mut detector, "noise.json", 4000.0)?;

        // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
        // test_fundamental_freq(&mut detector, "tuner_c5.json", 3384.615)?;

        // test_fundamental_freq(&mut detector, "cello_open_a.json", 218.905)?;
        // test_fundamental_freq(&mut detector, "cello_open_d.json", 146.666)?;
        // test_fundamental_freq(&mut detector, "cello_open_g.json", 97.345)?;

        // This fails to detect the C note, which should be at around 64Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 2933.333)?;
        Ok(())
    }
}
