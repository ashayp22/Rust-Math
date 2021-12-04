use eframe::{egui, epi};
use egui::{containers::*, widgets::*, *};
use std::mem::swap;

// use std::time::{ Instant };
#[derive(PartialEq)]

pub struct HTree {
    paused: bool,
    time: f64,
    zoom: f32,
    start_line_width: f32,
    depth: usize,
    length_factor: f32,
    luminance_factor: f32,
    width_factor: f32,
    line_count: usize,

    n: f32,
    last_n: f32,
    shapes: Vec<Shape>,
    _branch_angle: f32,
    r: u8,
    g: u8,
    b: u8,
}

impl Default for HTree {
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

            n: 1.0,
            last_n: 1.0,
            shapes: Vec::new(),
            _branch_angle: 0.26,
            r: 255,
            g: 0,
            b: 0,
        }
    }
}

impl epi::App for HTree {
    fn name(&self) -> &str {
        "H Tree"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl HTree {
    pub fn ui(&mut self, ui: &mut Ui) {
        //Don't recalculate if we have the same n
        if (self.last_n - self.n).abs() < 0.01 {
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
                CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });

        self.last_n = self.n;
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        //sliders for all the input values
        ui.add(Slider::new(&mut self.n, 1.0..=3.0).text("N"));

        ui.add(Slider::new(&mut self._branch_angle, 0.01..=1.0).text("Branch angle"));
        ui.add(Slider::new(&mut self.r, 0..=255).text("r"));

        ui.add(Slider::new(&mut self.g, 0..=255).text("g"));
        ui.add(Slider::new(&mut self.b, 0..=255).text("b"));

        egui::reset_button(ui, self);
    }

    fn drawtree(
        &mut self,
        length: f32,
        x1: f32,
        y1: f32,
        angle: f32,
        painter: &Painter,
        // start: Instant,
    ) {
        // each branch is 87% of the length as previous
        let _scaling_factor = 0.87;
        //min length based on input slider
        let _min_branch_length = 20.0 * (4.0 - self.n);

        // switches to opposite color if it is the leaf node
        let mut curr_r = self.r;
        let mut curr_g = self.g;
        let mut curr_b = self.b;
        if length <= _min_branch_length {
            curr_r = 255 - curr_r;
            curr_g = 255 - curr_g;
            curr_b = 255 - curr_b;
        }

        //calculate tip x,y for left and right trees
        let xr = x1 + ((angle - self._branch_angle).cos() * length);
        let yr = y1 - ((angle - self._branch_angle).sin() * length);
        let xl = x1 + ((angle + self._branch_angle).cos() * length);
        let yl = y1 - ((angle + self._branch_angle).sin() * length);
        let p1 = pos2(x1, y1);
        let p2 = pos2(xr, yr);
        let p3 = pos2(xl, yl);
        // draws two branches
        self.paint_line(
            [p1, p2],
            Color32::from_rgb(curr_r, curr_g, curr_b),
            0.5,
            painter,
        );
        self.paint_line(
            [p1, p3],
            Color32::from_rgb(curr_r, curr_g, curr_b),
            0.5,
            painter,
        );
        if length > _min_branch_length {
            //change color of leaves
            //recursive call to draw next two subtrees
            self.drawtree(
                length * _scaling_factor,
                xr,
                yr,
                angle - self._branch_angle,
                painter,
                // start,
            );

            self.drawtree(
                length * _scaling_factor,
                xl,
                yl,
                angle + self._branch_angle,
                painter,
                // start,
            );
        } else {
            //benchmarks that can be printed
            // let _end = start.elapsed();
            // println!("{} seconds for single thread.",end.as_secs());
        }
    }

    fn paint_line(&mut self, points: [Pos2; 2], color: Color32, width: f32, _painter: &Painter) {
        let line = [points[0], points[1]];

        self.shapes.push(Shape::line_segment(line, (width, color)));
    }

    fn paint(&mut self, painter: &Painter) {
        self.shapes = Vec::new();

        let rect = painter.clip_rect();
        // let start = Instant::now();

        //initial starting parameters for the tree
        self.drawtree(
            100.0,
            rect.width() / 2.0,
            2.0 * rect.height() / 3.0,
            //90 degrees in radians so rotation is correct
            1.5708,
            painter,
            // start,
        );

        let mut x: std::vec::Vec<Shape> = Vec::new();
        swap(&mut x, &mut self.shapes);
        painter.extend(x);
    }
}
