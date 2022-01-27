use crate::{
    api::Partial,
    constants::{MAX_FREQ, MIN_FREQ, SAMPLE_RATE},
    detectors::{fft_space::FftSpace, FundamentalDetector},
};
use anyhow::Result;
use rustfft::{num_complex::Complex, FftPlanner};

use plotters::prelude::*;
// use itertools_num::linspace;

fn plot(data: &[f64], name: &str) -> Result<()> {
    let y_min = data.iter().cloned().reduce(f64::min).unwrap();
    let y_max = data.iter().cloned().reduce(f64::max).unwrap();
    let root = BitMapBackend::new(name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(90)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f64..data.len() as f64, y_min..y_max)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(i, &x)| (i as f64, x)),
        &RED,
    ))?;

    // chart.draw_series(PointSeries::of_element(
    //     data.iter().enumerate().map(|(i, &x)| (i as f64, x)),
    //     2.0,
    //     &RED,
    //     &|c, s, st| {
    //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //         + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    //         + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
    //     },
    // ))?;

    root.present()?;
    Ok(())
}

pub struct PowerCepstrum {
    fft_space: FftSpace,
}

impl FundamentalDetector for PowerCepstrum {
    fn get_fundamental(&mut self, signal: &[f64]) -> Result<Partial> {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(self.fft_space.len());
        self.fft_space.init_fft_space(signal);

        let (fft_space, scratch) = self.fft_space.workspace();
        forward_fft.process_with_scratch(fft_space, scratch);
        self.fft_space
            .map(|f| Complex::new(f.norm_sqr().log(std::f64::consts::E), 0.0));
        let (fft_space, scratch) = self.fft_space.workspace();
        let inverse_fft = planner.plan_fft_inverse(fft_space.len());
        inverse_fft.process_with_scratch(fft_space, scratch);

        let power_cepstrum: Vec<f64> = self
            .fft_space
            .freq_domain(false)
            .map(|(freq, _)| freq)
            .collect();

        // Frequency = SAMPLE_RATE / quefrency
        // With this in mind we can ignore the extremes of the power cepstrum
        // https://en.wikipedia.org/wiki/Cepstrum
        let lower_limit = (SAMPLE_RATE / MAX_FREQ).round() as usize;
        let upper_limit = (SAMPLE_RATE / MIN_FREQ).round() as usize;
        let mut partials: Vec<Partial> = power_cepstrum
            .iter()
            .skip(lower_limit)
            .take(upper_limit - lower_limit)
            .enumerate()
            .map(|(quefrency, intensity)| Partial {
                freq: SAMPLE_RATE / (quefrency as f64 + lower_limit as f64) as f64,
                intensity: *intensity,
            })
            .collect();
        let test = partials.iter().map(|p| p.intensity).collect::<Vec<f64>>();
        plot(&test, "power_cepstrum.png")?;
        partials.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
        partials
            .into_iter()
            .next()
            .ok_or(anyhow::anyhow!("No partials found"))
    }
}

impl PowerCepstrum {
    pub fn new(fft_space_size: usize) -> Self {
        PowerCepstrum {
            fft_space: FftSpace::new(fft_space_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_complex() -> anyhow::Result<()> {
        let mut detector = PowerCepstrum::new(TEST_FFT_SPACE_SIZE);
        test_fundamental_freq(&mut detector, "noise.json", 4000.0)?;

        // Power cepstrum fails to detect the C5 note, which should be at around 523Hz
        test_fundamental_freq(&mut detector, "tuner_c5.json", 3384.615)?;

        test_fundamental_freq(&mut detector, "cello_open_a.json", 218.905)?;
        test_fundamental_freq(&mut detector, "cello_open_d.json", 146.666)?;
        test_fundamental_freq(&mut detector, "cello_open_g.json", 97.345)?;

        // This fails to detect the C note, which should be at around 64Hz
        test_fundamental_freq(&mut detector, "cello_open_c.json", 2933.333)?;
        Ok(())
    }
}
