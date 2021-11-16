use eframe::{egui, epi};
use egui::{containers::*, widgets::*, *};

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FractalClock {
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

impl Default for FractalClock {
    fn default() -> Self {
        Self {
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 0.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0,
            n: 10,
            last_n: 1
        }
    }
}

impl epi::App for FractalClock {
    fn name(&self) -> &str {
        "🕑 Fractal Clock"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl FractalClock {
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
        let _golden_ratio:f64 = ( 1.0_f64 + 5.0_f64.sqrt() ) / 2.0_f64;

        // struct Dash {
        //     start: Pos2,
        //     end: Pos2,
        //     dir: Vec2,
        // }
        
        //rendering with respect to screen's parameter
        let mut shapes: Vec<Shape> = Vec::new();
        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );
        
        //paint lines
        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let line = [to_screen * points[0], to_screen * points[1]];
            shapes.push(Shape::line_segment(line, (width, color)));
        };
    
        let mut s0 = String::from("0");
        let mut s1 = String::from("01");
        
        for _i in 2..self.n {
            let tmp = String::from(s1.as_str());
            s1.push_str(&s0);
            s0 = tmp;
            
        }

        let mut curr_pts = pos2(0.0, 0.0);
        let mut curr_dir = Vec2{x: 1.0, y: 0.0};
        for (i, c) in s1.chars().enumerate() {
            let curr_end = curr_pts + curr_dir;
            paint_line([curr_pts, curr_end], Color32::WHITE, self.start_line_width);
            curr_pts = curr_end;
            if c == '0' {
                if i % 2 == 0 {
                    curr_dir = Vec2{x: curr_dir.y, y: curr_dir.x};
                } else {
                    curr_dir = Vec2{x: -curr_dir.y, y: -curr_dir.x};
                }
            }
        }
        painter.extend(shapes);
        // let fibonacci_word_fractal = |golden_ratio: f64| {
            
        //     let num_dash = self.n;
        //     for curr_x in 1..num_dash {
        //         let curr_y:i64 = (golden_ratio - 1.0_f64) * (curr_x as f64).floor();
        //         let prev_y:i64 = (golden_ratio - 1.0_f64) * (curr_x as f64 - 1.0_f64);
                
        //     }
        // };
    }
}