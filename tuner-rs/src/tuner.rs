use crate::{api::FftComponent, TunerError};
use rustfft::{num_complex::Complex, FftPlanner};

const SAMPLE_RATE: f32 = 44000.0;

pub struct Harmonics {
    components: Vec<FftComponent>,
}

impl Harmonics {
    pub fn new(max_components: usize, absolute_values: &[(usize, f32)]) -> Self {
        Self {
            components: Self::calc_harmonic_components(max_components, absolute_values),
        }
    }

    pub fn components(&self) -> &[FftComponent] {
        &self.components
    }

    fn highest_intensity_components(
        num: usize,
        absolute_values: &[(usize, f32)],
    ) -> Vec<FftComponent> {
        let mut highest_intensity_components: Vec<FftComponent> =
            vec![FftComponent::default(); num];
        absolute_values.iter().for_each(|a| {
            Self::add_component_if_high_intensity(a, &mut highest_intensity_components);
        });
        let ratio = SAMPLE_RATE / absolute_values.len() as f32;
        highest_intensity_components
            .iter()
            .map(|component| FftComponent {
                freq: (component.freq as f32 * ratio),
                intensity: component.intensity / 1000.0,
            })
            .collect()
    }

    fn calc_harmonic_components(num: usize, absolute_values: &[(usize, f32)]) -> Vec<FftComponent> {
        let highest_intensity_components = Self::highest_intensity_components(num, absolute_values);
        let highest_intensity_component = highest_intensity_components
            .iter()
            .reduce(|accum, item| {
                if item.intensity > accum.intensity {
                    item
                } else {
                    accum
                }
            })
            .unwrap();
        let mut harmonics: Vec<FftComponent> = highest_intensity_components
            .iter()
            .filter(|component| Self::harmonic(component, highest_intensity_component).is_some())
            .map(|component| component.clone())
            .collect();
        harmonics.sort_by_key(|component| component.freq.round() as usize);
        harmonics
    }

    fn harmonic(component: &FftComponent, harmonic: &FftComponent) -> Option<usize> {
        if (harmonic.freq.round() as usize) % (component.freq.round() as usize) == 0 {
            Some(harmonic.freq.round() as usize / component.freq.round() as usize)
        } else {
            None
        }
    }

    fn add_component_if_high_intensity(
        component: &(usize, f32),
        highest_intensity_components: &mut Vec<FftComponent>,
    ) {
        let least_intense_idx = Self::get_index_of_lowest_intensity(&highest_intensity_components);
        let least_intense_component = &highest_intensity_components[least_intense_idx];
        if component.1 > least_intense_component.intensity {
            highest_intensity_components[least_intense_idx] = FftComponent {
                freq: component.0 as f32,
                intensity: component.1,
            };
        }
    }

    fn get_index_of_lowest_intensity(values: &[FftComponent]) -> usize {
        values
            .into_iter()
            .enumerate()
            .reduce(|accum, item| {
                if item.1.intensity < accum.1.intensity {
                    item
                } else {
                    accum
                }
            })
            .unwrap()
            .0
    }
}

pub struct Tuner {
    fft_planner: FftPlanner<f32>,
    optimized_num_samples: usize,
    scratch: Vec<Complex<f32>>,
}

impl Tuner {
    pub fn new(num_samples: usize) -> Self {
        let optimized_num_samples = Self::optimized_num_samples(num_samples);
        Self {
            fft_planner: FftPlanner::new(),
            optimized_num_samples,
            scratch: vec![Complex::new(0.0, 0.0); optimized_num_samples],
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

    pub fn fft(&mut self, byte_buffer: Vec<u8>) -> Result<FftComponent, TunerError> {
        if self.optimized_num_samples > byte_buffer.len() / 2 {
            return Err(TunerError::FftFailed);
        }

        let mut samples: Vec<Complex<f32>> = byte_buffer
            .chunks_exact(2)
            .take(self.optimized_num_samples)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]))
            .map(|a| Complex {
                re: a as f32,
                im: 0.0f32,
            })
            .collect();

        let fft = self.fft_planner.plan_fft_forward(samples.len());

        fft.process_with_scratch(&mut samples, &mut self.scratch);
        let absolute_values: Vec<(usize, f32)> = samples
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let sum = a.re.powf(2.0) + a.im.powf(2.0);
                (i, (sum as f32).sqrt())
            })
            .collect();
        let harmonics = Harmonics::new(5, &absolute_values);

        Ok(harmonics.components()[0].clone())
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
    fn fft_works() -> anyhow::Result<()> {
        let mut sample_data: SampleData = serde_json::from_str(include_str!("sampleData.json"))?;
        let buffer = sample_data.data.take().unwrap();
        let mut tuner = Tuner::new(buffer.len() / 2);
        let fft_peak = tuner.fft(buffer)?;
        assert!(fft_peak.freq.approx_eq(120.849, (0.02, 2)));
        Ok(())
    }
}
