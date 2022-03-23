use pitch_detector::note::NoteDetectionResult;

use crate::tuner::{tuner_change_algorithm, tuner_detect_pitch_with_buffer, tuner_init};
use pitch_detector::pitch::core::zero_crossing_rate;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NoteResult {
    pub note_name: String,
    pub octave: i32,
    pub cents_offset: f64,
    pub previous_note_name: String,
    pub next_note_name: String,
    pub in_tune: bool,
}

impl From<NoteDetectionResult> for NoteResult {
    fn from(note_result: NoteDetectionResult) -> Self {
        Self {
            note_name: note_result.note_name.to_string(),
            octave: note_result.octave,
            cents_offset: note_result.cents_offset,
            previous_note_name: note_result.previous_note_name.to_string(),
            next_note_name: note_result.next_note_name.to_string(),
            in_tune: note_result.in_tune,
        }
    }
}

pub fn change_algorithm(algorithm: String) -> anyhow::Result<()> {
    tuner_change_algorithm(&algorithm)
}

pub fn init_tuner(algorithm: String, num_samples: u32, sample_rate: f64) -> anyhow::Result<()> {
    tuner_init(&algorithm, num_samples as usize, sample_rate);
    Ok(())
}

pub fn detect_pitch_with_buffer(buffer: Vec<f64>) -> anyhow::Result<Option<NoteResult>> {
    let result = tuner_detect_pitch_with_buffer(&buffer);
    let mapped_result = match result {
        Ok(note_result) => Ok(note_result.map(|n| NoteResult::from(n))),
        Err(err) => Err(err),
    };
    mapped_result
}

pub fn zero_cross_rate(buffer: Vec<f64>, sample_rate: u32) -> anyhow::Result<f64> {
    let rate = zero_crossing_rate(buffer, sample_rate as f64);
    Ok(rate)
}
