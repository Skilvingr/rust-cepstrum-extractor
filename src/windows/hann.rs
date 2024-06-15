use rustfft::num_complex::Complex;
use rustfft::num_traits::Num;

/// Trait used to prepare a slice to be passed to the extractor.
/// Currently implemented for `f32` and `f64`.
pub trait Hann<T> {
    /// Applies a Hann window, modifying the slice.
    fn apply_hann_window(&mut self) -> &mut [T];
    /// Applies a Hann window, returning a new vec of complex.
    fn apply_hann_window_complex(&self) -> Vec<Complex<T>>;
}

#[inline(always)]
fn hann_complex<T: Clone + Copy + Num>(this: &[T], f: fn(&T, usize, usize) -> T) -> Vec<Complex<T>> {
    this.iter().enumerate().fold(Vec::with_capacity(this.len()), |mut acc, (i, sample)| {
        acc.push(Complex::from(f(sample, i, this.len())));

        acc
    })
}
#[inline(always)]
fn hann<T: Clone + Copy + Num>(this: &mut [T], f: fn(&T, usize, usize) -> T) {
    let len = this.len();
    for (i, el) in this.iter_mut().enumerate() {
        *el = f(el, i, len)
    }
}

#[inline(always)]
fn _hann_f32(sample: &f32, i: usize, len: usize) -> f32 {
    sample * (
        0.5 - (
            0.5 * (
                core::f32::consts::PI * 2. * i as f32 / (len - 1) as f32
            ).cos()
        )
    )
}
#[inline(always)]
fn _hann_f64(sample: &f64, i: usize, len: usize) -> f64 {
    sample * (
        0.5 - (
            0.5 * (
                core::f64::consts::PI * 2. * i as f64 / (len - 1) as f64
            ).cos()
        )
    )
}


impl Hann<f32> for [f32] {
    fn apply_hann_window(&mut self) -> &mut [f32] {
        hann(self, _hann_f32);
        self
    }

    #[inline]
    fn apply_hann_window_complex(&self) -> Vec<Complex<f32>> {
        hann_complex(self, _hann_f32)
    }
}

impl Hann<f64> for [f64] {
    fn apply_hann_window(&mut self) -> &mut [f64] {
        hann(self, _hann_f64);
        self
    }

    #[inline]
    fn apply_hann_window_complex(&self) -> Vec<Complex<f64>> {
        hann_complex(self, _hann_f64)
    }
}