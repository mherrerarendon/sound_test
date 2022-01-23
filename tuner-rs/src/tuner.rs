use crate::{
    api::Partial,
    constants::{CEPSTRUM_ALGORITHM, MARCO_ALGORITHM},
    detectors::{cepstrum, marco_detector, Detector, HarmonicDetector},
    TunerError,
};

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref TUNER: Mutex<Option<Tuner>> = Mutex::new(None);
}

pub fn tuner_detect_pitch(byte_buffer: &[u8]) -> Result<Vec<Partial>, TunerError> {
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .detect_pitch(byte_buffer)
}

pub fn tuner_set_algorithm(algorithm: &str) -> Result<(), TunerError> {
    let mut guard = TUNER.lock().unwrap();
    let num_samples = guard
        .as_ref()
        .ok_or(TunerError::TunerNotInitialized)?
        .optimized_num_samples();

    *guard = Some(Tuner::new(num_samples, algorithm));
    Ok(())
}

pub fn tuner_init(algorithm: &str, num_samples: usize) {
    let mut guard = TUNER.lock().unwrap();
    *guard = Some(Tuner::new(num_samples, algorithm));
}

pub struct Tuner {
    optimized_num_samples: usize,
    detector: Detector,
}

impl Tuner {
    pub fn new(num_samples: usize, algorithm: &str) -> Self {
        let optimized_num_samples = Self::calc_optimized_num_samples(num_samples);
        Self {
            optimized_num_samples,
            detector: match algorithm {
                CEPSTRUM_ALGORITHM => Detector::from(Detector::CepstrumDetector(
                    cepstrum::CepstrumDetector::new(optimized_num_samples),
                )),
                MARCO_ALGORITHM => Detector::from(Detector::MarcoDetector(
                    marco_detector::MarcoDetector::new(optimized_num_samples),
                )),
                _ => panic!("Invalid algorithm"),
            },
        }
    }

    fn optimized_num_samples(&self) -> usize {
        self.optimized_num_samples
    }

    fn calc_optimized_num_samples(num_samples: usize) -> usize {
        let mut optimized_sum_samples = (2 as usize).pow(14);
        loop {
            if optimized_sum_samples > num_samples {
                optimized_sum_samples /= 2;
            } else {
                break optimized_sum_samples;
            }
        }
    }

    pub fn detect_pitch(&mut self, byte_buffer: &[u8]) -> Result<Vec<Partial>, TunerError> {
        if self.optimized_num_samples > byte_buffer.len() / 2 {
            return Err(TunerError::FftFailed);
        }

        let signal: Vec<f64> = byte_buffer
            .chunks_exact(2)
            .take(self.optimized_num_samples)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f64)
            .collect();
        if let Some(harmonics) = self.detector.get_harmonics(&signal) {
            Ok(harmonics.harmonics.iter().cloned().collect())
        } else {
            Err(TunerError::FftFailed)
        }
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<(), TunerError> {
        match algorithm {
            MARCO_ALGORITHM => {
                self.detector = Detector::from(Detector::MarcoDetector(
                    marco_detector::MarcoDetector::new(self.optimized_num_samples),
                ));
                Ok(())
            }
            CEPSTRUM_ALGORITHM => {
                self.detector = Detector::from(Detector::CepstrumDetector(
                    cepstrum::CepstrumDetector::new(self.optimized_num_samples),
                ));
                Ok(())
            }
            _ => Err(TunerError::UnknownAlgorithm),
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
            serde_json::from_str(include_str!("../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();

        tuner_init(MARCO_ALGORITHM, buffer.len() / 2);
        let partials = tuner_detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(523.68, (0.02, 2)));
        assert!(partials[1].freq.approx_eq(1047.36, (0.02, 2)));

        tuner_set_algorithm(CEPSTRUM_ALGORITHM)?;
        let partials = tuner_detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(517.647, (0.02, 2)));

        Ok(())
    }
}
