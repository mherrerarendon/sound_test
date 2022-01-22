use crate::{
    api::Partial,
    constants::{MAX_FREQ, PARTIAL_INTENSITY_SCALING, SAMPLE_RATE},
    detectors::{HarmonicDetector, HarmonicPitch},
};
use float_cmp::ApproxEq;
use num_traits::Zero;
use pitch_detection::detector::{internals::Pitch, PitchDetector};
use rustfft::{num_complex::Complex, FftPlanner};

pub struct MarcoDetector {
    samples: Vec<Complex<f64>>,
    scratch: Vec<Complex<f64>>,
}

impl HarmonicDetector for MarcoDetector {
    fn get_harmonics(&mut self, signal: &[f64]) -> Option<HarmonicPitch> {
        assert_eq!(signal.len(), self.scratch.len());
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(signal.len());
        self.samples = signal.iter().map(|x| Complex::new(*x, 0.0)).collect();

        fft.process_with_scratch(&mut self.samples, &mut self.scratch);
        let absolute_values: Vec<(usize, f64)> = self
            .samples
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let sum = a.re.powi(2) + a.im.powi(2);
                (i, sum.sqrt())
            })
            .collect();
        Self::calc_harmonic_note(&absolute_values)
    }
}

impl MarcoDetector {
    pub fn new(num_samples: usize) -> Self {
        MarcoDetector {
            scratch: vec![Complex::zero(); num_samples],
            samples: vec![Complex::zero(); num_samples],
        }
    }

    pub fn calc_harmonic_note(absolute_values: &[(usize, f64)]) -> Option<HarmonicPitch> {
        let highest_intensity_partials =
            Self::scaled_and_ordered_highest_intensity_partials(absolute_values);
        let mut harmonic_notes = Self::decompose_into_notes(&highest_intensity_partials);
        harmonic_notes.sort_by_key(|note| note.harmonics[0].freq.round() as i32);
        harmonic_notes.sort_by(|a, b| b.absolute_intensity().cmp(&a.absolute_intensity()));
        harmonic_notes.into_iter().nth(0)
    }

    fn scale_partial(partial: &Partial, num_samples: usize) -> Partial {
        let ratio = SAMPLE_RATE / num_samples as f64;
        Partial {
            freq: (partial.freq as f64 * ratio),
            intensity: partial.intensity * PARTIAL_INTENSITY_SCALING,
        }
    }

    fn scaled_and_ordered_highest_intensity_partials(
        absolute_values: &[(usize, f64)],
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
        partial: &(usize, f64),
        highest_intensity_partials: &mut Vec<Partial>,
    ) {
        let least_intense_idx = Self::get_index_of_lowest_intensity(&highest_intensity_partials);
        let least_intense_partial = &highest_intensity_partials[least_intense_idx];
        if partial.1 > least_intense_partial.intensity && partial.0 < MAX_FREQ.round() as usize {
            highest_intensity_partials[least_intense_idx] = Partial {
                freq: partial.0 as f64,
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
