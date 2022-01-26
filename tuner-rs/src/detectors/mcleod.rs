use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, NUM_FUNDAMENTALS, SAMPLE_RATE},
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use num_traits::Zero;
use pitch_detection::detector::{mcleod, PitchDetector};
use rustfft::{num_complex::Complex, FftPlanner};
pub struct McleodDetector {
    mcleod: mcleod::McLeodDetector<f64>,
}

impl FundamentalDetector for McleodDetector {
    fn get_top_fundamentals(&mut self, signal: &[f64]) -> Result<Partial> {
        let pitch = self
            .mcleod
            .get_pitch(&signal, SAMPLE_RATE as usize, 5.0, 0.7)
            .unwrap();
    }
}

impl McleodDetector {
    pub fn new(sample_size: usize) -> Self {
        Self {
            mcleod: mcleod::McLeodDetector::new(sample_size, sample_size / 2),
        }
    }
}
