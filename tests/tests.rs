use std::{fs, thread};
use std::ops::Range;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};

use rayon::iter::ParallelIterator;
use rayon::prelude::{IndexedParallelIterator, ParallelSlice};

use cepstrum_extractor::{CepstrumExtractor, RealToComplex};

const SHIFT: usize = 20;
const WIN_LEN: usize = 128;
const WIN_START: usize = 1000;
const WIN_RANGE: Range<usize> = WIN_START..WIN_START + WIN_LEN;

#[test]
fn test_single_thread() {
    let mut signal: Vec<f64> = vec![];

    // Load a file of 1_000_000 samples into signal.
    for line in fs::read_to_string("assets/white_noise.txt").unwrap().lines().take(2000) {
        signal.push(f64::from_str(line).unwrap());
    }

    // Sum signal to itself shifted by SHIFT samples
    let signal: Vec<f64> = signal.iter()
        .zip([0.; SHIFT].iter().chain(signal.iter()))
        .map(|(x, y)| { *x + *y })
        .collect();

    // Create an instance of the extractor long WIN_LEN.
    let extractor = CepstrumExtractor::new(WIN_LEN);

    // Convert signal from real to complex.
    let mut cepstrum = signal[WIN_RANGE].to_complex_vec();
    // Extract cepstrum and place it within out.
    extractor.rceps_mut(&mut cepstrum);

    assert_eq!(cepstrum[SHIFT].re, 27.60245425474882);
}

const THREADS: usize = 5;

#[test]
fn test_multi_thread() {
    let mut signal: Vec<f32> = vec![];

    // Load a file of 1_000_000 samples into signal.
    for line in fs::read_to_string("assets/white_noise.txt").unwrap().lines().take(1_000) {
        signal.push(f32::from_str(line).unwrap());
    }

    // Sum signal to itself shifted by SHIFT samples
    let signal: Vec<f32> = signal.iter()
        .zip([0.; SHIFT].iter().chain(signal.iter()))
        .map(|(x, y)| { *x + *y })
        .collect();

    // Create an instance of the extractor long WIN_LEN.
    let extractor: Arc<CepstrumExtractor<f32>> = Arc::new(CepstrumExtractor::new(WIN_LEN));
    extractor.extend_instances(THREADS);

    let chunks_per_thread = (signal.len() as f32 / WIN_LEN as f32 / THREADS as f32).ceil() as usize;

    let chunks = signal.chunks(WIN_LEN * chunks_per_thread);

    let out = Arc::new(RwLock::new(vec![vec![]; signal.len() / WIN_LEN]));

    thread::scope(|s| {
        chunks
            .enumerate()
            .for_each(|(i, chunks)| {
                let extractor = extractor.clone();
                let out = out.clone();

                s.spawn(move || {
                    chunks
                        .chunks_exact(WIN_LEN)
                        .enumerate()
                        .for_each(|(j, chunk)| {
                            let res = extractor
                                .rceps_with_instance_to_vec(&chunk.to_complex_vec(), i);
                            let _ = out.write().map(|mut l| {
                                l[(i * chunks_per_thread) + j] = res;
                            });
                        });
                });
            });
    });

    signal
        .chunks_exact(WIN_LEN)
        .enumerate()
        .for_each(|(i, chunk)| {
            let mut conv = chunk.to_complex_vec();
            extractor.rceps_mut(&mut conv);
            assert_eq!(
                conv[..conv.len() / 2],
                out.read().unwrap()[i]
            );
        });
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_multi_thread_rayon() {
    let mut signal: Vec<f32> = vec![];

    // Load a file of 1_000_000 samples into signal.
    for line in fs::read_to_string("assets/white_noise.txt").unwrap().lines().take(1_000) {
        signal.push(f32::from_str(line).unwrap());
    }

    // Sum signal to itself shifted by SHIFT samples
    let signal: Vec<f32> = signal.iter()
        .zip([0.; SHIFT].iter().chain(signal.iter()))
        .map(|(x, y)| { *x + *y })
        .collect();

    // Create an instance of the extractor long WIN_LEN.
    let extractor: Arc<CepstrumExtractor<f32>> = Arc::new(CepstrumExtractor::new(WIN_LEN));
    extractor.extend_instances(THREADS);

    rayon::ThreadPoolBuilder::new().num_threads(THREADS).build_global().unwrap();

    let chunks = signal.par_chunks_exact(WIN_LEN);

    let out = Mutex::new(vec![vec![]; chunks.len()]);

    chunks
        .enumerate()
        .for_each(|(i, chunk)| {
            let res = extractor.clone()
                .rceps_with_instance_to_vec(&chunk.to_complex_vec(), rayon::current_thread_index().unwrap());
            let _ = out.lock().map(|mut l| {
                l[i] = res;
            });
        });

    let out = out.into_inner().unwrap();

    signal
        .chunks_exact(WIN_LEN)
        .enumerate()
        .for_each(|(i, chunk)| {
            let mut conv = chunk.to_complex_vec();
            extractor.rceps_mut(&mut conv);
            assert_eq!(
                conv[..conv.len() / 2],
                out[i]
            );
        });
}
