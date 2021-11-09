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
            .show(ctx, |ui| self.ui(ui, crate::seconds_since_midnight()));
    }
}



// impl epi::App for TemplateApp {
//     fn name(&self) -> &str {
//         "eframe template"
//     }

//     /// Called once before the first frame.
//     fn setup(
//         &mut self,
//         _ctx: &egui::CtxRef,
//         _frame: &mut epi::Frame<'_>,
//         _storage: Option<&dyn epi::Storage>,
//     ) {

//     }

//     /// Called by the frame work to save state before shutdown.
//     /// Note that you must enable the `persistence` feature for this to work.
//     #[cfg(feature = "persistence")]
//     fn save(&mut self, storage: &mut dyn epi::Storage) {
//         epi::set_value(storage, epi::APP_KEY, self);
//     }

//     /// Called each time the UI needs repainting, which may be many times per second.
//     /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
//     fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
//         let Self { label, value } = self;

//         // Examples of how to create different panels and windows.
//         // Pick whichever suits you.
//         // Tip: a good default choice is to just keep the `CentralPanel`.
//         // For inspiration and more examples, go to https://emilk.github.io/egui

//         egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
//             // The top panel is often a good place for a menu bar:
//             egui::menu::bar(ui, |ui| {
//                 egui::menu::menu(ui, "abcdefg", |ui| {
//                     if ui.button("Quit").clicked() {
//                         frame.quit();
//                     }
//                 });
//             });
//         });

//         egui::SidePanel::left("side_panel").show(ctx, |ui| {
//             ui.heading("Side Do it work with caching pt 3?");

//             ui.horizontal(|ui| {
//                 ui.label("Write something: ");
//                 ui.text_edit_singleline(label);
//             });

//             ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
//             if ui.button("Increment").clicked() {
//                 *value += 1.0;
//             }

//             ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
//                 ui.horizontal(|ui| {
//                     ui.spacing_mut().item_spacing.x = 0.0;
//                     ui.label("powered by ");
//                     ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//                 });
//             });
//         });

//         egui::CentralPanel::default().show(ctx, |ui| {
//             // The central panel the region left after adding TopPanel's and SidePanel's

//             ui.heading("eframe template");
//             ui.hyperlink("https://github.com/emilk/eframe_template");
//             ui.add(egui::github_link_file!(
//                 "https://github.com/emilk/eframe_template/blob/master/",
//                 "Source code."
//             ));
//             egui::warn_if_debug_build(ui);
//         });

//         if false {
//             egui::Window::new("Window").show(ctx, |ui| {
//                 ui.label("Windows can be moved by dragging them.");
//                 ui.label("They are automatically sized based on contents.");
//                 ui.label("You can turn on resizing and scrolling if you like.");
//                 ui.label("You would normally chose either panels OR windows.");
//             });
//         }
//     }
// }




