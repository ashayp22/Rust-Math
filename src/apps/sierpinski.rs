use eframe::{egui, epi};
use egui::{containers::*, widgets::*, *};

#[derive(PartialEq)]

/*
This file contains the code for recursively generating and then rendering the Sierpinski Carpet.
Fractal Explanation: https://en.wikipedia.org/wiki/Sierpi%C5%84ski_carpet
*/


pub struct SierpinskiCarpet {
    depth: usize, // The level of the recursion when generating the fractal
    last_depth: usize, // The depth of the last drawn fractal
}

impl Default for SierpinskiCarpet {
    fn default() -> Self {
        Self {
            depth: 1,
            last_depth: 1,
        }
    }
}

impl epi::App for SierpinskiCarpet {
    fn name(&self) -> &str {
        "Sierpinski Carpet"
    }

    // Called every frame
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl SierpinskiCarpet {

    // Paints the fractal
    pub fn ui(&mut self, ui: &mut Ui) {

        // If we have the same depth as the last frame, repaint the fractal.
        // This saves recalulating the fractal. 
        if self.last_depth == self.depth {
            ui.ctx().request_repaint();
        }

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

        self.last_depth = self.depth;
    }

    // An options window for setting the depth of the fractal generation
    fn options_ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.depth, 1..=7).text("Depth"));
        egui::reset_button(ui, self);
    }

    fn paint(&mut self, painter: &Painter) {
  
        let mut shapes: Vec<Shape> = Vec::new(); // A vector of shapes that will be drawn

        let rect = painter.clip_rect();

        // Paints a rectangular given its center and size
        fn paint_rect(center: Pos2, size: Vec2, color: Color32, shapes: &mut Vec<Shape>) {
            shapes.push(Shape::rect_filled(Rect::from_center_size(center, size), 0.0, color));
        }

        // A recursive function for creatng the sierpinski carpet
        fn sierpinski_carpet(center_x: f32, center_y: f32, width: f32, height: f32, n: i64, shapes: &mut Vec<Shape>) {
            
            // draw in middle square
            paint_rect(pos2(center_x, center_y), vec2(width / 3.0, height / 3.0), Color32::BLUE, shapes);
            // recurse on 8 other squares until you hit the base case n = 0
            if n > 0 {
                for row in -1..=1 {
                    for col in -1..=1 {
                        if row != 0 || col != 0 {
                            let new_center_x = center_x + (row as f32) * width / 3.0;
                            let new_center_y = center_y + (col as f32) * height / 3.0;
                            sierpinski_carpet(new_center_x, new_center_y, width / 3.0, height / 3.0, n - 1, shapes);
                        }
                    }
                }
            }
        }

        // Generates the fractal in the center of the screen
        let num_levels = self.depth as i64;
        sierpinski_carpet(rect.width() / 2.0, rect.height() / 2.0, 500.0, 500.0, num_levels, &mut shapes);        

        // Draws the fractal
        painter.extend(shapes);
    }
}