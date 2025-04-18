use std::{iter, ptr};
use std::sync::{Arc, Mutex};

use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use crate::CepFloat;

pub struct CepFft<T: CepFloat> {
    len: usize,
    scratch_len: usize,
    scratches: Mutex<Vec<Vec<Complex<T>>>>,

    fft_instance: Arc<dyn Fft<T>>,
    ifft_instance: Arc<dyn Fft<T>>
}

impl<T: CepFloat> CepFft<T> {
    pub fn new(len: usize) -> CepFft<T> {
        let mut fft_planner = FftPlanner::<T>::new();

        let fft_instance = fft_planner.plan_fft_forward(len);

        CepFft {
            len,
            scratch_len: fft_instance.get_inplace_scratch_len(),
            scratches: Mutex::new(vec![vec![Complex::zero(); fft_instance.get_inplace_scratch_len()]]),

            fft_instance,
            ifft_instance: fft_planner.plan_fft_inverse(len),
        }
    }

    pub fn set_len(&mut self, len: usize) {
        if len == self.len {
            return;
        }

        let mut fft_planner = FftPlanner::<T>::new();

        self.fft_instance = fft_planner.plan_fft_forward(len);
        self.ifft_instance = fft_planner.plan_fft_inverse(len);

        let new_scratches_len = self.fft_instance.get_inplace_scratch_len();

        for scratch in self.scratches.lock().unwrap().iter_mut() {
            if new_scratches_len > scratch.len() {
                scratch.extend(iter::repeat(Complex::zero()).take(new_scratches_len - scratch.len()));
            } else {
                scratch.truncate(new_scratches_len);
            }
        }

        self.scratch_len = new_scratches_len;
        self.len = len;
    }

    pub fn extend_scratches(&self, new_count: usize) {
        let mut s = self.scratches.lock().unwrap();
        let len = s.len();

        if new_count > s.len() {
            s.extend(
                iter::repeat(vec![Complex::zero(); self.scratch_len]).take(new_count - len)
            )
        }
    }

    #[inline(always)]
    fn retrieve_scratch(&self, i: usize) -> &mut [Complex<T>] {
        let mut scratches = self.scratches.lock().unwrap();
        assert!(i < scratches.len(), "Index out of bounds: {}", i);

        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(
                scratches.get_unchecked_mut(i).as_mut_ptr(),
                self.scratch_len
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

    const LEN: usize = 10;

    #[test]
    fn check_scratches() {
        let mut inst: CepFft<f32> = CepFft::new(LEN);

        assert_eq!(inst.scratches.lock().unwrap().len(), 1);

        inst.extend_scratches(10);

        assert_eq!(inst.scratches.lock().unwrap().len(), 10);

        inst.extend_scratches(9);

        assert_eq!(inst.scratches.lock().unwrap().len(), 10);

        assert!(inst.scratches.lock().unwrap().iter().all(|s| s.len() == inst.scratch_len && s.len() == LEN));

        inst.set_len(LEN * 2);

        assert!(inst.scratches.lock().unwrap().iter().all(|s| s.len() == inst.scratch_len && s.len() == LEN * 2));
    }
}