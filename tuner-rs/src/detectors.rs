pub mod cepstrum;
pub mod marco_detector;

use float_cmp::ApproxEq;
use num_traits::float::FloatCore;

use crate::{api::Partial, constants::NUM_PARTIALS};

use self::{cepstrum::CepstrumDetector, marco_detector::MarcoDetector};

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

// TODO: Assumes sample rate
pub trait HarmonicDetector<T>
where
    T: FloatCore,
{
    fn get_harmonics(&mut self, signal: &[T]) -> Option<HarmonicPitch>;
}

// TODO: use enum dispatch
pub enum Detector {
    Marco(MarcoDetector),
    Cepstrum(CepstrumDetector),
}

impl Detector {
    pub fn new(num_samples: usize) -> Self {
        // Detector::Marco(MarcoDetector::new(num_samples))
        Detector::Cepstrum(CepstrumDetector::new(num_samples))
    }

    pub fn detect(&mut self, signal: &[f64]) -> Option<HarmonicPitch> {
        match *self {
            Self::Marco(ref mut detector) => detector.get_harmonics(signal),
            Self::Cepstrum(ref mut detector) => detector.get_harmonics(signal),
        }
    }
}
