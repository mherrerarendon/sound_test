use crate::constants::{A4_FREQ, NOTES};
use crate::tuner::{tuner_change_algorithm, tuner_detect_pitch_with_buffer, tuner_init};

#[derive(Debug, Clone)]
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

// This struct is used for auto-generated code, so it has to be declared in this file.
#[derive(Debug, Clone)]
pub struct Pitch {
    pub note_name: String,
    pub cents_offset: f64,
    pub previous_note_name: String,
    pub next_note_name: String,
}

impl From<f64> for Pitch {
    fn from(freq: f64) -> Self {
        let steps_from_a4 = (freq / A4_FREQ).log2() * 12.0;
        Self {
            note_name: NOTES[(steps_from_a4.round() as usize) % NOTES.len()].into(),
            cents_offset: (steps_from_a4 - steps_from_a4.round()) * 100.0,
            previous_note_name: NOTES[(steps_from_a4.round() as usize - 1) % NOTES.len()].into(),
            next_note_name: NOTES[(steps_from_a4.round() as usize + 1) % NOTES.len()].into(),
        }
    }
}

pub fn change_algorithm(algorithm: String) -> anyhow::Result<()> {
    tuner_change_algorithm(&algorithm)
}

pub fn init_tuner(algorithm: String) -> anyhow::Result<()> {
    tuner_init(&algorithm);
    Ok(())
}

pub fn detect_pitch_with_buffer(byte_buffer: Vec<u8>) -> anyhow::Result<Option<Pitch>> {
    tuner_detect_pitch_with_buffer(&byte_buffer)
}
