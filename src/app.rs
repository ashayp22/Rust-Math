use eframe::{egui, epi};

use egui::{containers::*, widgets::*, *};
use std::f32::consts::TAU;

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



// impl FractalClock {
//     pub fn ui(&mut self, ui: &mut Ui) {

       
//         if self.last_n == self.n {
//             ui.ctx().request_repaint();
//         }

//         let painter = Painter::new(
//             ui.ctx().clone(),
//             ui.layer_id(),
//             ui.available_rect_before_wrap(),
//         );
//         self.paint(&painter);
//         ui.expand_to_include_rect(painter.clip_rect());

//         Frame::popup(ui.style())
//             .stroke(Stroke::none())
//             .show(ui, |ui| {
//                 ui.set_max_width(270.0);
//                 CollapsingHeader::new("Settings")
//                     .show(ui, |ui| self.options_ui(ui));
//             });

//         self.last_n = self.n;
        
//     }

//     fn options_ui(&mut self, ui: &mut Ui) {

//         ui.add(Slider::new(&mut self.n, 1..=15).text("N"));
//         egui::reset_button(ui, self);
//     }

   

//     fn drawtree(&self, length: f32, x1: f32, y1: f32, angle: f32, painter: &Painter) {
//         let mut shapes: Vec<Shape> = Vec::new();
//         let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
//             let rect = painter.clip_rect();
//             let to_screen = emath::RectTransform::from_to(
//                 Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
//                 rect,
//             );
//             let line = [to_screen * points[0], to_screen * points[1]];

   
            
//                 shapes.push(Shape::line_segment(line, (width, color)));
            
//         };
//         let SCALING_FACTOR = 0.87;
//         let BRANCH_ANGLE = 0.26; //0.26;
//         let MIN_BRANCH_LENGTH = 6.0;
//         let xR = x1 + ((angle - BRANCH_ANGLE) * length).cos();
//         let yR = y1 - ((angle - BRANCH_ANGLE) * length).sin();
//         let xL = x1 + ((angle + BRANCH_ANGLE) * length).cos();
//         let yL = y1 - ((angle + BRANCH_ANGLE) * length).sin();
//         if length <= MIN_BRANCH_LENGTH {
//             let p1 = pos2(x1, y1);
//             let p2 = pos2(xR, yR);
//             let p3 = pos2(xL, yL);
               
//             paint_line([p1, p2], Color32::from_rgb(255, 0, 0), 2.5);
//             paint_line([p1, p3], Color32::from_rgb(255, 0, 0), 2.5);

//             return;
//         } else {
//             print!("{}",length);
//             let p1 = pos2(x1, y1);
//             let p2 = pos2(xR, yR);
//             let p3 = pos2(xL, yL);

//             paint_line([p1, p2], Color32::from_rgb(255, 0, 0), 2.5);
//             paint_line([p1, p3], Color32::from_rgb(255, 0, 0), 2.5);

//             painter.extend(shapes);
//             self.drawtree(
//                 length * SCALING_FACTOR,
//                 xR,
//                 yR,
//                 angle - BRANCH_ANGLE,
//                 painter,
//             );
            
//             self.drawtree(
//                 length * SCALING_FACTOR,
//                 xL,
//                 yL,
//                 angle + BRANCH_ANGLE,
//                 painter,
//             );
//         }
//     }

//     fn paint(&mut self, painter: &Painter) {
//         self.drawtree(25.0, 15.0, 15.0, 0.0, painter);

       
//     }
// }







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
  
        let mut shapes: Vec<Shape> = Vec::new();

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );

        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
                        let rect = painter.clip_rect();
                        let to_screen = emath::RectTransform::from_to(
                            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
                            rect,
                        );
                        let line = [to_screen * points[0], to_screen * points[1]];
            
               
                        
                            shapes.push(Shape::line_segment(line, (width, color)));
                        
                    };

        let mut drawtree=|  length: f32, x1: f32, y1: f32, angle: f32| {
                   
            
            let SCALING_FACTOR = 0.87;
        let BRANCH_ANGLE = 0.26; //0.26;
        let MIN_BRANCH_LENGTH = 6.0;
        let xR = x1 + ((angle - BRANCH_ANGLE).cos() * length);
        let yR = y1 - ((angle - BRANCH_ANGLE).sin() * length);
        let xL = x1 + ((angle + BRANCH_ANGLE).cos() * length);
        let yL = y1 - ((angle + BRANCH_ANGLE).sin() * length);
        let p1 = pos2(x1, y1);
            let p2 = pos2(xR, yR);
            let p3 = pos2(xL, yL);
          //     print!("{},{}" ,p2.x,p2.y);
            paint_line([p1, p2], Color32::from_rgb(255, 0, 0), 30.0);
            paint_line([p1, p3], Color32::from_rgb(255, 0, 0), 30.0);


            //draw in middle square
            //paint_rect(pos2(center_x, center_y), vec2(width / 3.0, height / 3.0), Color32::WHITE, to_screen, shapes);
            //recurse on 8 other squares 

            // if length > MIN_BRANCH_LENGTH{
            //     self.drawtree(
            //                         length * SCALING_FACTOR,
            //                         xR,
            //                         yR,
            //                         angle - BRANCH_ANGLE,
            //                         to_screen,
            //                         shapes,
            //                     );
                                
            //                     self.drawtree(
            //                         length * SCALING_FACTOR,
            //                         xL,
            //                         yL,
            //                         angle + BRANCH_ANGLE,
            //                         to_screen,
            //                         shapes,
            //                     );

            // }


            // if n > 0 {
            //     for row in -1..=1 {
            //         for col in -1..=1 {
            //             if row != 0 && col != 0 {
            //                 let new_center_x = center_x + (row as f32) * width / 3.0;
            //                 let new_center_y = center_y + (col as f32) * height / 3.0;
            //                 sierpinski_carpet(new_center_x, new_center_y, width / 3.0, height / 3.0, n - 1, to_screen, shapes);
            //             }
            //         }
            //     }
            // }
        };
//length: f32, x1: f32, y1: f32, angle: f32, to_screen: &emath::RectTransform, shapes: &mut Vec<Shape>
        drawtree(100.0, 100.0, 100.0, 0.0);

       

        
        painter.extend(shapes);
    }
}