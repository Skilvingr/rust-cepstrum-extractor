use std::ptr;
use std::sync::{Arc, RwLock};

use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use crate::CepFloat;

pub struct CepFft<T: CepFloat> {
    len: usize,
    scratches: RwLock<Vec<Vec<Complex<T>>>>,

    fft_instance: Arc<dyn Fft<T>>,
    ifft_instance: Arc<dyn Fft<T>>
}

impl<T: CepFloat> CepFft<T> {
    pub fn new(len: usize) -> CepFft<T> {
        let mut fft_planner = FftPlanner::<T>::new();

        let fft_instance = fft_planner.plan_fft_forward(len);

        CepFft {
            len,
            scratches: RwLock::new(vec![vec![Complex::zero(); fft_instance.get_inplace_scratch_len()]]),

            fft_instance,
            ifft_instance: fft_planner.plan_fft_inverse(len),
        }
    }

    pub fn extend_scratches(&self, new_count: usize) {
        let mut s = self.scratches.write().unwrap();
        let len = s.len();

        if new_count > s.len() {
            s.extend(
                (0..new_count - len).map(|_| vec![Complex::zero(); self.fft_instance.get_inplace_scratch_len()])
            )
        }
    }

    #[inline(always)]
    fn retrieve_scratch(&self, i: usize) -> &mut [Complex<T>] {
        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(
                self.scratches.write().map(|mut scratches| {
                    scratches[i].as_mut_ptr()
                }).unwrap(),
                self.len
            )
        }
    }

    #[inline]
    pub fn do_fft(&self, input: &mut [Complex<T>], i: usize) {
        self.fft_instance.process_with_scratch(
            input,
            self.retrieve_scratch(i)
        );
    }

    #[inline]
    pub fn do_ifft(&self, input: &mut [Complex<T>], i: usize) {
        self.ifft_instance.process_with_scratch(
            input,
            self.retrieve_scratch(i)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::CepFft;

    #[test]
    fn check_scratches() {
        let inst: CepFft<f32> = CepFft::new(10);

        assert_eq!(inst.scratches.read().unwrap().len(), 1);

        inst.extend_scratches(10);

        assert_eq!(inst.scratches.read().unwrap().len(), 10);

        inst.extend_scratches(9);

        assert_eq!(inst.scratches.read().unwrap().len(), 10);
    }
}