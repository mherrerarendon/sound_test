use num_traits::Zero;
use rustfft::num_complex::Complex;

mod utils {
    use rustfft::num_complex::Complex;
    pub struct FreqDomainIter<'a> {
        pub(super) complex_iter: std::slice::Iter<'a, Complex<f64>>,
        pub(super) square_rooted: bool,
    }

    impl Iterator for FreqDomainIter<'_> {
        type Item = (f64, f64);

        fn next(&mut self) -> Option<Self::Item> {
            match self.complex_iter.next() {
                Some(complex) => {
                    let value = complex.norm_sqr();
                    let phase = complex.arg();
                    if self.square_rooted {
                        Some((value.sqrt(), phase))
                    } else {
                        Some((value, phase))
                    }
                }
                None => None,
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.complex_iter.size_hint()
        }
    }
}
pub struct FftSpace {
    space: Vec<Complex<f64>>,
    scratch: Vec<Complex<f64>>,
}

impl FftSpace {
    pub fn new(size: usize) -> Self {
        FftSpace {
            space: vec![Complex::zero(); size],
            scratch: vec![Complex::zero(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.space.len()
    }

    pub fn space(&self) -> &[Complex<f64>] {
        &self.space
    }

    pub fn workspace(&mut self) -> (&mut [Complex<f64>], &mut [Complex<f64>]) {
        (&mut self.space, &mut self.scratch)
    }

    pub fn init_fft_space(&mut self, signal: &[f64]) {
        assert!(signal.len() <= self.space.len());
        signal
            .iter()
            .zip(self.space.iter_mut())
            .for_each(|(sample, fft)| {
                fft.re = *sample;
                fft.im = 0.0;
            });
        self.space[signal.len()..]
            .iter_mut()
            .for_each(|o| *o = Complex::zero())
    }

    pub fn freq_domain(&self, square_rooted: bool) -> utils::FreqDomainIter {
        utils::FreqDomainIter {
            complex_iter: self.space.iter(),
            square_rooted,
        }
    }
}

impl FromIterator<Complex<f64>> for FftSpace {
    fn from_iter<I: IntoIterator<Item = Complex<f64>>>(iter: I) -> Self {
        let the_iter = iter.into_iter();
        let mut fft_space = FftSpace::new(the_iter.size_hint().1.unwrap());
        for (idx, complex) in the_iter.enumerate() {
            fft_space.space[idx] = complex;
        }

        fft_space
    }
}
