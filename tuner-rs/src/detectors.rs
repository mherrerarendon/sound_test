pub mod marco_detector;

use float_cmp::ApproxEq;
use num_traits::float::FloatCore;

use crate::{api::Partial, constants::NUM_PARTIALS};

#[derive(Debug, Clone)]
pub struct HarmonicPitch {
    pub harmonics: [Partial; NUM_PARTIALS],
}

impl Default for HarmonicPitch {
    fn default() -> Self {
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
    fn num_overtones(&self) -> usize {
        self.harmonics
            .iter()
            .skip(1)
            .filter(|p| p.freq > 0.0)
            .count()
    }

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
            .any(|partial| partial.freq.approx_eq(0.0, (0.02, 2)))
    }
}

// Assumes sample rate
pub trait HarmonicDetector<T>
where
    T: FloatCore,
{
    fn get_harmonics(&mut self, signal: &[T]) -> Option<HarmonicPitch>;
}
