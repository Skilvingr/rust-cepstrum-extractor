use rustfft::num_complex::Complex;
use rustfft::num_traits::Num;

/// Performs conversions between real and complex slices.

pub trait RealToComplex<T> {
    /// Produces a vec of complex, given a slice of reals.
    fn to_complex_vec(&self) -> Vec<Complex<T>>;
}
pub trait ComplexToReal<T> {
    /// Produces a vec of reals, given a slice of complex.
    fn to_real_vec(&self) -> Vec<T>;
}

#[inline(always)]
pub fn real_to_complex<T: Clone + Copy + Num>(this: &[T]) -> Vec<Complex<T>> {
    this.iter().fold(Vec::with_capacity(this.len()), |mut acc, sample| {
        acc.push(Complex::from(*sample));

        acc
    })
}

#[inline(always)]
pub fn complex_to_real<T: Clone + Copy + Num>(this: &[Complex<T>]) -> Vec<T> {
    this.iter().fold(Vec::with_capacity(this.len()), |mut acc, sample| {
        acc.push(sample.re);

        acc
    })
}

impl RealToComplex<f32> for [f32] {
    #[inline(always)]
    fn to_complex_vec(&self) -> Vec<Complex<f32>> {
        real_to_complex(self)
    }
}
impl ComplexToReal<f32> for [Complex<f32>] {
    #[inline(always)]
    fn to_real_vec(&self) -> Vec<f32> {
        complex_to_real(self)
    }
}

impl RealToComplex<f64> for [f64] {
    #[inline(always)]
    fn to_complex_vec(&self) -> Vec<Complex<f64>> {
        real_to_complex(self)
    }
}
impl ComplexToReal<f64> for [Complex<f64>] {
    #[inline(always)]
    fn to_real_vec(&self) -> Vec<f64> {
        complex_to_real(self)
    }
}