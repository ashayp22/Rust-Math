use eframe::{egui, epi};
use egui::{containers::*, widgets::*, *};
use num::complex::Complex;

#[derive(PartialEq)]

/*

    Sources: 
    https://levelup.gitconnected.com/mandelbrot-set-with-python-983e9fc47f56
    https://mathigon.org/course/fractals/mandelbrot
*/

pub struct Mandelbrot {
    zoom: f32,                     // The level of the recursion when generating the fractal
    last_zoom: f32,                // The depth of the last drawn fractal
    shapes: Vec<Shape>,            // A vector containing the shapes that will be painted on the screen,
    max_steps: u8,                // change this for more detail in the fractal
    last_max_steps: u8,
    num_pixels: f32,               // change this for a larger set
    last_num_pixels: f32,
    threshold: f32,                // no need to change this
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            last_zoom: 0.9,
            shapes: Vec::new(),
            max_steps: 80,
            last_max_steps: 100,
            num_pixels: 255.0,
            last_num_pixels: 255.0,
            threshold: 4.0,
        }
    }
}

impl epi::App for Mandelbrot {
    fn name(&self) -> &str {
        "Mandelbrot Set"
    }

    // Called every frame
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl Mandelbrot {

    // Paints the fractal
    pub fn ui(&mut self, ui: &mut Ui) {

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);

        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings")
                    .show(ui, |ui| self.options_ui(ui));
            });
    }

    // An options window for setting the zoom of the fractal generation
    fn options_ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.zoom, 0.0000001..=1.0).text("Zoom"));
        ui.add(Slider::new(&mut self.num_pixels, 100.0..=750.0).text("Size"));
        ui.add(Slider::new(&mut self.max_steps, 25..=255).text("Colors"));
        egui::reset_button(ui, self);
    }

    // Paints a rectangular given its center and size
    fn paint_rect(&mut self, center: Pos2, size: Vec2, color: Color32) {
        self.shapes.push(Shape::rect_filled(Rect::from_center_size(center, size), 0.0, color));
    }

    // Converts a hsv value, where 0 <= h, s, v <= 1
    fn hsv_to_rgb(&mut self, h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        
        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match (i as i32) % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            },
            1 => {
                r = q;
                g = v;
                b = p;
            },
            2 => {
                r = p;
                g = v;
                b = t;
            },
            3 => {
                r = p;
                g = q;
                b = v;
            },
            4 => {
                r = t;
                g = p;
                b = v;
            },
            5 => {
                r = v;
                g = p;
                b = q;
            },
            _ => println!("Something went wrong"),
        }

        let r_val = (r * 255.0).ceil();
        let g_val = (g * 255.0).ceil();
        let b_val = (b * 255.0).ceil();

        return (r_val as u8, g_val as u8, b_val as u8);
    }

    // Returns a color for a specific pixel given a distance
    fn calculate_color(&mut self, i: u8) -> Color32 {

        let hue = (359.0 * (i as f32)) / (self.max_steps as f32);
        let saturation = 1.0;
        let value = if i < self.max_steps { 1.0 } else { 0.0 };

        let color = self.hsv_to_rgb(hue, saturation, value);
        return Color32::from_rgb(color.0, color.1, color.2);
    }

    // Used to find the divergence of a complex number
    fn get_divergence(&mut self, c: Complex<f32>, threshold: f32, max_steps: u8) -> u8 {
        let mut z = c.clone();
        let mut i = 1;
        while i < max_steps && (z*z.conj()).re < threshold {
            z = z*z + c;
            i += 1;
        }
        return i;
    }

    // Generates the mandelbrot set
    fn plot_mandelbrot(&mut self, width_shift: f32, height_shift: f32) {

        // Handles the mandelbrot zoom
        let zoom_dif = 1.0 - self.zoom;

        // The x and y bounds that will be used to create complex numbers
        let x_min = -2.0 + zoom_dif;
        let x_max = 0.47 - zoom_dif;

        let y_min = -0.92 + zoom_dif;
        let y_max = 1.32 - zoom_dif;

        let mx = (x_max - x_min) / (self.num_pixels - 1.0);
        let my = (y_max - y_min) / (self.num_pixels - 1.0);

        for x in 0..(self.num_pixels as i64) {
            for y in 0..(self.num_pixels as i64) {
                let mapped_x = mx*(x as f32) + x_min;
                let mapped_y = my*(y as f32) + y_min;

                let complex_num = Complex::new(mapped_x, mapped_y);
                let it = self.get_divergence(complex_num, self.threshold, self.max_steps);

                // Applies shift to center the set on the screen
                let x_coord = (x as f32) + width_shift;
                let y_coord = (y as f32) + height_shift;

                let color = self.calculate_color(it);

                // Adds square with a special color
                self.paint_rect(pos2(x_coord, y_coord), vec2(0.5, 0.5), color);
            }
        }
    }


    fn paint(&mut self, painter: &Painter) {
  
        let rect = painter.clip_rect();

        let need_to_recalculate = self.zoom != self.last_zoom || self.max_steps != self.last_max_steps || self.num_pixels != self.last_num_pixels;

        // Shifts used for centering the fractal on the screen
        let width_shift = rect.width() / 2.0 - (self.num_pixels / 2.0);
        let height_shift = rect.height() / 2.0 - (self.num_pixels / 2.0);

        // Need to recalculate due to a change in one of the settings
        if need_to_recalculate {
            self.shapes.clear();
            self.plot_mandelbrot(width_shift, height_shift);
        }

        // Update the the markers in order to prevent further recalculations 
        self.last_zoom = self.zoom;
        self.last_max_steps = self.max_steps;
        self.last_num_pixels = self.num_pixels;

        // Copy over shapes since painter.extend doesn't take a reference
        let cloned_shapes : std::vec::Vec<Shape> = self.shapes.clone();

        // Draws the fractal
        painter.extend(cloned_shapes);
    }

}