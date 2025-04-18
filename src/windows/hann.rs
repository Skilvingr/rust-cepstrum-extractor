//! Hann window module.
//!
//! More information at <https://it.mathworks.com/help/signal/ref/hann.html>.

use std::convert::From;

use rustfft::num_complex::Complex;

use crate::num_traits::{AsPrimitive, Float, FloatConst};

/// Trait used to prepare a slice of reals to be passed to the extractor.
pub trait Hann<T> {
    /// Applies a Hann window, modifying the slice.
    fn hann(&mut self) -> &mut [T];
    /// Applies a Hann window, returning a new vector of complex numbers.
    fn hann_complex(&self) -> Vec<Complex<T>>;
}

/// Trait used to prepare a slice of complex to be passed to the extractor.
pub trait HannComplex<T> {
    /// Applies a Hann window, returning a new vector of complex numbers.
    fn hann(&self) -> Vec<Complex<T>>;

    /// Applies a Hann window, mutating the slice.
    fn hann_mut(&mut self) -> &mut [Complex<T>];
}

#[inline(always)]
fn _hann<T: Float + FloatConst + 'static>(sample: &T, i: usize, len: usize) -> T
where
    usize: AsPrimitive<T>,
    f32: AsPrimitive<T>,
    f64: AsPrimitive<T>
{
    *sample * (
        0.5.as_() * (
            1.as_() - (
                T::PI() * 2.as_() * i.as_() / (len - 1).as_()
            ).cos()
        )
    )
}

#[inline(always)]
fn hann_complex<T: Copy + Float + FloatConst + 'static>(this: &[T]) -> Vec<Complex<T>>
    where
        usize: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>
{
    this.iter().enumerate().fold(Vec::with_capacity(this.len()), |mut acc, (i, sample)| {
        acc.push(Complex::from(_hann(sample, i, this.len())));

        acc
    })
}
#[inline(always)]
fn hann<T: Copy + Float + FloatConst + 'static>(this: &mut [T])
    where
        usize: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>
{
    let len = this.len();
    for (i, el) in this.iter_mut().enumerate() {
        *el = _hann(el, i, len)
    }
}

impl<T: Float + FloatConst + 'static> Hann<T> for [T]
    where
        usize: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>
{
    #[inline]
    fn hann(&mut self) -> &mut [T] {
        hann(self);
        self
    }

    #[inline]
    fn hann_complex(&self) -> Vec<Complex<T>> {
        hann_complex(self)
    }
}

impl<T: Float + FloatConst + 'static> HannComplex<T> for [Complex<T>]
    where
        usize: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>
{
    #[inline]
    fn hann(&self) -> Vec<Complex<T>> {
        self.iter().enumerate().fold(Vec::with_capacity(self.len()), |mut acc, (i, sample)| {
            let mut el = sample.clone();
            el.re = _hann(&sample.re, i, self.len());
            acc.push(el);

            acc
        })
    }

    #[inline]
    fn hann_mut(&mut self) -> &mut [Complex<T>] {
        let len = self.len();

        self.iter_mut().enumerate().for_each(|(i, sample)| {
            sample.re = _hann(&sample.re, i, len);
        });

        self
    }
}
