const HISTORY_SIZE: usize = 10;
const A4_FREQ: f64 = 440.0;
const A4_MIDI: f64 = 69.0;
const VALID_RANGE: f64 = 6.0;
const MIN_FILTER_FREQ: f64 = 55.0; // A1
pub struct TunerFilter {
    pitches: [f64; HISTORY_SIZE],
    curr_idx: usize,
}

impl TunerFilter {
    pub fn new() -> Self {
        Self {
            pitches: [A4_FREQ; HISTORY_SIZE],
            curr_idx: 0,
        }
    }

    pub fn add_freq(&mut self, freq: f64) {
        if freq > MIN_FILTER_FREQ {
            self.pitches[self.curr_idx] = self.freq_to_pitch(freq);
            self.curr_idx = (self.curr_idx + 1) % HISTORY_SIZE;
        }
    }

    pub fn within_range(&self, freq: f64) -> bool {
        let average_pitch = self.get_average_pitch();
        let pitch = self.freq_to_pitch(freq);
        (pitch - average_pitch).abs() < VALID_RANGE
    }

    fn freq_to_pitch(&self, freq: f64) -> f64 {
        (freq / A4_FREQ).log2() * 12.0 + A4_MIDI
    }

    fn get_average_pitch(&self) -> f64 {
        self.pitches.iter().sum::<f64>() / HISTORY_SIZE as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
