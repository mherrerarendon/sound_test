use crate::{
    api::Partial,
    constants::*,
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::{anyhow, Result};
use float_cmp::ApproxEq;
use rustfft::FftPlanner;

pub const NUM_PARTIALS: usize = 5;

#[derive(Debug, Clone)]
pub struct HarmonicPitch {
    pub harmonics: [Partial; NUM_PARTIALS],
}

impl Default for HarmonicPitch {
    fn default() -> Self {
        assert_eq!(NUM_PARTIALS, 5);
        HarmonicPitch {
            harmonics: [
                Partial::default(),
                Partial::default(),
                Partial::default(),
                Partial::default(),
                Partial::default(),
            ],
        }
    }
}

impl HarmonicPitch {
    fn absolute_intensity(&self) -> i64 {
        self.harmonics
            .iter()
            .fold(0i64, |accum, item| accum + item.intensity.round() as i64)
    }

    fn new(fundamental: Partial) -> Self {
        let mut note = HarmonicPitch::default();
        note.harmonics[0] = fundamental;
        note
    }

    fn is_harmonic(&self) -> bool {
        self.harmonics
            .iter()
            .skip(1)
            .any(|partial| !partial.freq.approx_eq(0.0, (0.02, 2)))
    }
}
pub struct MarcoDetector {
    fft_space: FftSpace,
}

impl FundamentalDetector for MarcoDetector {
    fn detect_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        fft.process_with_scratch(fft_space, scratch);
        let absolute_values: Vec<(usize, f64)> = self.spectrum();
        self.calc_top_fundamentals(&absolute_values)
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
        MARCO_ALGORITHM
    }
}

impl MarcoDetector {
    pub fn new(fft_space_size: usize) -> Self {
        MarcoDetector {
            fft_space: FftSpace::new(fft_space_size),
        }
    }

    fn calc_top_fundamentals(&self, absolute_values: &[(usize, f64)]) -> Result<Partial> {
        let highest_intensity_partials =
            self.scaled_and_ordered_highest_intensity_partials(absolute_values);
        let mut harmonic_notes = Self::decompose_into_notes(&highest_intensity_partials);
        if harmonic_notes.is_empty() {
            // Didn't find a harmonic pitch, so just return the highest intensity partial
            harmonic_notes.push(
                highest_intensity_partials
                    .iter()
                    .reduce(|accum, item| {
                        if item.intensity > accum.intensity {
                            item
                        } else {
                            accum
                        }
                    })
                    .map(|partial| HarmonicPitch::new(partial.clone()))
                    .ok_or(anyhow!("No highest intensity partial found"))?,
            );
        }
        harmonic_notes.sort_by(|a, b| {
            a.harmonics[0]
                .freq
                .partial_cmp(&b.harmonics[0].freq)
                .unwrap()
        });
        harmonic_notes.sort_by(|a, b| {
            b.absolute_intensity()
                .partial_cmp(&a.absolute_intensity())
                .unwrap()
        });
        harmonic_notes
            .into_iter()
            .next()
            .map(|note| Partial {
                freq: note.harmonics[0].freq,
                intensity: note.absolute_intensity() as f64,
            })
            .ok_or(anyhow!("No harmonic notes found"))
    }

    fn scale_partial(&self, partial: &Partial) -> Partial {
        let ratio = SAMPLE_RATE / self.fft_space.len() as f64;
        Partial {
            freq: ((partial.freq /*- 1.0*/ as f64) * ratio), // https://wiki.analytica.com/FFT substracts 1?
            intensity: partial.intensity * PARTIAL_INTENSITY_SCALING,
        }
    }

    fn scaled_and_ordered_highest_intensity_partials(
        &self,
        absolute_values: &[(usize, f64)],
    ) -> Vec<Partial> {
        let mut highest_intensity_partials: Vec<Partial> = vec![Partial::default(); 30];
        absolute_values.iter().for_each(|a| {
            Self::add_partial_if_high_intensity_and_within_freq_range(
                Partial {
                    freq: a.0 as f64,
                    intensity: a.1,
                },
                &mut highest_intensity_partials,
            );
        });
        highest_intensity_partials = highest_intensity_partials
            .iter()
            .map(|partial| self.scale_partial(partial))
            .collect();

        highest_intensity_partials.sort_by_key(|partial| partial.freq.round() as i32);
        highest_intensity_partials
    }

    fn decompose_into_notes(ordered_partials: &[Partial]) -> Vec<HarmonicPitch> {
        let mut notes: Vec<HarmonicPitch> = Vec::new();
        ordered_partials.iter().for_each(|partial| {
            let mut note = HarmonicPitch::new(partial.clone());
            (2..note.harmonics.len()).for_each(|harmonic| {
                let overtone = partial.freq * harmonic as f64;
                if let Some(partial) = ordered_partials
                    .iter()
                    .find(|partial| partial.freq.approx_eq(overtone, (0.02, 2)))
                {
                    note.harmonics[harmonic - 1] = partial.clone();
                }
            });
            if note.is_harmonic() {
                notes.push(note);
            }
        });
        notes
    }

    fn add_partial_if_high_intensity_and_within_freq_range(
        partial: Partial,
        highest_intensity_partials: &mut Vec<Partial>,
    ) {
        let least_intense_idx =
            Self::get_index_of_lowest_intensity(highest_intensity_partials).unwrap();
        let least_intense_partial = &highest_intensity_partials[least_intense_idx];
        if partial.intensity > least_intense_partial.intensity && partial.freq < MAX_FREQ {
            highest_intensity_partials[least_intense_idx] = partial;
        }
    }

    fn get_index_of_lowest_intensity(partials: &[Partial]) -> Option<usize> {
        partials
            .iter()
            .enumerate()
            .reduce(|accum, item| {
                if item.1.intensity < accum.1.intensity {
                    item
                } else {
                    accum
                }
            })
            .map(|(idx, _)| idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_complex() -> anyhow::Result<()> {
        let mut detector = MarcoDetector::new(TEST_FFT_SPACE_SIZE);
        test_fundamental_freq(&mut detector, "noise.json", 60.424)?;

        // Fails to detect c5, which should be at around 523 Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 38.940)?;
        test_fundamental_freq(&mut detector, "cello_open_a.json", 218.872)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.362)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 96.679)?;
        test_fundamental_freq(&mut detector, "cello_open_c.json", 64.45)?;
        Ok(())
    }
}
