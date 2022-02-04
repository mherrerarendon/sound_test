use crate::{
    api::Partial,
    constants::{
        AUTOCORRELATION_ALGORITHM, FRAME_BUFFER_SIZE, POWER_CEPSTRUM_ALGORITHM, RAW_FFT_ALGORITHM,
    },
    detectors::{autocorrelation, cepstrum, raw_fft, Detector, FundamentalDetector},
    utils::{audio_buffer_to_samples, calc_optimized_fft_space_size},
    TunerError,
};

use anyhow::{bail, Result};
use flutter_rust_bridge::StreamSink;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TUNER: Mutex<Option<Tuner>> = Mutex::new(None);
}

pub fn tuner_change_algorithm(algorithm: &str) -> Result<()> {
    println!("Tuner: Changing algorithm to {}", algorithm);
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .set_algorithm(algorithm)
}

pub fn tuner_init(algorithm: &str) {
    println!("Tuner: Initializing tuner");
    let mut guard = TUNER.lock().unwrap();
    *guard = Some(Tuner::new(algorithm));
}

pub fn tuner_init_stream(sink: StreamSink<Partial>) -> Result<()> {
    println!("Tuner: Initializing tuner");
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .init_stream(sink)
}

pub fn tuner_new_audio_data(byte_buffer: &[u8]) -> Result<()> {
    println!("Tuner: New audio data");
    TUNER
        .lock()
        .unwrap()
        .as_mut()
        .ok_or(TunerError::TunerNotInitialized)?
        .new_audio_data(byte_buffer);
    Ok(())
}

pub struct Tuner {
    detector: Detector,
    sink: Option<StreamSink<Partial>>,
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
            sink: None,
            frame_buffer: [0; FRAME_BUFFER_SIZE],
            remaining_frame_capacity: FRAME_BUFFER_SIZE,
        }
    }

    pub fn init_stream(&mut self, sink: StreamSink<Partial>) -> Result<()> {
        self.sink = Some(sink);
        Ok(())
    }

    fn process_audio_data(&mut self) -> Option<Partial> {
        let mut partial = None;
        if let Some(detected_partial) = self.detect_pitch() {
            partial = Some(detected_partial.clone());
            if let Some(sink) = self.sink.as_ref() {
                sink.add(detected_partial);
            } else {
                println!("Tuner: No sink");
            }
        }

        self.reset_frame_buffer();
        partial
    }

    pub fn new_audio_data(&mut self, byte_buffer: &[u8]) -> Option<Partial> {
        let mut new_pitch = None;
        let samples = audio_buffer_to_samples(byte_buffer);
        if samples.len() > self.remaining_frame_capacity {
            new_pitch = self.process_audio_data();
            self.new_audio_data(byte_buffer)?;
        } else {
            let append_idx = FRAME_BUFFER_SIZE - self.remaining_frame_capacity;
            let append_end_idx = append_idx + samples.len();
            self.frame_buffer[append_idx..append_end_idx].copy_from_slice(&samples);
            self.remaining_frame_capacity -= samples.len();
            if self.remaining_frame_capacity == 0 {
                new_pitch = self.process_audio_data();
            }
        }

        new_pitch
    }

    fn reset_frame_buffer(&mut self) {
        self.frame_buffer.iter_mut().for_each(|sample| *sample = 0);
        self.remaining_frame_capacity = FRAME_BUFFER_SIZE;
    }

    pub fn detect_pitch(&mut self) -> Option<Partial> {
        let iter = self
            .frame_buffer
            .iter()
            .take(FRAME_BUFFER_SIZE - self.remaining_frame_capacity)
            .map(|sample| *sample as f64);
        self.detector.detect_fundamental(iter)
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
    use super::*;
    use float_cmp::ApproxEq;
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct SampleData {
        data: Option<Vec<u8>>,
    }

    #[test]
    fn add_data() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(AUTOCORRELATION_ALGORITHM);
        let partial = tuner.new_audio_data(&buffer).expect("msg");
        assert!(partial.freq.approx_eq(219.634, (0.02, 2)));

        tuner.set_algorithm(POWER_CEPSTRUM_ALGORITHM)?;
        let partial = tuner.new_audio_data(&buffer).expect("msg");
        assert!(partial.freq.approx_eq(219.418, (0.02, 2)));
        Ok(())
    }

    // #[test]
    // fn global_tuner() -> anyhow::Result<()> {
    //     let mut sample_data: SampleData =
    //         serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
    //     let buffer = sample_data.data.take().unwrap();

    //     tuner_init(AUTOCORRELATION_ALGORITHM, buffer.len() / 2);
    //     let partial = tuner_detect_pitch(&buffer)?;
    //     assert!(partial.freq.approx_eq(219.634, (0.02, 2)));

    //     Ok(())
    // }

    // TODO: test with different buffer sizes
}
