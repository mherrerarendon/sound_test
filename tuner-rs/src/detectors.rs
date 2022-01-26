pub mod autocorrelation;
pub mod cepstrum;
mod fft_space;
pub mod marco_detector;
// pub mod mcleod;

use crate::{api::Partial, constants::NUM_FUNDAMENTALS};

use self::{
    autocorrelation::AutocorrelationDetector, cepstrum::CepstrumDetector,
    marco_detector::MarcoDetector,
};
use enum_dispatch::enum_dispatch;

pub struct TopFundamentals {
    partials: [Partial; NUM_FUNDAMENTALS],
}

impl TopFundamentals {
    pub fn partials(&self) -> &[Partial] {
        &self.partials
    }

    pub fn new(partial: Partial) -> Self {
        let mut top_fundamentals = Self::default();
        top_fundamentals.partials[0] = partial;
        top_fundamentals
    }
}

impl Default for TopFundamentals {
    fn default() -> Self {
        assert_eq!(NUM_FUNDAMENTALS, 5);
        TopFundamentals {
            partials: [
                Partial::default(),
                Partial::default(),
                Partial::default(),
                Partial::default(),
                Partial::default(),
            ],
        }
    }
}

impl FromIterator<Partial> for TopFundamentals {
    fn from_iter<I: IntoIterator<Item = Partial>>(iter: I) -> Self {
        let mut top_fundamentals = TopFundamentals::default();

        for (idx, partial) in iter.into_iter().take(NUM_FUNDAMENTALS).enumerate() {
            top_fundamentals.partials[idx] = partial;
        }

        top_fundamentals
    }
}

#[enum_dispatch]
pub trait FundamentalDetector {
    fn get_top_fundamentals(&mut self, signal: &[f64]) -> TopFundamentals;
}

#[enum_dispatch(FundamentalDetector)]
pub enum Detector {
    MarcoDetector,
    CepstrumDetector,
    AutocorrelationDetector,
}
