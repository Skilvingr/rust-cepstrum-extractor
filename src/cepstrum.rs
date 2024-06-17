use crate::CepFloat;
use crate::fft::CepFft;
use crate::num_complex::{Complex, ComplexFloat};

/// The main struct of this crate; can be used to extract both complex and real cepstrums out of a signal.
///
/// As far as possible, when used multiple times, this struct will try to re-use internal data.
///
/// ## Examples
/// ```rust
/// use cepstrum_extractor::num_complex::Complex;
/// use cepstrum_extractor::num_traits::Zero;
/// use cepstrum_extractor::{CepstrumExtractor, Hann, RealToComplex};
///
/// let extractor = CepstrumExtractor::new(10);
///
/// // Different ways to obtain a vector of complex
/// let signal: Vec<Complex<f32>> = vec![Complex::zero(); 10];
/// let signal: Vec<Complex<f32>> = [0.; 10].to_complex_vec();
/// let signal: Vec<Complex<f32>> = [0.; 10].apply_hann_window_complex();
///
/// // Create new vectors of len `signal.len() / 2`
/// let real_ceps = extractor.rceps_to_vec(&signal);
/// let complex_ceps = extractor.cceps_to_vec(&signal);
///
/// // Use passed slices (useful range will be `0..len/2`)
/// let mut real_ceps = signal.clone();
/// extractor.rceps_mut(&mut real_ceps);
/// real_ceps.truncate(real_ceps.len() / 2);
///
/// let mut complex_ceps = signal.clone();
/// extractor.rceps_mut(&mut complex_ceps);
/// complex_ceps.truncate(complex_ceps.len() / 2);
/// ```
///
/// # Use in a concurrent environment
/// This extractor can be put in an Arc to be shared between different threads.
/// So one could use plain threads or external tools like tokio and rayon to perform parallel
/// computations. It is important to note, however, that each thread must always use the same
/// index, to make sure that the same part of the extractor is not
/// being used by another thread, in a certain moment.
/// `*_with_instance_*` methods require a `usize` parameter, representing the index of the thread
/// which is calling the method.
///
/// As it is built, the extractor has only one instance available, so it can be used only by one thread
/// at a time.
/// Calling [`Self::extend_instances`], one can increase the number of instances.
///
/// So, if one creates an extractor with 10 available instances, threads indices are numbered from 0
/// to 9.
///
/// ## Examples
/// ```rust
///
/// use std::sync::Arc;
/// use std::thread;
/// use cepstrum_extractor::num_complex::Complex;
/// use cepstrum_extractor::num_traits::Zero;
/// use cepstrum_extractor::CepstrumExtractor;
///
/// const THREADS: usize = 2;
/// const CEP_LEN: usize = 10;
///
/// let extractor: Arc<CepstrumExtractor<f32>> = Arc::new(CepstrumExtractor::new(CEP_LEN));
/// extractor.extend_instances(THREADS);
///
/// let signal: Vec<Complex<f32>> = vec![Complex::zero(); 100];
///
/// thread::scope(|s| {
///     for (idx, thread_chunk) in signal.chunks(signal.len() / THREADS).enumerate() {
///         let ex = extractor.clone();
///         s.spawn(move || {
///             for chunk in thread_chunk.chunks(CEP_LEN) {
///                 let complex_ceps = ex.cceps_with_instance_to_vec(chunk, idx);
///                 let real_ceps = ex.rceps_with_instance_to_vec(chunk, idx);
///             }
///         });
///     }
/// });
/// ```
pub struct CepstrumExtractor<T: CepFloat> {
    fft_instance: CepFft<T>,
}

impl<T: CepFloat> CepstrumExtractor<T> {
    fn _ceps_with_instance_mut(&self, mut signal: &mut [Complex<T>], f: fn(&Complex<T>) -> Complex<T>, instance: usize) {
        self.fft_instance.do_fft(&mut signal, instance);

        signal.iter_mut().for_each(|fft_component| {
            *fft_component = f(fft_component);
        });

        self.fft_instance.do_ifft(&mut signal, instance);
    }


    /// Builds a new extractor with a single instance available, i.e. an extractor to be used in a
    /// single-threaded environment.
    pub fn new(win_len: usize) -> CepstrumExtractor<T> {
        Self {
            fft_instance: CepFft::new(win_len)
        }
    }

