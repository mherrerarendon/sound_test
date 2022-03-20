use crate::{api::PitchRs, constants::*, utils::audio_buffer_to_signal, TunerError};

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use pitch_detector::{core::fft_space::FftSpace, note::NoteDetectionResult, pitch::PitchDetector};
use std::sync::Mutex;

lazy_static! {
    static ref TUNER: Mutex<Option<Tuner>> = Mutex::new(None);
}

pub fn tuner_init(algorithm: &str, num_samples: usize, sample_rate: f64) {
    let mut guard = TUNER.lock().unwrap();
    *guard = Some(Tuner::new(algorithm, num_samples, sample_rate));
}

pub fn tuner_change_algorithm(algorithm: &str) -> Result<()> {
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .set_algorithm(algorithm);
    Ok(())
}

pub fn tuner_detect_pitch_with_buffer(byte_buffer: &[u8]) -> Result<Option<NoteDetectionResult>> {
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
    detector: Box<dyn PitchDetector + Send>,
    sample_rate: f64,
    fft_space: FftSpace,
}

impl Tuner {
    pub fn new(algorithm: &str, num_samples: usize, sample_rate: f64) -> Self {
        Self {
            detector: Self::match_algorithm(algorithm),
            sample_rate,
            fft_space: FftSpace::new(num_samples),
        }
    }

    fn match_algorithm(algorithm: &str) -> Box<dyn PitchDetector + Send> {
        match algorithm {
            POWER_CEPSTRUM_ALGORITHM => Box::new(pitch_detector::pitch::cepstrum::PowerCepstrum),

            RAW_FFT_ALGORITHM => Box::new(pitch_detector::pitch::raw_fft::RawFftDetector),
            AUTOCORRELATION_ALGORITHM => {
                Box::new(pitch_detector::pitch::autocorrelation::AutocorrelationDetector)
            }
            _ => panic!("Invalid algorithm"),
        }
    }

    pub fn detect_pitch_with_buffer(&mut self, byte_buffer: &[u8]) -> Option<NoteDetectionResult> {
        let mut signal = audio_buffer_to_signal(byte_buffer);
        self.detector
            .detect(&signal, self.sample_rate)
            .and_then(|f| NoteDetectionResult::try_from(f).ok())
    }

    pub fn set_algorithm(&mut self, algorithm: &str) {
        self.detector = Self::match_algorithm(algorithm);
    }
}

#[cfg(test)]
mod tests {
    const TEST_SAMPLE_RATE: f64 = 44000.0;
    const TEST_NUM_SAMPLES: usize = 17600;

    use super::*;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn detect_with_buffer() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(
            AUTOCORRELATION_ALGORITHM,
            TEST_NUM_SAMPLES,
            TEST_SAMPLE_RATE,
        );
        let pitch: PitchRs = tuner
            .detect_pitch_with_buffer(&buffer)
            .expect("failed to detect pitch")
            .into();
        assert_eq!(pitch.note_name, "A");

        tuner.set_algorithm(POWER_CEPSTRUM_ALGORITHM);
        let pitch: PitchRs = tuner
            .detect_pitch_with_buffer(&buffer)
            .expect("failed to detect pitch")
            .into();
        assert_eq!(pitch.note_name, "A");
        Ok(())
    }
}
