extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Event::Input, Input as InputTypes, Button, ButtonState, Key};
use piston::window::*;

mod document;
mod panel;
mod cursor;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    panels: Vec<panel::Panel>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const FONT_SIZE: u32 = 75;

        //let square = rectangle::square(0.0, 0.0, RECT_SIZE);
        //let rotation = self.rotation;

        // I don't know why fonts work this way, but from manual testing it appears
        // that a line is 4/3 * FONT_SIZE pixels tall and the first FONT_SIZE pixels
        // are the stuff that would go above the line when writing on ruled paper
        // and the rest of the pixels are for stuff below it (like for the bottom of j)
        let line_height = FONT_SIZE as f64 * (4.0 / 3.0);

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let ref mut glyphs = GlyphCache::new("assets/FiraCode-Regular.ttf", (), texture_settings).expect("Could not load font");
        let panel_iterator = self.panels.iter();
       
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for panel in panel_iterator{
                panel.draw(glyphs, c, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }

    fn add_text(&mut self, s: &str){
        let active_panel = self.panels.get_mut(0).unwrap();
        active_panel.add_text(s);
    }

    fn remove_text(&mut self, amount: usize){
        let active_panel = self.panels.get_mut(0).unwrap();
        active_panel.remove_text(amount);
    }
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
        panels: vec![panel::Panel::new()]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input(ref input_type, _) => {
                match input_type {
                    InputTypes::Text(string) => app.add_text(&string[..]),
                    InputTypes::Button(button_args) => {
                        match button_args.button {
                            Button::Keyboard(key) => {
                                if let ButtonState::Press = button_args.state {
                                    if key == Key::Backspace {
                                        app.remove_text(1);
                                        println!("Backspace");
                                    }
                                }
                            }
                            _ => ()
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}