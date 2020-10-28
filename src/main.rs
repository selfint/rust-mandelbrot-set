use angular_units::Deg;
use num::complex::{Complex32, Complex64};
use piston_window::*;
use prisma::{Color, FromColor, Hsl, Rgb};

fn steps_in_mandelbrot(n: &Complex32, iterations: &u64) -> u64 {
    let mut temp = n.clone();
    for iteration in 0..*iterations {
        temp = temp.powu(2) + n;
        if temp.norm() > 2.0 {
            return iteration + 1;
        }
    }

    0
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let real_offset = -0.75;
    let imaginary_offset = 0.1;
    let iterations = 128;
    let mut zoom = 100.0;
    while let Some(event) = window.next() {
        zoom *= 2.0;
        window.draw_2d(&event, |context, graphics, _device| {
            const BLACK: [f32; 4] = [0.0; 4];
            clear(BLACK, graphics);
            for real in -200..200 {
                let real = real as f32;
                for imaginary in -200..200 {
                    let imaginary = imaginary as f32;
                    draw_pixel(
                        &real,
                        &imaginary,
                        &real_offset,
                        &imaginary_offset,
                        &iterations,
                        &zoom,
                        context,
                        graphics,
                    );
                }
            }
        });
    }
}

fn draw_pixel(
    real: &f32,
    imaginary: &f32,
    real_offset: &f32,
    imaginary_offset: &f32,
    iterations: &u64,
    zoom: &f32,
    context: Context,
    graphics: &mut G2d,
) {
    let steps = steps_in_mandelbrot(
        &Complex32::new(
            real / zoom + real_offset,
            imaginary / zoom + imaginary_offset,
        ),
        iterations,
    );
    let hsl = Hsl::new(
        Deg(360.0 * ((steps as f32) / (*iterations as f32)) % 360.0),
        0.8,
        0.8,
    );
    let (r, g, b) = Rgb::from_color(&hsl).to_tuple();
    let mut color = [r as f32, g as f32, b as f32, 1.0];
    let rect: [f64; 4] = [*real as f64 + 200.0, *imaginary as f64 + 200.0, 1.0, 1.0];
    if steps == 0 {
        color = [0.0; 4];
    }
    rectangle(color, rect, context.transform, graphics);
}
