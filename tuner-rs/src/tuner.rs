use crate::{
    api::Partial,
    constants::{CEPSTRUM_ALGORITHM, MARCO_ALGORITHM},
    detectors::{cepstrum, marco_detector, Detector, HarmonicDetector},
    TunerError,
};

pub struct Tuner {
    optimized_num_samples: usize,
    detector: Detector,
}

impl Tuner {
    pub fn new(num_samples: usize, algorithm: &str) -> Self {
        let optimized_num_samples = Self::optimized_num_samples(num_samples);
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

    fn optimized_num_samples(num_samples: usize) -> usize {
        let mut optimized_sum_samples = (2 as usize).pow(14);
        loop {
            if optimized_sum_samples > num_samples {
                optimized_sum_samples /= 2;
            } else {
                break optimized_sum_samples;
            }
        }
    }

    pub fn detect_pitch(&mut self, byte_buffer: Vec<u8>) -> Result<Vec<Partial>, TunerError> {
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