impl FractalClock {
    pub fn ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        // if !self.paused {
        //     self.time = seconds_since_midnight.unwrap_or_else(|| ui.input().time);
        //     ui.ctx().request_repaint();
        // }

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
                    .show(ui, |ui| self.options_ui(ui, seconds_since_midnight));
            });
    }

    fn options_ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        // if seconds_since_midnight.is_some() {
        //     ui.label(format!(
        //         "Local time: {:02}:{:02}:{:02}.{:03}",
        //         (self.time % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
        //         (self.time % (60.0 * 60.0) / 60.0).floor(),
        //         (self.time % 60.0).floor(),
        //         (self.time % 1.0 * 100.0).floor()
        //     ));
        // } else {
        //     ui.label("The fractal_clock clock is not showing the correct time");
        // };
        ui.label(format!("Painted line count: {}", self.line_count));

        ui.checkbox(&mut self.paused, "Paused");
        ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        ui.add(Slider::new(&mut self.depth, 0..=14).text("depth"));
        ui.add(Slider::new(&mut self.length_factor, 0.0..=1.0).text("length factor"));
        ui.add(Slider::new(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
        ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));

        egui::reset_button(ui, self);

        // ui.hyperlink_to(
        //     "Inspired by a screensaver by Rob Mayoff",
        //     "http://www.dqd.com/~mayoff/programs/FractalClock/",
        // );
    }

    fn drawtree(&self,  length: f32,  x1: f32,  y1: f32,  angle: f32, painter: &Painter){
        let mut shapes: Vec<Shape> = Vec::new();
        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let rect = painter.clip_rect();
             let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );
            let line = [to_screen * points[0], to_screen * points[1]];

            // culling
            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };
        let SCALING_FACTOR = 0.87;
        let BRANCH_ANGLE = 0.26;//0.26;
        let MIN_BRANCH_LENGTH = 6.0;
       let xR=x1+(( angle-BRANCH_ANGLE)*length).cos();
       let yR=y1-(( angle-BRANCH_ANGLE)*length).sin();
       let  xL=x1+(( angle+BRANCH_ANGLE)*length).cos();
       let  yL=y1-(( angle+BRANCH_ANGLE)*length).sin();
        if length <= MIN_BRANCH_LENGTH
         {
            
            
            let p1 = pos2(x1, y1,);
            let p2 = pos2(xR, yR);
            let p3 = pos2(xL, yL );

             paint_line([p1, p2], Color32::from_rgb(255, 0, 0), 2.5);
           paint_line([p1, p3], Color32::from_rgb(255, 0, 0), 2.5);

             
             
             
             return;
         }
         else
         {
            
            let p1 = pos2(x1, y1,);
            let p2 = pos2(xR, yR);
            let p3 = pos2(xL, yL );

            paint_line([p1, p2], Color32::from_rgb(255, 0, 0), 2.5);
            paint_line([p1, p3], Color32::from_rgb(255, 0, 0), 2.5);
            self.line_count = shapes.len();
                painter.extend(shapes);
                drawTree ( length*SCALING_FACTOR, xR, yR , angle-BRANCH_ANGLE,painter  );
                drawTree ( length*SCALING_FACTOR, xL, yL , angle+BRANCH_ANGLE, painter);
            }

    }

    fn paint(&mut self, painter: &Painter) {
        
        self. drawtree( 25.0,  15.0, 15.0,  0.0, painter);



        // struct Hand {
        //     length: f32,
        //     angle: f32,
        //     vec: Vec2,
        // }

        // impl Hand {
        //     fn from_length_angle(length: f32, angle: f32) -> Self {
        //         Self {
        //             length,
        //             angle,
        //             vec: length * Vec2::angled(angle),
        //         }
        //     }
        // }

        // let angle_from_period =
        //     |period| TAU * (self.time.rem_euclid(period) / period) as f32 - TAU / 4.0;

        // let hands = [
        //     // Second hand:
        //     Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
        //     // Minute hand:
        //     Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
        //     // Hour hand:
        //     Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
        // ];

        // let mut shapes: Vec<Shape> = Vec::new();

        // let rect = painter.clip_rect();
        // let to_screen = emath::RectTransform::from_to(
        //     Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
        //     rect,
        // );

        

//         let hand_rotations = [
//             hands[0].angle - hands[2].angle + TAU / 2.0,
//             hands[1].angle - hands[2].angle + TAU / 2.0,
//         ];

//         let hand_rotors = [
//             hands[0].length * emath::Rot2::from_angle(hand_rotations[0]),
//             hands[1].length * emath::Rot2::from_angle(hand_rotations[1]),
//         ];

//         #[derive(Clone, Copy)]
//         struct Node {
//             pos: Pos2,
//             dir: Vec2,
//         }

//         let mut nodes = Vec::new();

//         let mut width = self.start_line_width;


//         //

       

// //
//         for (i, hand) in hands.iter().enumerate() {
//             let center = pos2(0.0, 0.0);
//             let end = center + hand.vec;
//             paint_line([center, end], Color32::from_additive_luminance(255), width);
//             if i < 2 {
//                 nodes.push(Node {
//                     pos: end,
//                     dir: hand.vec,
//                 });
//             }
//         }

//         let mut luminance = 0.7; // Start dimmer than main hands

//         let mut new_nodes = Vec::new();
//         for _ in 0..self.depth {
//             new_nodes.clear();
//             new_nodes.reserve(nodes.len() * 2);

//             luminance *= self.luminance_factor;
//             width *= self.width_factor;

//             let luminance_u8 = (255.0 * luminance).round() as u8;
//             if luminance_u8 == 0 {
//                 break;
//             }

//             for &rotor in &hand_rotors {
//                 for a in &nodes {
//                     let new_dir = rotor * a.dir;
//                     let b = Node {
//                         pos: a.pos + new_dir,
//                         dir: new_dir,
//                     };
//                     paint_line(
//                         [a.pos, b.pos],
//                         Color32::from_additive_luminance(luminance_u8),
//                         width,
//                     );
//                     new_nodes.push(b);
//                 }
//             }

//             std::mem::swap(&mut nodes, &mut new_nodes);
//         }
//         self.line_count = shapes.len();
//         painter.extend(shapes);
    }
}