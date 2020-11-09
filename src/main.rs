mod mandelbrot;

use mandelbrot::{Frame, ZoomLocation};
use std::time::{Duration, Instant};

use angular_units::Deg;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use prisma::{Color, FromColor, Hsl, Rgb};

use graphics::*;
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    zl: ZoomLocation,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let frame_start = Instant::now();
        let frame = mandelbrot::generate_frame(&self.zl, &args.window_size);
        let frame_duration = frame_start.elapsed();
        println!("Frame generated in {:?}", frame_duration);

        let draw_start = Instant::now();
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 0.0], gl);
            render_frame(frame, c, gl);
        });
        let draw_duration = draw_start.elapsed();
        println!("Frame render in {:?}", draw_duration);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.zl.zoom *= 2.0;
    }
}

fn render_frame(f: Frame, c: Context, gl: &mut GlGraphics) {
    for (row, step_fractions) in f.step_fractions.iter().enumerate() {
        for (col, step_fraction) in step_fractions.iter().enumerate() {
            if *step_fraction == 0.0 {
                continue;
            }

            let color = get_step_fraction_color(step_fraction);

            draw_pixel(row as f64, col as f64, color, c, gl);
        }
    }
}

fn get_step_fraction_color(step_fraction: &f64) -> [f32; 4] {
    let deg = 360.0 - ((360.0 * step_fraction) % 359.0);
    let hsl = Hsl::new(Deg(deg), 0.8, 0.8);
    let (r, g, b) = Rgb::from_color(&hsl).to_tuple();
    let color = [r, g, b, 1.0];
    color
}

fn draw_pixel(x: f64, y: f64, color: [f32; 4], c: Context, gl: &mut GlGraphics) {
    // Clear the screen.

    let transform = c.transform.trans(x, y);

    // Draw a box rotating around the middle of the screen.
    let pixel = rectangle::square(0.0, 0.0, 1.0);
    rectangle(color, pixel, transform, gl);
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("mandelbrot", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let zl = ZoomLocation {
        re: -1.74999841099374081749002,
        im: -0.00000000000000165712469,
        zoom: 100.0,
        iterations: 4196,
    };
    let gl = GlGraphics::new(opengl);
    let mut app = App { gl, zl };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
