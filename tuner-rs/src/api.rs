use pitch_detector::note::NoteDetectionResult;

use crate::constants::{A4_FREQ, MAX_CENTS_OFFSET, NOTES};
use crate::tuner::{tuner_change_algorithm, tuner_detect_pitch_with_buffer, tuner_init};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Partial {
    pub freq: f64,
    pub intensity: f64,
}

impl Default for Partial {
    fn default() -> Self {
        Self {
            freq: 0.0,
            intensity: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PitchRs {
    pub note_name: String,
    pub octave: i32,
    pub cents_offset: f64,
    pub previous_note_name: String,
    pub next_note_name: String,
    pub in_tune: bool,
}

impl From<NoteDetectionResult> for PitchRs {
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

pub fn detect_pitch_with_buffer(byte_buffer: Vec<u8>) -> anyhow::Result<Option<PitchRs>> {
    tuner_detect_pitch_with_buffer(&byte_buffer)
}
