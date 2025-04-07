//! Hamming window module.
//! 
//! More info at:
//! - <https://it.mathworks.com/help/signal/ref/hamming.html>
//! - <https://numpy.org/doc/stable/reference/generated/numpy.hamming.html>

use std::convert::From;

use rustfft::num_complex::Complex;

use crate::num_traits::{AsPrimitive, Float, FloatConst};

/// Trait used to prepare a slice of reals to be passed to the extractor.
pub trait Hamming<T> {
    /// Applies a Hamming window, modifying the slice.
    fn hamming(&mut self) -> &mut [T];
    /// Applies a Hamming window, returning a new vec of complex.
    fn hamming_complex(&self) -> Vec<Complex<T>>;
}

/// Trait used to prepare a slice of complex to be passed to the extractor.
pub trait HammingComplex<T> {
    /// Applies a Hamming window, returning a new vec of complex.
    fn hamming(&self) -> Vec<Complex<T>>;

    /// Applies a Hamming window, mutating the slice.
    fn hamming_mut(&mut self) -> &mut [Complex<T>];
}

#[inline(always)]
fn _hamming<T: Float + FloatConst + 'static>(sample: &T, i: usize, len: usize) -> T
where usize: AsPrimitive<T>,
      f32: AsPrimitive<T>,
      f64: AsPrimitive<T>
{
    *sample * (
        0.54.as_() - (
            0.46.as_() * (
                T::PI() * 2.as_() * i.as_() / (len - 1).as_()
            ).cos()
        )
    )
}

#[inline(always)]
fn hamming_complex<T: Copy + Float + FloatConst + 'static>(this: &[T]) -> Vec<Complex<T>>
where usize: AsPrimitive<T>,
      f32: AsPrimitive<T>,
      f64: AsPrimitive<T>
{
    this.iter().enumerate().fold(Vec::with_capacity(this.len()), |mut acc, (i, sample)| {
        acc.push(Complex::from(_hamming(sample, i, this.len())));

        acc
    })
}
#[inline(always)]
fn hamming<T: Copy + Float + FloatConst + 'static>(this: &mut [T])
where usize: AsPrimitive<T>,
      f32: AsPrimitive<T>,
      f64: AsPrimitive<T>
{
    let len = this.len();
    for (i, el) in this.iter_mut().enumerate() {
        *el = _hamming(el, i, len)
    }
}

impl<T: Float + FloatConst + 'static> Hamming<T> for [T]
where usize: AsPrimitive<T>,
      f32: AsPrimitive<T>,
      f64: AsPrimitive<T>
{
    #[inline]
    fn hamming(&mut self) -> &mut [T] {
        hamming(self);
        self
    }

    #[inline]
    fn hamming_complex(&self) -> Vec<Complex<T>> {
        hamming_complex(self)
    }
}

impl<T: Float + FloatConst + 'static> HammingComplex<T> for [Complex<T>]
where usize: AsPrimitive<T>,
      f32: AsPrimitive<T>,
      f64: AsPrimitive<T>
{
    #[inline]
    fn hamming(&self) -> Vec<Complex<T>> {
        self.iter().enumerate().fold(Vec::with_capacity(self.len()), |mut acc, (i, sample)| {
            let mut el = sample.clone();
            el.re = _hamming(&sample.re, i, self.len());
            acc.push(el);

            acc
        })
    }

    #[inline]
    fn hamming_mut(&mut self) -> &mut [Complex<T>] {
        let len = self.len();

        self.iter_mut().enumerate().for_each(|(i, sample)| {
            sample.re = _hamming(&sample.re, i, len);
        });

        self
    }
}