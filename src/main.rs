use std::time::Instant;

use angular_units::Deg;
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use prisma::{Color, FromColor, Hsl, Rgb};

use mandelbrot::{Frame, RgbFrame, ZoomLocation};
use piston::EventLoop;

mod mandelbrot;

enum RenderMode {
    Fast,
    Slow,
}

pub struct App {
    gl: GlGraphics,
    zl: ZoomLocation,
    render_mode: RenderMode,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        match self.render_mode {
            RenderMode::Fast => self.render_fast(args),
            RenderMode::Slow => self.render_slow(args),
        }
    }

    fn render_slow(&mut self, args: &RenderArgs) {
        let frame_start = Instant::now();
        let f = &mandelbrot::generate_frame(self.zl, &args.window_size);
        let frame_duration = frame_start.elapsed();
        println!("Frame generated in {:?}", frame_duration);

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0; 4], gl);
            render_frame(f, c, gl);
        });
    }

    fn render_fast(&mut self, args: &RenderArgs) {
        let frame_start = Instant::now();
        let f = &mandelbrot::generate_rgb_frame_parallel(self.zl, &args.window_size);
        let frame_duration = frame_start.elapsed();
        println!("Frame generated in {:?}", frame_duration);

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0; 4], gl);
            render_rgb_frame(f, c, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.zl.zoom *= 1.5;
    }
}

fn render_frame(f: &Frame, c: Context, gl: &mut GlGraphics) {
    for (row, step_fractions) in f.step_fractions.iter().enumerate() {
        for (col, step_fraction) in step_fractions.iter().enumerate() {
            if *step_fraction == 0.0 {
                continue;
            }

            let deg = 360.0 - ((360.0 * step_fraction) % 359.0);
            let hsl = Hsl::new(Deg(deg), 0.8, 0.8);
            let (r, g, b) = Rgb::from_color(&hsl).to_tuple();
            let color = [r, g, b, 1.0];

            draw_pixel(row as f64, col as f64, color, c, gl);
        }
    }
}

fn render_rgb_frame(f: &RgbFrame, c: Context, gl: &mut GlGraphics) {
    for (row, pixels) in f.pixels.iter().enumerate() {
        for (col, &color) in pixels.iter().enumerate() {
            match color {
                Some(cl) => draw_pixel(row as f64, col as f64, cl, c, gl),
                None => (),
            }
        }
    }
}

fn draw_pixel(x: f64, y: f64, color: [f32; 4], c: Context, gl: &mut GlGraphics) {
    let transform = c.transform.trans(x, y);
    let pixel = rectangle::square(0.0, 0.0, 1.0);
    rectangle(color, pixel, transform, gl);
}

fn main() {
    run_app(RenderMode::Slow);
    run_app(RenderMode::Fast);
}

fn run_app(render_mode: RenderMode) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("mandelbrot", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let zl = ZoomLocation {
        re: -1.74999841099374081749002,
        im: -0.00000000000000165712469,
        zoom: 100.0,
        iterations: 2048,
    };
    let gl = GlGraphics::new(opengl);
    let mut app = App {
        gl,
        zl,
        render_mode,
    };

    let settings = EventSettings::new().max_fps(1).ups(1);
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
