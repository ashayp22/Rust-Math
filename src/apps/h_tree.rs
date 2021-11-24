use eframe::{egui, epi};
use std::mem::swap;
use egui::{containers::*, widgets::*, *};
extern crate time;
use time::PreciseTime;
#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
    b: u8
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
            shapes:Vec::new(),
            _branch_angle: 0.26,
            r:255,
            g:0,
            b:0
        }
    }
}

impl epi::App for HTree {
    fn name(&self) -> &str {
        "Htree"
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
        ui.add(Slider::new(&mut self.n, 1.0..=3.0).text("N"));
        ui.add(Slider::new(&mut self._branch_angle, 0.01..=1.0).text("Branch angle"));
        ui.add(Slider::new(&mut self.r, 0..=255).text("r"));

        ui.add(Slider::new(&mut self.g, 0..=255).text("g"));
        ui.add(Slider::new(&mut self.b, 0..=255).text("b"));


        
        // ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        egui::reset_button(ui, self);
    }
    
    fn drawtree( &mut self, length: f32, x1: f32, y1: f32, angle: f32,painter: &Painter, start:PreciseTime ) {
                   
            
        let _scaling_factor = 0.87;
   // let _branch_angle = 0.26; //0.26;
    let _min_branch_length = 20.0*(4.0-self.n);
    let xr = x1 + ((angle - self._branch_angle).cos() * length);
    let yr = y1 - ((angle - self._branch_angle).sin() * length);
    let xl = x1 + ((angle + self._branch_angle).cos() * length);
    let yl = y1 - ((angle + self._branch_angle).sin() * length);
    let p1 = pos2(x1, y1);
        let p2 = pos2(xr, yr);
        let p3 = pos2(xl, yl);
      //     print!("{},{}" ,p2.x,p2.y);
        self.paint_line([p1, p2], Color32::from_rgb(self.r, self.g, self.b), 0.5, painter );
        self.paint_line([p1, p3], Color32::from_rgb(self.r, self.g, self.b), 0.5, painter);
         if length > _min_branch_length{
                self.drawtree(
                                    length * _scaling_factor,
                                    xr,
                                    yr,
                                    angle - self._branch_angle,
                                    painter,
                                    start
                                );
                                
                                self.drawtree(
                                    length * _scaling_factor,
                                    xl,
                                    yl,
                                    angle +self. _branch_angle,
                                    painter, 
                                    start
                                ); }
         else{
            let end = PreciseTime::now();
            println!("{} seconds for single thread.", start.to(end));
         }

       
    }


    fn paint_line (& mut self, points: [Pos2; 2], color: Color32, width: f32, _painter: &Painter) {
        let line = [points[0], points[1]];

        self.shapes.push(Shape::line_segment(line, (width, color)));
    }

    fn paint(&mut self, painter: &Painter) {
        
        self.shapes= Vec::new();

        let rect = painter.clip_rect();
        let start = PreciseTime::now();


        //length: f32, x1: f32, y1: f32, angle: f32, to_screen: &emath::RectTransform, shapes: &mut Vec<Shape>
        self.drawtree(100.0, rect.width() / 2.0, rect.height() , 1.5708,painter, start);

       
        let mut x : std::vec::Vec<Shape> = Vec::new();
        swap(&mut x, &mut self.shapes);
    //    for c in self.shapes{

    //    }
        
        painter.extend( x);
    }
}
