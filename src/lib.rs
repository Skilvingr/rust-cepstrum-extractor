#![doc = include_str!("../README.md")]

use rustfft::FftNum;
pub use rustfft::num_complex;
pub use rustfft::num_traits;
use rustfft::num_traits::{Float, FloatConst};

pub use cepstrum::CepstrumExtractor;
pub use conversions::{ComplexToReal, RealToComplex};

mod fft;
mod cepstrum;
pub mod windows;
mod conversions;

/// Trait implemented for types that can be used with the cepstrum extractor. Currently, it supports
/// `f32` and `f64`.
pub trait CepFloat: FftNum + Float + FloatConst {}

impl CepFloat for f32 {}
impl CepFloat for f64 {}
