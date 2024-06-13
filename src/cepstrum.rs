use rustfft::FftNum;
use rustfft::num_complex::{Complex, ComplexFloat};
use rustfft::num_traits::{Float, FloatConst};

use crate::fft::CepFft;

pub struct CepstrumExtractor<T: FftNum + Float + FloatConst> {
    fft_instance: CepFft<T>,
}

impl<T: FftNum + Float + FloatConst> CepstrumExtractor<T> {
    pub fn new(len: usize) -> CepstrumExtractor<T> {
        Self {
            fft_instance: CepFft::new(len)
        }
    }

    pub fn prepare_instances(&self, instances: usize) {
        self.fft_instance.extend_scratches(instances);
    }

    pub fn extract_cepstrum(&self, signal: &mut [Complex<T>]) {
        self.extract_cepstrum_instance(signal, 0)
    }

    pub fn extract_cepstrum_instance(&self, mut signal: &mut [Complex<T>], instance: usize) {
        self.fft_instance.do_fft(&mut signal, instance);

        signal.iter_mut().for_each(|fft_component| {
            *fft_component = Complex::from(
                match fft_component.re == T::zero() {
                    true => fft_component.abs().abs(),
                    false => fft_component.abs().abs().ln(),
                }
            );
        });

        self.fft_instance.do_ifft(&mut signal, instance);
    }
}