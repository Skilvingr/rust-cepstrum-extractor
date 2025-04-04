use criterion::{black_box, Bencher, Criterion};
use cepstrum_extractor::{CepstrumExtractor, RealToComplex};

const BUF_LEN: usize = 4096;
const WIN_LEN: usize = 128;

pub fn setup(c: &mut Criterion) {
    c.bench_function("rceps_mut", rceps_mut);
    c.bench_function("rceps", rceps);
    c.bench_function("cceps_mut", cceps_mut);
    c.bench_function("cceps", cceps);
}


fn rceps_mut(b: &mut Bencher) {
    let mut buf = vec![0f32; BUF_LEN].to_complex_vec();

    let extractor = CepstrumExtractor::new(WIN_LEN);
    b.iter(|| {
        extractor.rceps_mut(&mut buf);
    });
}

fn rceps(b: &mut Bencher) {
    let buf = vec![0f32; BUF_LEN].to_complex_vec();

    let extractor = CepstrumExtractor::new(WIN_LEN);
    b.iter(|| {
        black_box(extractor.rceps_to_vec(&buf));
    });
}

fn cceps_mut(b: &mut Bencher) {
    let mut buf = vec![0f32; BUF_LEN].to_complex_vec();

    let extractor = CepstrumExtractor::new(WIN_LEN);
    b.iter(|| {
        extractor.cceps_mut(&mut buf);
    });
}

fn cceps(b: &mut Bencher) {
    let buf = vec![0f32; BUF_LEN].to_complex_vec();

    let extractor = CepstrumExtractor::new(WIN_LEN);
    b.iter(|| {
        black_box(extractor.cceps_to_vec(&buf));
    });
}