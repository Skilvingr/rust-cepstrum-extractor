use rustfft::num_complex::Complex;
use rustfft::num_traits::Num;
use crate::prepare::hann::{_hann_f32, _hann_f64, hann, hann_complex};

mod hann;


#[inline(always)]
pub fn real_to_complex<T: Clone + Copy + Num>(this: &[T]) -> Vec<Complex<T>> {
    this.iter().fold(Vec::with_capacity(this.len()), |mut acc, sample| {
        acc.push(Complex::from(*sample));

        acc
    })
}


/// Trait used to prepare a slice to be passed to the extractor.
/// Currently implemented for `f32` and `f64`.
pub trait Prepare<T> {
    /// Transforms a slice of reals into a slice of complex, returning a new vec.
    fn to_complex_slice(&self) -> Vec<Complex<T>>;
    /// Applies a Hann window, modifying the slice.
    fn apply_hann_window(&mut self) -> &mut [T];
    /// Applies a Hann window, returning a new vec of complex.
    fn apply_hann_window_complex(&self) -> Vec<Complex<T>>;
}

impl Prepare<f32> for [f32] {
    fn to_complex_slice(&self) -> Vec<Complex<f32>> {
        real_to_complex(self)
    }

    fn apply_hann_window(&mut self) -> &mut [f32] {
        hann(self, _hann_f32);
        self
    }

    #[inline]
    fn apply_hann_window_complex(&self) -> Vec<Complex<f32>> {
        hann_complex(self, _hann_f32)
    }
}

impl Prepare<f64> for [f64] {
    fn to_complex_slice(&self) -> Vec<Complex<f64>> {
        real_to_complex(self)
    }

    fn apply_hann_window(&mut self) -> &mut [f64] {
        hann(self, _hann_f64);
        self
    }

    #[inline]
    fn apply_hann_window_complex(&self) -> Vec<Complex<f64>> {
        hann_complex(self, _hann_f64)
    }
}

