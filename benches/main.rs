#![cfg(bench)]

use cepstrum_extractor::{CepstrumExtractor, RealToComplex};

const BUF_LEN: usize = 4096;
const WIN_LEN: usize = 128;

fn main() {
    divan::main();
}

fn prepare() -> (Vec<f32>, CepstrumExtractor<f32>) {
    (
        vec![0f32; BUF_LEN],
        CepstrumExtractor::new(WIN_LEN)
    )
}

#[divan::bench(sample_size = 1000)]
fn rceps_mut(b: divan::Bencher) {
    let (buf, extractor) = prepare();
    b.bench_local(|| {
        extractor.rceps_mut(&mut buf.to_complex_vec());
    });
}

#[divan::bench(sample_size = 1000)]
fn rceps(b: divan::Bencher) {
    let (buf, extractor) = prepare();
    b.bench_local(|| {
        divan::black_box(extractor.rceps_to_vec(&buf.to_complex_vec()));
    });
}

#[divan::bench(sample_size = 1000)]
fn cceps_mut(b: divan::Bencher) {
    let (buf, extractor) = prepare();
    b.bench_local(|| {
        extractor.cceps_mut(&mut buf.to_complex_vec());
    });
}

#[divan::bench(sample_size = 1000)]
fn cceps(b: divan::Bencher) {
    let (buf, extractor) = prepare();
    b.bench_local(|| {
        divan::black_box(extractor.cceps_to_vec(&buf.to_complex_vec()));
    });
}