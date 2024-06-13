use rustfft::num_complex::Complex;
use rustfft::num_traits::Num;

#[inline(always)]
pub fn hann_complex<T: Clone + Copy + Num>(this: &[T], f: fn(&T, usize, usize) -> T) -> Vec<Complex<T>> {
    this.iter().enumerate().fold(Vec::with_capacity(this.len()), |mut acc, (i, sample)| {
        acc.push(Complex::from(f(sample, i, this.len())));

        acc
    })
}
#[inline(always)]
pub fn hann<T: Clone + Copy + Num>(this: &mut [T], f: fn(&T, usize, usize) -> T) {
    let len = this.len();
    for (i, el) in this.iter_mut().enumerate() {
        *el = f(el, i, len)
    }
}

#[inline(always)]
pub fn _hann_f32(sample: &f32, i: usize, len: usize) -> f32 {
    sample * (
        0.5 - (
            0.5 * (
                core::f32::consts::PI * 2. * i as f32 / (len - 1) as f32
            ).cos()
        )
    )
}
#[inline(always)]
pub fn _hann_f64(sample: &f64, i: usize, len: usize) -> f64 {
    sample * (
        0.5 - (
            0.5 * (
                core::f64::consts::PI * 2. * i as f64 / (len - 1) as f64
            ).cos()
        )
    )
}
