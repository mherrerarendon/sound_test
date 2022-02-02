use crate::{
    api::Partial,
    constants::{
        AUTOCORRELATION_ALGORITHM, COMPLEX_CEPSTRUM_ALGORITHM, POWER_CEPSTRUM_ALGORITHM,
        RAW_FFT_ALGORITHM,
    },
    detectors::{
        autocorrelation,
        cepstrum::{complex, power},
        raw_fft, Detector, FundamentalDetector,
    },
    utils::{audio_buffer_to_signal, calc_optimized_fft_space_size},
    TunerError,
};

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TUNER: Mutex<Option<Tuner>> = Mutex::new(None);
}

pub fn tuner_detect_pitch(byte_buffer: &[u8]) -> Result<Partial> {
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .detect_pitch(byte_buffer)
}

pub fn tuner_set_algorithm(algorithm: &str) -> Result<()> {
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .set_algorithm(algorithm)
}

pub fn tuner_init(algorithm: &str, num_samples: usize) {
    let mut guard = TUNER.lock().unwrap();
    *guard = Some(Tuner::new(num_samples, algorithm));
}

pub struct Tuner {
    optimized_fft_space_size: usize,
    detector: Detector,
}

impl Tuner {
    pub fn new(num_samples: usize, algorithm: &str) -> Self {
        let optimized_fft_space_size = calc_optimized_fft_space_size(num_samples);
        Self {
            optimized_fft_space_size,
            detector: match algorithm {
                COMPLEX_CEPSTRUM_ALGORITHM => Detector::ComplexCepstrum(
                    complex::ComplexCepstrum::new(optimized_fft_space_size),
                ),
                POWER_CEPSTRUM_ALGORITHM => {
                    Detector::PowerCepstrum(power::PowerCepstrum::new(optimized_fft_space_size))
                }
                RAW_FFT_ALGORITHM => {
                    Detector::RawFftDetector(raw_fft::RawFftDetector::new(optimized_fft_space_size))
                }
                AUTOCORRELATION_ALGORITHM => Detector::AutocorrelationDetector(
                    autocorrelation::AutocorrelationDetector::new(optimized_fft_space_size),
                ),
                _ => panic!("Invalid algorithm"),
            },
        }
    }

    pub fn detect_pitch(&mut self, byte_buffer: &[u8]) -> Result<Partial> {
        let signal = audio_buffer_to_signal(byte_buffer);
        self.detector.detect_fundamental(&signal)
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<()> {
        match algorithm {
            RAW_FFT_ALGORITHM => {
                self.detector = Detector::RawFftDetector(raw_fft::RawFftDetector::new(
                    self.optimized_fft_space_size,
                ));
                Ok(())
            }
            COMPLEX_CEPSTRUM_ALGORITHM => {
                self.detector = Detector::ComplexCepstrum(complex::ComplexCepstrum::new(
                    self.optimized_fft_space_size,
                ));
                Ok(())
            }
            POWER_CEPSTRUM_ALGORITHM => {
                self.detector = Detector::PowerCepstrum(power::PowerCepstrum::new(
                    self.optimized_fft_space_size,
                ));
                Ok(())
            }
            AUTOCORRELATION_ALGORITHM => {
                self.detector = Detector::AutocorrelationDetector(
                    autocorrelation::AutocorrelationDetector::new(self.optimized_fft_space_size),
                );
                Ok(())
            }
            _ => bail!(TunerError::UnknownAlgorithm),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::ApproxEq;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn global_tuner() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();

        tuner_init(AUTOCORRELATION_ALGORITHM, buffer.len() / 2);
        let partial = tuner_detect_pitch(&buffer)?;
        assert!(partial.freq.approx_eq(218.543, (0.02, 2)));

        tuner_set_algorithm(COMPLEX_CEPSTRUM_ALGORITHM)?;
        let partial = tuner_detect_pitch(&buffer)?;
        assert!(partial.freq.approx_eq(218.905, (0.02, 2)));

        Ok(())
    }
}
