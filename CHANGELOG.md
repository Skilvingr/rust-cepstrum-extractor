<a name="v0.2.0"></a>
## v0.2.0 (unreleased)
- `Hann` window has been fixed and moved to `windows::hann:Hann`.
- Added `Hamming` window.

<a name="v0.1.5"></a>
## v0.1.5 (04/04/2025)
- Add benchmarks.
- Use `Mutex` instead of `RwLock`: increases performance.

<a name="v0.1.4"></a>
## v0.1.4 (20/11/2024)
- Fix another bug in set_len.

<a name="v0.1.3"></a>
## v0.1.3 (30/10/2024)
- Fix crash after len change.

<a name="v0.1.2"></a>
## v0.1.2 (19/06/2024)

- Clean types and generics.
- Hann windows can now be applied to `[Complex]` as well as to `[f32]` and `[f64]`

<a name="v0.1.1"></a>
## v0.1.1 (17/06/2024)

Add a new method to set the length of the window for an existent extractor.