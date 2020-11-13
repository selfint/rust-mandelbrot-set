use angular_units::Deg;
use num::complex::Complex64;
use prisma::{Color, FromColor, Hsl, Rgb};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub struct Frame {
    pub step_fractions: Vec<Vec<f64>>,
}

pub struct RgbFrame {
    pub pixels: Vec<Vec<Option<[f32; 4]>>>,
}

#[derive(Clone, Copy)]
pub struct ZoomLocation {
    pub re: f64,
    pub im: f64,
    pub zoom: f64,
    pub iterations: u64,
}

fn step_fraction_in_mandelbrot(n: &Complex64, iterations: &u64) -> f64 {
    let mut temp = n.clone();
    for iteration in 0..*iterations {
        temp = temp.powu(2) + n;
        if temp.norm() > 2.0 {
            return ((iteration + 1) as f64) / (*iterations as f64);
        }
    }

    0.0
}

pub fn generate_frame(zl: ZoomLocation, window_size: &[f64; 2]) -> Frame {
    let mut step_fractions = vec![];
    let rows = (window_size[0] / 2.0) as i64;
    let columns = (window_size[1] / 2.0) as i64;

    for row in -rows..rows {
        let mut row_step_fractions = vec![];
        for col in -columns..columns {
            let re = zl.re + (row as f64) / zl.zoom;
            let im = zl.im + (col as f64) / zl.zoom;
            let n = Complex64::new(re, im);
            let step_fraction = step_fraction_in_mandelbrot(&n, &zl.iterations);
            row_step_fractions.push(step_fraction);
        }

        step_fractions.push(row_step_fractions);
    }

    Frame { step_fractions }
}

pub fn generate_rgb_frame_parallel(zl: ZoomLocation, window_size: &[f64; 2]) -> RgbFrame {
    let rows = (window_size[0] / 2.0) as i64;
    let columns = (window_size[1] / 2.0) as i64;
    let mut pixels = vec![];
    for _ in -rows..rows {
        let mut step_fraction_row = vec![];
        for _ in -columns..columns {
            step_fraction_row.push(None);
        }

        pixels.push(step_fraction_row);
    }

    calc_rgb_parallel(zl, rows, columns, &mut pixels);

    RgbFrame { pixels }
}

fn calc_rgb_parallel(
    zl: ZoomLocation,
    rows: i64,
    columns: i64,
    buffer: &mut Vec<Vec<Option<[f32; 4]>>>,
) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for row in -rows..rows {
        let tx = tx.clone();
        pool.execute(move || {
            for col in -columns..columns {
                let pixel_row = row + rows;
                let pixel_col = col + columns;
                let re = zl.re + (row as f64) / zl.zoom;
                let im = zl.im + (col as f64) / zl.zoom;
                let n = Complex64::new(re, im);
                let step_fraction = step_fraction_in_mandelbrot(&n, &zl.iterations);
                if step_fraction > 0.0 {
                    let deg = 359.0 - ((360.0 * step_fraction) % 359.0);
                    let hsl = Hsl::new(Deg(deg), 0.8, 0.8);
                    let (r, g, b) = Rgb::from_color(&hsl).to_tuple();

                    tx.send((pixel_row, pixel_col, Some([r, g, b, 1.0])))
                        .expect(&format!(
                            "Failed to send Some pixel at {} {}",
                            pixel_row, pixel_col
                        ));
                } else {
                    tx.send((pixel_row, pixel_col, None)).expect(&format!(
                        "Failed to send None pixel at {} {}",
                        pixel_row, pixel_col
                    ));
                }
            }
        });
    }

    // we can limit the processing time and stop receiving pixels
    // after X seconds to speed up rendering!
    for _ in 0..(rows * 2 * columns * 2) {
        let (row, col, step_fraction) = rx.recv().unwrap();

        buffer[row as usize][col as usize] = step_fraction;
    }
}
