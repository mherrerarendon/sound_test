use crate::{
    api::Partial,
    constants::{PARTIAL_INTENSITY_SCALING, SAMPLE_RATE},
};

pub struct HarmonicPartials {
    harmonic_partials: Vec<Partial>,
}

impl HarmonicPartials {
    pub fn new(max_partials: usize, absolute_values: &[(usize, f32)]) -> Self {
        Self {
            harmonic_partials: Self::calc_harmonic_partials(max_partials, absolute_values),
        }
    }

    pub fn harmonic_partials(&self) -> &[Partial] {
        &self.harmonic_partials
    }

    fn scale_partial(partial: &Partial, num_samples: usize) -> Partial {
        let ratio = SAMPLE_RATE / num_samples as f32;
        Partial {
            freq: (partial.freq as f32 * ratio),
            intensity: partial.intensity * PARTIAL_INTENSITY_SCALING,
        }
    }

    fn scaled_highest_intensity_partials(
        num: usize,
        absolute_values: &[(usize, f32)],
    ) -> Vec<Partial> {
        let mut highest_intensity_partials: Vec<Partial> = vec![Partial::default(); num];
        absolute_values.iter().for_each(|a| {
            Self::add_partial_if_high_intensity(a, &mut highest_intensity_partials);
        });
        highest_intensity_partials
            .iter()
            .map(|partial| Self::scale_partial(partial, absolute_values.len()))
            .collect()
    }

    fn calc_harmonic_partials(num: usize, absolute_values: &[(usize, f32)]) -> Vec<Partial> {
        let highest_intensity_partials =
            Self::scaled_highest_intensity_partials(num, absolute_values);
        let highest_intensity_partial = highest_intensity_partials
            .iter()
            .reduce(|accum, item| {
                if item.intensity > accum.intensity {
                    item
                } else {
                    accum
                }
            })
            .unwrap();
        let mut harmonics: Vec<Partial> = highest_intensity_partials
            .iter()
            .filter(|partial| Self::harmonic(partial, highest_intensity_partial).is_some())
            .map(|partial| partial.clone())
            .collect();
        harmonics.sort_by_key(|partial| partial.freq.round() as usize);
        harmonics
    }

    fn harmonic(partial: &Partial, harmonic: &Partial) -> Option<f32> {
        if (harmonic.freq.round() as usize) % (partial.freq.round() as usize) == 0 {
            Some(harmonic.freq / partial.freq)
        } else if (partial.freq.round() as usize) % (harmonic.freq.round() as usize) == 0 {
            Some(partial.freq / harmonic.freq)
        } else {
            None
        }
    }

    fn add_partial_if_high_intensity(
        partial: &(usize, f32),
        highest_intensity_partials: &mut Vec<Partial>,
    ) {
        let least_intense_idx = Self::get_index_of_lowest_intensity(&highest_intensity_partials);
        let least_intense_partial = &highest_intensity_partials[least_intense_idx];
        if partial.1 > least_intense_partial.intensity {
            highest_intensity_partials[least_intense_idx] = Partial {
                freq: partial.0 as f32,
                intensity: partial.1,
            };
        }
    }

    fn get_index_of_lowest_intensity(partials: &[Partial]) -> usize {
        partials
            .into_iter()
            .enumerate()
            .reduce(|accum, item| {
                if item.1.intensity < accum.1.intensity {
                    item
                } else {
                    accum
                }
            })
            .unwrap()
            .0
    }
}
