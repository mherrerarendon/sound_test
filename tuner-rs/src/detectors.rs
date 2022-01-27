pub mod autocorrelation;
pub mod cepstrum;
mod fft_space;
pub mod marco_detector;
// pub mod mcleod;

use crate::api::Partial;
use anyhow::Result;

use self::{
    autocorrelation::AutocorrelationDetector,
    cepstrum::{complex::ComplexCepstrum, power::PowerCepstrum},
    marco_detector::MarcoDetector,
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FundamentalDetector {
    fn detect_fundamental(&mut self, signal: &[f64]) -> Result<Partial>;

    fn spectrum(&self) -> Vec<(usize, f64)>;

    #[cfg(test)]
    fn name(&self) -> &'static str;
}

#[enum_dispatch(FundamentalDetector)]
pub enum Detector {
    MarcoDetector,
    ComplexCepstrum,
    PowerCepstrum,
    AutocorrelationDetector,
}