    /// Sets the length of the window to `len`.
    pub fn set_len(&mut self, len: usize) {
        self.fft_instance.set_len(len);
    }

    /// Increases the number of instances available for parallel computing to `new_count`.
    pub fn extend_instances(&self, new_count: usize) {
        self.fft_instance.extend_scratches(new_count);
    }


    // ----------------------------------------- REAL ----------------------------------------------


    /// Extract the real cepstrum mutating the provided slice.
    /// <div class="warning">
    ///
    /// As for spectrums, the meaningful area will be `signal[0..signal.len() / 2]`.
    /// </div>
    pub fn rceps_mut(&self, mut signal: &mut [Complex<T>]) {
        self.rceps_with_instance_mut(&mut signal, 0);
    }

    /// Extract the real cepstrum placing the result in a new vec.
    /// Such a vec will be already truncated to half `signal.len()`.
    pub fn rceps_to_vec(&self, signal: &[Complex<T>]) -> Vec<Complex<T>> {
        let mut copied = Vec::with_capacity(signal.len());
        copied.extend_from_slice(signal);

        self.rceps_with_instance_mut(&mut copied, 0);
        copied.truncate(copied.len() / 2);

        copied
    }

    #[inline]
    fn r_f(x: &Complex<T>) -> Complex<T> {
        Complex::from(if x.re == T::zero() { x.abs() } else { x.abs().ln() })
    }
    /// As [`Self::rceps_mut`], but uses the passed instance at index `instance`.
    ///
    /// <div class="warning">
    ///
    /// As for spectrums, the meaningful area will be `signal[0..signal.len() / 2]`.
    /// </div>
    pub fn rceps_with_instance_mut(&self, signal: &mut [Complex<T>], instance: usize) {
        self._ceps_with_instance_mut(signal, Self::r_f, instance)
    }

    /// As [`Self::rceps_to_vec`], but uses the passed instance at index `instance`.
    pub fn rceps_with_instance_to_vec(&self, signal: &[Complex<T>], instance: usize) -> Vec<Complex<T>> {
        let mut copied = Vec::with_capacity(signal.len());
        copied.extend_from_slice(signal);

        self.rceps_with_instance_mut(&mut copied, instance);
        copied.truncate(copied.len() / 2);

        copied
    }


    // --------------------------------------- COMPLEX ---------------------------------------------


    /// Extract the complex cepstrum mutating the provided slice.
    /// <div class="warning">
    ///
    /// As for spectrums, the meaningful area will be `signal[0..signal.len() / 2]`.
    /// </div>
    pub fn cceps_mut(&self, mut signal: &mut [Complex<T>]) {
        self.rceps_with_instance_mut(&mut signal, 0);
    }

    /// Extract the complex cepstrum placing the result in a new vec.
    /// Such a vec will be already truncated to half `signal.len()`.
    pub fn cceps_to_vec(&self, signal: &[Complex<T>]) -> Vec<Complex<T>> {
        let mut copied = Vec::with_capacity(signal.len());
        copied.extend_from_slice(signal);

        self.rceps_with_instance_mut(&mut copied, 0);
        copied.truncate(copied.len() / 2);

        copied
    }

    #[inline]
    fn c_f(x: &Complex<T>) -> Complex<T> {
        if x.re == T::zero() { *x } else { x.ln() }
    }
    /// As [`Self::cceps_mut`], but uses the passed instance at index `instance`.
    ///
    /// <div class="warning">
    ///
    /// As for spectrums, the meaningful area will be `signal[0..signal.len() / 2]`.
    /// </div>
    pub fn cceps_with_instance_mut(&self, signal: &mut [Complex<T>], instance: usize) {
        self._ceps_with_instance_mut(signal, Self::c_f, instance)
    }

    /// As [`Self::cceps_to_vec`], but uses the passed instance at index `instance`.
    pub fn cceps_with_instance_to_vec(&self, signal: &[Complex<T>], instance: usize) -> Vec<Complex<T>> {
        let mut copied = Vec::with_capacity(signal.len());
        copied.extend_from_slice(signal);

        self.rceps_with_instance_mut(&mut copied, instance);
        copied.truncate(copied.len() / 2);

        copied
    }
}