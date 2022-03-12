use crate::{
    api::Pitch,
    constants::*,
    frequency_detector::{autocorrelation, cepstrum, raw_fft, Detector, FrequencyDetector},
    utils::{audio_buffer_to_samples, calc_optimized_fft_space_size},
    TunerError,
};

use anyhow::{bail, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use num_traits::signum;
use std::sync::Mutex;

lazy_static! {
    static ref TUNER: Mutex<Option<Tuner>> = Mutex::new(None);
}

pub fn tuner_change_algorithm(algorithm: &str) -> Result<()> {
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .set_algorithm(algorithm)
}

pub fn tuner_init(algorithm: &str) {
    let mut guard = TUNER.lock().unwrap();
    *guard = Some(Tuner::new(algorithm));
}

pub fn tuner_detect_pitch_with_buffer(byte_buffer: &[u8]) -> Result<Option<Pitch>> {
    if byte_buffer.len() % 2 != 0 {
        bail!("Audio buffer size must be a multiple of 2");
    }
    Ok(TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .detect_pitch_with_buffer(byte_buffer))
}

pub struct Tuner {
    detector: Detector,
    frame_buffer: [i16; FRAME_BUFFER_SIZE],
    remaining_frame_capacity: usize,
}

impl Tuner {
    pub fn new(algorithm: &str) -> Self {
        let optimized_fft_space_size = calc_optimized_fft_space_size(FRAME_BUFFER_SIZE);
        Self {
            detector: match algorithm {
                POWER_CEPSTRUM_ALGORITHM => {
                    Detector::PowerCepstrum(cepstrum::PowerCepstrum::new(optimized_fft_space_size))
                }
                RAW_FFT_ALGORITHM => {
                    Detector::RawFftDetector(raw_fft::RawFftDetector::new(optimized_fft_space_size))
                }
                AUTOCORRELATION_ALGORITHM => Detector::AutocorrelationDetector(
                    autocorrelation::AutocorrelationDetector::new(optimized_fft_space_size),
                ),
                _ => panic!("Invalid algorithm"),
            },
            frame_buffer: [0; FRAME_BUFFER_SIZE],
            remaining_frame_capacity: FRAME_BUFFER_SIZE,
        }
    }

    fn process_audio_data(&mut self) -> Option<Pitch> {
        let mut partial = None;
        if let Some(detected_partial) = self.detect_pitch() {
            partial = Some(detected_partial.clone());
        }

        self.reset_frame_buffer();
        partial
    }

    pub fn detect_pitch_with_buffer(&mut self, byte_buffer: &[u8]) -> Option<Pitch> {
        if self.remaining_frame_capacity != FRAME_BUFFER_SIZE {
            self.reset_frame_buffer();
        }
        let mut samples = audio_buffer_to_samples(byte_buffer);
        if samples.len() > FRAME_BUFFER_SIZE {
            samples = samples[0..FRAME_BUFFER_SIZE].to_vec();
        }
        self.copy_samples_into_frame_buffer(&samples);
        self.process_audio_data()
    }

    fn copy_samples_into_frame_buffer(&mut self, samples: &[i16]) {
        assert!(samples.len() <= FRAME_BUFFER_SIZE);
        let append_idx = FRAME_BUFFER_SIZE - self.remaining_frame_capacity;
        let append_end_idx = append_idx + samples.len();
        self.frame_buffer[append_idx..append_end_idx].copy_from_slice(&samples);
        self.remaining_frame_capacity -= samples.len();
    }

    fn reset_frame_buffer(&mut self) {
        self.frame_buffer.iter_mut().for_each(|sample| *sample = 0);
        self.remaining_frame_capacity = FRAME_BUFFER_SIZE;
    }

    fn zero_crossing_rate(&self) -> usize {
        self.frame_buffer
            .iter()
            .take(FRAME_BUFFER_SIZE - self.remaining_frame_capacity)
            .tuple_windows()
            .map(|(a, b)| (signum(*a) - signum(*b)).abs() as usize)
            .sum::<usize>()
            / 2
    }

    pub fn detect_pitch(&mut self) -> Option<Pitch> {
        // ZERO_CROSSING_RATE_THRESHOLD is not totally accurate yet.
        // if self.zero_crossing_rate() > ZERO_CROSSING_RATE_THRESHOLD {
        //     return None;
        // }
        let iter = self
            .frame_buffer
            .iter()
            .take(FRAME_BUFFER_SIZE - self.remaining_frame_capacity)
            .map(|sample| *sample as f64);
        self.detector.detect_frequency(iter).map(|f| f.into())
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> Result<()> {
        let optimized_fft_space_size = calc_optimized_fft_space_size(FRAME_BUFFER_SIZE);
        match algorithm {
            RAW_FFT_ALGORITHM => {
                self.detector = Detector::RawFftDetector(raw_fft::RawFftDetector::new(
                    optimized_fft_space_size,
                ));
                Ok(())
            }
            POWER_CEPSTRUM_ALGORITHM => {
                self.detector =
                    Detector::PowerCepstrum(cepstrum::PowerCepstrum::new(optimized_fft_space_size));
                Ok(())
            }
            AUTOCORRELATION_ALGORITHM => {
                self.detector = Detector::AutocorrelationDetector(
                    autocorrelation::AutocorrelationDetector::new(optimized_fft_space_size),
                );
                Ok(())
            }
            _ => bail!(TunerError::UnknownAlgorithm),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use float_cmp::ApproxEq;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    fn test_zero_rate_crossing(
        tuner: &mut Tuner,
        filename: &str,
        expected_rate: usize,
    ) -> anyhow::Result<()> {
        let file_path = format!("{}/test_data/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let mut sample_data: SampleData = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        let buffer = sample_data.data.take().unwrap();
        let samples = audio_buffer_to_samples(&buffer);
        tuner.reset_frame_buffer();
        tuner.copy_samples_into_frame_buffer(&samples);
        let zero_crossings = tuner.zero_crossing_rate();
        assert_eq!(zero_crossings, expected_rate);
        Ok(())
    }

    #[test]
    fn zero_crossings() -> anyhow::Result<()> {
        let mut tuner = Tuner::new(AUTOCORRELATION_ALGORITHM);

        test_zero_rate_crossing(&mut tuner, "cello_open_a.json", 258)?;
        test_zero_rate_crossing(&mut tuner, "cello_open_d.json", 262)?;
        test_zero_rate_crossing(&mut tuner, "cello_open_g.json", 162)?;
        test_zero_rate_crossing(&mut tuner, "cello_open_c.json", 158)?;
        test_zero_rate_crossing(&mut tuner, "noise.json", 102)?;

        Ok(())
    }

    #[test]
    fn detect_with_buffer() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(AUTOCORRELATION_ALGORITHM);
        let pitch = tuner
            .detect_pitch_with_buffer(&buffer)
            .expect("failed to detect pitch");
        assert_eq!(pitch.note_name, "A");

        tuner.set_algorithm(POWER_CEPSTRUM_ALGORITHM)?;
        let partial = tuner
            .detect_pitch_with_buffer(&buffer)
            .expect("failed to detect pitch");
        assert_eq!(pitch.note_name, "A");
        Ok(())
    }
}
