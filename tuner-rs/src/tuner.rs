use crate::{
    api::Partial,
    detectors::{marco_detector, HarmonicDetector},
    TunerError,
};
use pitch_detection::detector::PitchDetector;
use rustfft::{num_complex::Complex, FftPlanner};

const POWER_THRESHOLD: f64 = 5.0;
const CLARITY_THRESHOLD: f64 = 0.7;

pub struct Tuner {
    optimized_num_samples: usize,
}

impl Tuner {
    pub fn new(num_samples: usize) -> Self {
        let optimized_num_samples = Self::optimized_num_samples(num_samples);
        Self {
            optimized_num_samples,
        }
    }

    fn optimized_num_samples(num_samples: usize) -> usize {
        let mut optimized_sum_samples = (2 as usize).pow(14);
        loop {
            if optimized_sum_samples > num_samples {
                optimized_sum_samples /= 2;
            } else {
                break optimized_sum_samples;
            }
        }
    }

    // fn scale_partial(partial: &Partial, num_samples: usize) -> Partial {
    //     let ratio = SAMPLE_RATE / num_samples as f64;
    //     Partial {
    //         freq: (partial.freq as f64 * ratio),
    //         intensity: partial.intensity * PARTIAL_INTENSITY_SCALING,
    //     }
    // }

    pub fn detect_pitch(&mut self, byte_buffer: Vec<u8>) -> Result<Vec<Partial>, TunerError> {
        if self.optimized_num_samples > byte_buffer.len() / 2 {
            return Err(TunerError::FftFailed);
        }

        let signal: Vec<f64> = byte_buffer
            .chunks_exact(2)
            .take(self.optimized_num_samples)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f64)
            .collect();
        let mut marco_detector = marco_detector::MarcoDetector::new(self.optimized_num_samples);
        if let Some(harmonics) = marco_detector.get_harmonics(&signal) {
            Ok(harmonics.harmonics.iter().cloned().collect())
        } else {
            Err(TunerError::FftFailed)
        }

        // let fft = self.fft_planner.plan_fft_forward(samples.len());

        // fft.process_with_scratch(&mut samples, &mut self.scratch);
        // let absolute_values: Vec<(usize, f32)> = samples
        //     .iter()
        //     .enumerate()
        //     .map(|(i, a)| {
        //         let sum = a.re.powf(2.0) + a.im.powf(2.0);
        //         (i, (sum as f32).sqrt())
        //     })
        //     .collect();
        // let harmonics =
        //     HarmonicNote::calc_harmonic_note(&absolute_values).ok_or(TunerError::FftFailed)?;
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
    fn noise() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/noise.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.detect_pitch(buffer)?;
        assert!(partials[0].freq.approx_eq(40.28, (0.02, 2)));
        assert!(partials[2].freq.approx_eq(120.849, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn tuner_c5() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/tuner_c5.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.detect_pitch(buffer)?;
        assert!(partials[0].freq.approx_eq(523.68, (0.02, 2)));
        assert!(partials[1].freq.approx_eq(1047.36, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_a() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_a.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let partials = tuner.detect_pitch(buffer)?;
        assert!(partials[0].freq.approx_eq(220.21, (0.02, 2)));
        assert!(partials[1].freq.approx_eq(440.43, (0.02, 2)));
        assert!(partials[3].freq.approx_eq(880.86, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_d() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_d.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.detect_pitch(buffer)?;
        assert!(fft_peak[0].freq.approx_eq(147.705, (0.02, 2)));
        assert!(fft_peak[1].freq.approx_eq(295.41, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_g() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_g.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.detect_pitch(buffer)?;
        assert!(fft_peak[0].freq.approx_eq(96.68, (0.02, 2)));
        assert!(fft_peak[1].freq.approx_eq(193.36, (0.02, 2)));
        assert!(fft_peak[2].freq.approx_eq(290.04, (0.02, 2)));
        assert!(fft_peak[3].freq.approx_eq(386.72, (0.02, 2)));
        Ok(())
    }

    #[test]
    fn cello_open_c() -> anyhow::Result<()> {
        let mut sample_data: SampleData =
            serde_json::from_str(include_str!("../test_data/cello_open_c.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.detect_pitch(buffer)?;
        assert!(fft_peak[0].freq.approx_eq(64.45, (0.02, 2)));
        assert!(fft_peak[1].freq.approx_eq(128.91, (0.02, 2)));
        assert!(fft_peak[2].freq.approx_eq(193.34, (0.02, 2)));
        assert!(fft_peak[3].freq.approx_eq(257.81, (0.02, 2)));
        Ok(())
    }
}
