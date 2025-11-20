# Cepstrum Extractor

[![crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Rust + Miri][tests-badge]][tests-url]

[crates-badge]: https://img.shields.io/crates/v/cepstrum-extractor.svg
[crates-url]: https://crates.io/crates/cepstrum-extractor
[docs-badge]: https://docs.rs/cepstrum-extractor/badge.svg
[docs-url]: https://docs.rs/cepstrum-extractor
[tests-badge]: https://github.com/Skilvingr/rust-cepstrum-extractor/actions/workflows/rust.yml/badge.svg
[tests-url]: https://github.com/Skilvingr/rust-cepstrum-extractor/actions/workflows/rust.yml

An easy-to-use crate to compute the cepstrum of a signal.

For more information about the concept of cepstrum, refer to
[this original paper](https://www.researchgate.net/profile/Samuel-Demir-2/post/Anyone-has-this-paper-quefrency-analysis-of-time-series-for-echoes-cepstrum-pseudo-autocovariance-cross-cepstrum-and-saphe-cracking/attachment/5f0493ca4ba4fb0001a4a3c5/AS%3A910684434989062%401594135497855/download/The+quefrency+analysis+of+time+series+for+echoes.pdf).

## Usage
This crate is quite simple to use: create a [`CepstrumExtractor`] with a specified length and use it to compute the real or complex cepstrum of a signal.

The extractor accepts a slice of `Complex` as input. The method [`RealToComplex::to_complex_vec`] creates a new vector of `Complex` starting from a slice of `f32` or `f64`.

These slices also implement windowing functions; more information can be found in the related module [`windows`].

## A Note About the Length of the Results
As with spectrums, only the first half of the result of an FFT has meaningful values. Cepstrums are computed using an FFT, so the same applies here.

Methods that return a vector already truncate the result to half the input slice. However, `*_mut` methods, which mutate the slice passed as input, cannot do this, so please pay attention when using these methods.

### Example
Given a `CepstrumExtractor` with a length equal to `128`, the `rceps_mut` method mutates the input slice (which should also be 128 samples long), but only the first `64` samples of the mutated slice actually represent the cepstrum.

## A Note About Multithreading
This crate can also be used in a concurrent environment. Only one instance of the extractor is needed, and it can be shared between threads using a simple `Arc`. More information about this can be found in the relevant documentation page.

An example can be found in the `example` folder, under the name `concurrent`.

## Tests and Examples
Miri tests can be found in the `scripts` directory.

The following commands must be run from the root of the crate.

Tests can be run with:

```shell
cargo test
```

Benchmarks can be run with:
```shell
cargo bench
```

The concurrent example can be run with:
```shell
cargo run --example concurrent
```

Other examples can be run with:
```shell
cargo run --example `example_name`
```
