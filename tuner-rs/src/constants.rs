pub const SAMPLE_RATE: f64 = 44000.0;
pub const MAX_FREQ: f64 = 4186.0;
pub const MIN_FREQ: f64 = 27.5;
pub const FRAME_BUFFER_SIZE: usize = 17600; // num of samples (2 bytes per sample)

pub const RAW_FFT_ALGORITHM: &str = "rawfft";
pub const POWER_CEPSTRUM_ALGORITHM: &str = "power";
pub const AUTOCORRELATION_ALGORITHM: &str = "autocorrelation";

pub const ZERO_CROSSING_RATE_THRESHOLD: usize = 3;
pub const A4_FREQ: f64 = 440.0;
pub const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
