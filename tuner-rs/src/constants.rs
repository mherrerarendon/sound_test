pub const RAW_FFT_ALGORITHM: &str = "rawfft";
pub const POWER_CEPSTRUM_ALGORITHM: &str = "power";
pub const AUTOCORRELATION_ALGORITHM: &str = "autocorrelation";

pub const ZERO_CROSSING_RATE_THRESHOLD: usize = 3;
pub const A4_FREQ: f64 = 440.0;
pub const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// Noticable pitch difference starts at around 10-25 cents
pub const MAX_CENTS_OFFSET: f64 = 10.0;
