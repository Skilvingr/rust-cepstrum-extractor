use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::{env, fs, thread};
use cepstrum_extractor::{CepstrumExtractor, RealToComplex};

const SHIFT: usize = 20;
const WIN_LEN: usize = 128;
const THREADS: usize = 16;

fn main() {
    let mut signal: Vec<f32> = vec![];

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Load a file of 1_000_000 samples into signal.
    for line in fs::read_to_string(format!("{crate_dir}/assets/white_noise.txt")).unwrap().lines().take(1000) {
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

    // `out` now contains the cepstrums of the 128 samples long blocks which compose the signal.
}