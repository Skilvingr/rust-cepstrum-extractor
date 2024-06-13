use std::sync::Arc;
use std::thread;

use cepstrum_extractor::{CepstrumExtractor, Prepare};

const LEN: usize = 10;

#[test]
fn test_single_thread() {
    let cep_ex: CepstrumExtractor<f32> = CepstrumExtractor::new(LEN);

    let mut slice = (0..LEN).map(|x| x as f32).collect::<Vec<f32>>().as_slice().to_complex_slice();

    cep_ex.extract_cepstrum(&mut slice);
}

const THREADS: usize = 10;

#[test]
fn test_multi_thread() {
    let cep_ex: Arc<CepstrumExtractor<f32>> = Arc::new(CepstrumExtractor::new(LEN));

    cep_ex.prepare_instances(THREADS);

    for i in 0..10 {
        let clone = cep_ex.clone();
        let mut slice = (0..LEN).map(|x| x as f32).collect::<Vec<f32>>()
            .as_slice()
            .to_complex_slice();

        thread::spawn(move || {
            clone.extract_cepstrum_instance(&mut slice, i);
        });
    }
}