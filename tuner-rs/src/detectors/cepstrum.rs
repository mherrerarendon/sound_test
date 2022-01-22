use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, SAMPLE_RATE},
    detectors::{HarmonicDetector, HarmonicPitch},
};
use num_traits::Zero;
use rustfft::{num_complex::Complex, FftPlanner};

pub struct CepstrumDetector {
    fft: Vec<Complex<f64>>,
    scratch: Vec<Complex<f64>>,
}

impl HarmonicDetector<f64> for CepstrumDetector {
    fn get_harmonics(&mut self, signal: &[f64]) -> Option<HarmonicPitch> {
        assert_eq!(signal.len(), self.scratch.len());
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(signal.len());
        self.fft = signal.iter().map(|x| Complex::new(*x, 0.0)).collect();

        forward_fft.process_with_scratch(&mut self.fft, &mut self.scratch);
        let quefrency = self.power_spectrum(&mut planner);
        quefrency.map(|partial| HarmonicPitch::new(partial))
    }
}

impl CepstrumDetector {
    pub fn new(num_samples: usize) -> Self {
        CepstrumDetector {
            scratch: vec![Complex::zero(); num_samples],
            fft: vec![Complex::zero(); num_samples],
        }
    }

    fn complex_spectrum(&mut self, planner: &mut FftPlanner<f64>) -> Option<Partial> {
        self.fft = self
            .fft
            .iter()
            .map(|f| {
                let sum = f.re.powi(2) + f.im.powi(2);
                Complex::new(sum.sqrt().log(std::f64::consts::E), (f.im / f.re).atan())
            })
            .collect();
        let inverse_fft = planner.plan_fft_inverse(self.scratch.len());
        inverse_fft.process_with_scratch(&mut self.fft, &mut self.scratch);
        let quefrency =
            self.fft.iter().enumerate().reduce(
                |accum, item| {
                    if item.1.re > accum.1.re {
                        item
                    } else {
                        accum
                    }
                },
            );
        let fundamental = quefrency.map(|quefrency| Partial {
            freq: SAMPLE_RATE / quefrency.0 as f64,
            intensity: quefrency.1.re,
        });
        fundamental
    }

    fn power_spectrum(&mut self, planner: &mut FftPlanner<f64>) -> Option<Partial> {
        self.fft = self
            .fft
            .iter()
            .map(|f| Complex::new((f.re.powi(2) + f.im.powi(2)).log(std::f64::consts::E), 0.0))
            .collect();
        let forward_fft = planner.plan_fft_forward(self.scratch.len());
        forward_fft.process_with_scratch(&mut self.fft, &mut self.scratch);

        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        let power_cepstrum: Vec<f64> = self
            .fft
            .iter()
            .map(|i| i.re.powi(2) + i.im.powi(2))
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .collect();
        let quefrency =
            power_cepstrum.iter().enumerate().reduce(
                |accum, item| {
                    if item.1 > accum.1 {
                        item
                    } else {
                        accum
                    }
                },
            );
        let fundamental = quefrency.map(|quefrency| Partial {
            freq: SAMPLE_RATE / (quefrency.0 as f64 + lower_limit as f64) as f64,
            intensity: *quefrency.1,
        });
        fundamental
    }
}
