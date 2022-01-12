use crate::{
    api::Partial,
    constants::{MAX_FREQ, PARTIAL_INTENSITY_SCALING, SAMPLE_RATE},
};
use float_cmp::ApproxEq;

#[derive(Debug, Clone)]
pub struct HarmonicNote {
    pub harmonics: [Partial; 5],
}

impl Default for HarmonicNote {
    fn default() -> Self {
        HarmonicNote {
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

impl HarmonicNote {
    pub fn calc_harmonic_note(absolute_values: &[(usize, f32)]) -> Option<HarmonicNote> {
        let highest_intensity_partials =
            Self::scaled_and_ordered_highest_intensity_partials(absolute_values);
        let mut harmonic_notes = Self::decompose_into_notes(&highest_intensity_partials);
        harmonic_notes.sort_by_key(|note| note.harmonics[0].freq.round() as i32);
        harmonic_notes.sort_by(|a, b| b.num_overtones().cmp(&a.num_overtones()));
        harmonic_notes.into_iter().nth(0)
    }

    fn num_overtones(&self) -> usize {
        self.harmonics
            .iter()
            .skip(1)
            .filter(|p| p.freq > 0.0)
            .count()
    }

    fn new(fundamental: Partial) -> Self {
        let mut note = HarmonicNote::default();
        note.harmonics[0] = fundamental;
        note
    }

    fn is_harmonic(&self) -> bool {
        self.harmonics
            .iter()
            .skip(1)
            .any(|partial| partial.freq.approx_eq(0.0, (0.02, 2)))
    }

    fn scale_partial(partial: &Partial, num_samples: usize) -> Partial {
        let ratio = SAMPLE_RATE / num_samples as f32;
        Partial {
            freq: (partial.freq as f32 * ratio),
            intensity: partial.intensity * PARTIAL_INTENSITY_SCALING,
        }
    }

    fn scaled_and_ordered_highest_intensity_partials(
        absolute_values: &[(usize, f32)],
    ) -> Vec<Partial> {
        let mut highest_intensity_partials: Vec<Partial> = vec![Partial::default(); 30];
        absolute_values.iter().for_each(|a| {
            Self::add_partial_if_high_intensity_and_within_freq_range(
                a,
                &mut highest_intensity_partials,
            );
        });
        highest_intensity_partials = highest_intensity_partials
            .iter()
            .map(|partial| Self::scale_partial(partial, absolute_values.len()))
            .collect();

        highest_intensity_partials.sort_by_key(|partial| partial.freq.round() as i32);
        highest_intensity_partials
    }

    fn decompose_into_notes(ordered_partials: &[Partial]) -> Vec<HarmonicNote> {
        let mut notes: Vec<HarmonicNote> = Vec::new();
        ordered_partials.iter().for_each(|partial| {
            let mut note = HarmonicNote::new(partial.clone());
            (2..note.harmonics.len()).for_each(|harmonic| {
                let overtone = partial.freq * harmonic as f32;
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
        partial: &(usize, f32),
        highest_intensity_partials: &mut Vec<Partial>,
    ) {
        let least_intense_idx = Self::get_index_of_lowest_intensity(&highest_intensity_partials);
        let least_intense_partial = &highest_intensity_partials[least_intense_idx];
        if partial.1 > least_intense_partial.intensity && partial.0 < MAX_FREQ.round() as usize {
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
