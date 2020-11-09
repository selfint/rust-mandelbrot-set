mod mandelbrot;

use mandelbrot::Frame;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use graphics::*;
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
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
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

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
