pub mod cepstrum;
pub mod marco_detector;

use float_cmp::ApproxEq;
use num_traits::float::FloatCore;

use crate::{api::Partial, constants::NUM_PARTIALS};

use self::{cepstrum::CepstrumDetector, marco_detector::MarcoDetector};
use enum_dispatch::enum_dispatch;

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

#[enum_dispatch]
pub trait HarmonicDetector {
    fn get_harmonics(&mut self, signal: &[f64]) -> Option<HarmonicPitch>;
}

#[enum_dispatch(HarmonicDetector)]
pub enum Detector {
    MarcoDetector,
    CepstrumDetector,
}
