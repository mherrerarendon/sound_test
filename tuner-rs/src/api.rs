use crate::constants::{A4_FREQ, MAX_CENTS_OFFSET, MIN_FREQ, NOTES};
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

// This struct is used for auto-generated code, so it has to be declared in this file.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pitch {
    pub note_name: String,
    pub octave: i32,
    pub cents_offset: f64,
    pub previous_note_name: String,
    pub next_note_name: String,
    pub in_tune: bool,
}

impl TryFrom<f64> for Pitch {
    type Error = anyhow::Error;
    fn try_from(freq: f64) -> Result<Self, Self::Error> {
        if freq < MIN_FREQ {
            return Err(anyhow::anyhow!("Invalid frequency: {}", freq));
        }
        let steps_from_a4 = (freq / A4_FREQ).log2() * 12.0;
        let steps_from_c5 = steps_from_a4 - 2.0;
        let cents_offset = (steps_from_a4 - steps_from_a4.round()) * 100.0;
        Ok(Self {
            note_name: NOTES[(steps_from_a4.round() as usize) % NOTES.len()].into(),
            octave: (5. + (steps_from_c5 / 12.0).floor()) as i32,
            cents_offset,
            previous_note_name: NOTES
                [(steps_from_a4.round() as isize - 1).rem_euclid(NOTES.len() as isize) as usize]
                .into(),
            next_note_name: NOTES[(steps_from_a4.round() as usize + 1) % NOTES.len()].into(),
            in_tune: cents_offset.abs() < MAX_CENTS_OFFSET,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use float_cmp::ApproxEq;
    fn test_pitch_from_f64(
        freq: f64,
        note_name: &str,
        octave: i32,
        cents_offset: f64,
        previous_note_name: &str,
        next_note_name: &str,
        in_tune: bool,
    ) -> Result<()> {
        let pitch = Pitch::try_from(freq)?;
        assert_eq!(pitch.note_name, note_name);
        assert_eq!(pitch.octave, octave);
        assert!(pitch.cents_offset.approx_eq(cents_offset, (0.02, 2)));
        assert_eq!(pitch.previous_note_name, previous_note_name);
        assert_eq!(pitch.next_note_name, next_note_name);
        assert_eq!(pitch.in_tune, in_tune);
        Ok(())
    }

    #[test]
    fn pitch_from_f64_works() -> Result<()> {
        test_pitch_from_f64(440., "A", 4, 0., "G#", "A#", true)?;
        test_pitch_from_f64(493.88, "B", 4, 0., "A#", "C", true)?;
        test_pitch_from_f64(523.25, "C", 5, 0., "B", "C#", true)?;
        test_pitch_from_f64(880., "A", 5, 0., "G#", "A#", true)?;
        test_pitch_from_f64(220., "A", 3, 0., "G#", "A#", true)?;
        test_pitch_from_f64(27.51, "A", 0, 0., "G#", "A#", true)?;
        assert!(test_pitch_from_f64(0., "A", 0, 0., "G#", "A#", true).is_err());
        Ok(())
    }
}
