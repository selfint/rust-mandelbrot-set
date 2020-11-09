mod mandelbrot;

use mandelbrot::{Frame, ZoomLocation};

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
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let window_size = (x, y);
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        let frame = mandelbrot::generate_frame(&self.zl, &window_size);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            render_frame(frame, c, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn render_frame(f: Frame, c: Context, gl: &mut GlGraphics) {
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
        iterations: 128,
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
