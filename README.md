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

For more info about the concept of cepstrum, [here](https://www.researchgate.net/profile/Samuel-Demir-2/post/Anyone-has-this-paper-quefrency-analysis-of-time-series-for-echoes-cepstrum-pseudo-autocovariance-cross-cepstrum-and-saphe-cracking/attachment/5f0493ca4ba4fb0001a4a3c5/AS%3A910684434989062%401594135497855/download/The+quefrency+analysis+of+time+series+for+echoes.pdf)'s
the original paper.


## Usage
Quite a simple crate: create a [`CepstrumExtractor`] with a given length and
use it to compute real or complex cepstrum of a signal.

The extractor accepts a slice of Complex as input, [`RealToComplex::to_complex_vec`]
creates a new vec of Complex starting from a slice of `f32` or `f64`.

Such slices also implement traits with windowing functions, at the moment only
one is available: [`Hann`].

## A note about the length of the results:
As for spectrums, only the first half of the result of a fft has meaningful
values. Cepstrums are computed with a fft, so here it's the same.

Methods that return a vec already truncate the result to half the input slice,
but `*_mut` methods, the ones which mutate the slice passed as input, clearly can't,
so pay attention to what you do when using these methods.

### Example:
Given a `CepstrumExtractor` with len equal to `128`, `rceps_mut` mutates the
input slice (long 128 samples as well), but only the first `64` samples of
the mutated slice actually represent the cepstrum.

## A note about multithreading:
This crate can also be used in a concurrent environment. Only one instance
of the extractor is needed, and that can be shared between the threads with
a simple `Arc`; more info about this are available in the relative docs page.

An example can be found within `example` folder, under the name `concurrent`.

## Tests and examples
Miri test can be found within `script`.

The following commands must be run starting from the root of the crate.

Tests can be run with:

```shell
cargo test
```

Benchmarks can be run with:
```shell
RUSTFLAGS="--cfg bench" cargo +nightly bench
```

Concurrent example can be run with:
```shell
RUSTFLAGS="--cfg examples" cargo run --example concurrent
```

Other examples can be run with:
```shell
RUSTFLAGS="--cfg examples" cargo run --example `example_name`
```
