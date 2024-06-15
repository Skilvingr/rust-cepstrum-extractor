#![doc = include_str!("../README.md")]

use rustfft::FftNum;
pub use rustfft::num_complex;
pub use rustfft::num_traits;
use rustfft::num_traits::{Float, FloatConst};

pub use cepstrum::CepstrumExtractor;
pub use conversions::{ComplexToReal, RealToComplex};
pub use windows::hann::Hann;

mod fft;
mod cepstrum;
mod windows;
mod conversions;

pub trait CepFloat: FftNum + Float + FloatConst {}

impl CepFloat for f32 {}
impl CepFloat for f64 {}
