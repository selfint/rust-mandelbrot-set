use angular_units::Deg;
use num::complex::Complex64;
use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;

pub struct Frame {
    pub step_fractions: Vec<Vec<f64>>,
}

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

pub fn generate_frame(zl: &ZoomLocation, window_size: &[f64; 2]) -> Frame {
    let mut step_fractions = vec![];
    let columns = window_size[0] as i64;
    let rows = window_size[1] as i64;

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

pub fn generate_frame_parallel(zl: &ZoomLocation, window_size: &[f64; 2]) -> Frame {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png")?;
}
