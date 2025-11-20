use std::env;

fn main() {
    use std::fs;
    use std::ops::Range;
    use std::str::FromStr;

    use cepstrum_extractor::num_complex::Complex;
    use minifb::{Key, Window, WindowOptions};
    use plotters::backend::{BGRXPixel, BitMapBackend};
    use plotters::drawing::IntoDrawingArea;
    use plotters::prelude::{ChartBuilder, IntoFont, LineSeries, BLACK, GREEN};

    use cepstrum_extractor::{CepstrumExtractor, RealToComplex};

    /// Helper function to plot data; feel free to ignore it.
    fn draw_cepstrum(cepstrum: &[Complex<f32>]) {
        const W: usize = 800;
        const H: usize = 600;

        let mut buf = vec![0u32; W * H];

        let mut window = Window::new(
            &"Example - Press <ESC> to close",
            W,
            H,
            WindowOptions::default(),
        )
        .unwrap();

        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            unsafe {
                core::slice::from_raw_parts_mut(
                    buf.as_mut_ptr() as *mut _,
                    buf.len() * core::mem::size_of::<u32>(),
                )
            },
            (W as u32, H as u32),
        )
        .unwrap()
        .into_drawing_area();
        root.fill(&BLACK).unwrap();

        // Find max
        let max = cepstrum
            .iter()
            .enumerate()
            .skip(1)
            .max_by(|(_, x), (_, y)| x.re.total_cmp(&y.re))
            .unwrap();
        // Find min
        let min = cepstrum
            .iter()
            .enumerate()
            .skip(1)
            .min_by(|(_, x), (_, y)| x.re.total_cmp(&y.re))
            .unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .set_all_label_area_size(30)
            .build_cartesian_2d(0..cepstrum.len() - 1, min.1.re - 1. ..max.1.re + 1.)
            .unwrap();

        chart
            .configure_mesh()
            .label_style(("sans-serif", 15).into_font().color(&GREEN))
            .axis_style(&GREEN)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                cepstrum.iter().enumerate().map(|(i, x)| (i, x.re)),
                GREEN,
            ))
            .expect("error drawing series");

        root.present().unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buf, W, H).unwrap();
        }
    }

    const SHIFT: usize = 100;
    const WIN_LEN: usize = 512;
    const WIN_START: usize = 1000;
    const WIN_RANGE: Range<usize> = WIN_START..WIN_START + WIN_LEN;

    let mut signal: Vec<f32> = vec![];

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Load a file of 1_000_000 samples into signal.
    for line in fs::read_to_string(format!("{crate_dir}/assets/white_noise.txt"))
        .unwrap()
        .lines()
    {
        signal.push(f32::from_str(line).unwrap());
    }

    // Sum signal to itself shifted by SHIFT samples
    let signal: Vec<f32> = signal
        .iter()
        .zip([0.; SHIFT].iter().chain(signal.iter()))
        .map(|(x, y)| *x + *y)
        .collect();

    // Create an instance of the extractor long WIN_LEN.
    let extractor = CepstrumExtractor::new(WIN_LEN);

    // Convert signal from real to complex.
    let mut cepstrum = signal[WIN_RANGE].to_complex_vec();
    // Extract cepstrum and place it within out.
    extractor.rceps_mut(&mut cepstrum);

    // There's a peak at SHIFT.
    draw_cepstrum(&cepstrum[0..cepstrum.len() / 2]);
}
