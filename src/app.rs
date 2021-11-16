use eframe::{egui, epi};

use egui::{containers::*, widgets::*, *};

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct SierpinskiCarpet {
    paused: bool,
    time: f64,
    zoom: f32,
    start_line_width: f32,
    depth: usize,
    length_factor: f32,
    luminance_factor: f32,
    width_factor: f32,
    line_count: usize,
    n: usize,
    last_n: usize
}

impl Default for SierpinskiCarpet {
    fn default() -> Self {
        Self {
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0,
            n: 1,
            last_n: 1
        }
    }
}

impl epi::App for SierpinskiCarpet {
    fn name(&self) -> &str {
        "Sierpinski Carpet"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl SierpinskiCarpet {
    pub fn ui(&mut self, ui: &mut Ui) {

        //Don't recalculate if we have the same n
        if self.last_n == self.n {
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

        self.last_n = self.n;
        
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        // ui.checkbox(&mut self.paused, "Paused");
        ui.add(Slider::new(&mut self.n, 1..=15).text("N"));
        // ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        egui::reset_button(ui, self);
    }

    fn paint(&mut self, painter: &Painter) {
  
        let mut shapes: Vec<Shape> = Vec::new();

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );

        fn paint_rect(center: Pos2, size: Vec2, color: Color32, _to_screen: &emath::RectTransform, shapes: &mut Vec<Shape>) {
            let updated_center = center;
            shapes.push(Shape::rect_filled(Rect::from_center_size(updated_center, size), 0.0, color))
        }

        fn sierpinski_carpet(center_x: f32, center_y: f32, width: f32, height: f32, n: i64, to_screen: &emath::RectTransform, shapes: &mut Vec<Shape>) {
            
            //draw in middle square
            paint_rect(pos2(center_x, center_y), vec2(width / 3.0, height / 3.0), Color32::WHITE, to_screen, shapes);
            //recurse on 8 other squares 
            if n > 0 {
                for row in -1..=1 {
                    for col in -1..=1 {
                        if row != 0 || col != 0 {
                            let new_center_x = center_x + (row as f32) * width / 3.0;
                            let new_center_y = center_y + (col as f32) * height / 3.0;
                            sierpinski_carpet(new_center_x, new_center_y, width / 3.0, height / 3.0, n - 1, to_screen, shapes);
                        }
                    }
                }
            }
        }

        let num_levels = self.n as i64;

        sierpinski_carpet(rect.width() / 2.0, rect.height() / 2.0, 500.0, 500.0, num_levels, &to_screen, &mut shapes);        
        
        painter.extend(shapes);
    }
}