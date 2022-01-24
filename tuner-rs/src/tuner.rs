use crate::{
    api::Partial,
    constants::{CEPSTRUM_ALGORITHM, MARCO_ALGORITHM},
    detectors::{cepstrum, marco_detector, Detector, FundamentalDetector},
    tuner_filter::TunerFilter,
    TunerError,
};

use lazy_static::lazy_static;
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
    filter: TunerFilter,
}

impl Tuner {
    pub fn new(num_samples: usize, algorithm: &str) -> Self {
        let optimized_fft_space_size = Self::calc_optimized_fft_space_size(num_samples);
        Self {
            optimized_fft_space_size,
            detector: match algorithm {
                CEPSTRUM_ALGORITHM => Detector::from(Detector::CepstrumDetector(
                    cepstrum::CepstrumDetector::new(optimized_fft_space_size),
                )),
                MARCO_ALGORITHM => Detector::from(Detector::MarcoDetector(
                    marco_detector::MarcoDetector::new(optimized_fft_space_size),
                )),
                _ => panic!("Invalid algorithm"),
            },
            filter: TunerFilter::new(),
        }
    }

    fn calc_optimized_fft_space_size(num_samples: usize) -> usize {
        let mut optimized_sum_samples = (2 as usize).pow(10);
        loop {
            if optimized_sum_samples < num_samples * 2 {
                optimized_sum_samples *= 2;
            } else {
                break optimized_sum_samples;
            }
        }
    }

    pub fn detect_pitch(&mut self, byte_buffer: &[u8]) -> Result<Vec<Partial>, TunerError> {
        let signal: Vec<f64> = byte_buffer
            .chunks_exact(2)
            // .take(self.optimized_num_samples)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f64)
            .collect();
        let top_fundamentals = self.detector.get_top_fundamentals(&signal);
        let best_fundamental = match top_fundamentals
            .partials()
            .iter()
            .find(|partial| self.filter.within_range(partial.freq))
        {
            Some(partial) => partial,
            None => top_fundamentals.partials().first().unwrap(),
        };
        self.filter.add_freq(best_fundamental.freq);
        Ok(vec![best_fundamental.clone()])
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<(), TunerError> {
        match algorithm {
            MARCO_ALGORITHM => {
                self.detector = Detector::from(Detector::MarcoDetector(
                    marco_detector::MarcoDetector::new(self.optimized_fft_space_size),
                ));
                Ok(())
            }
            CEPSTRUM_ALGORITHM => {
                self.detector = Detector::from(Detector::CepstrumDetector(
                    cepstrum::CepstrumDetector::new(self.optimized_fft_space_size),
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
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();

        tuner_init(MARCO_ALGORITHM, buffer.len() / 2);
        let partials = tuner_detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(219.543, (0.02, 2)));

        tuner_set_algorithm(CEPSTRUM_ALGORITHM)?;
        let partials = tuner_detect_pitch(&buffer)?;
        assert!(partials[0].freq.approx_eq(218.905, (0.02, 2)));

        Ok(())
    }
}
