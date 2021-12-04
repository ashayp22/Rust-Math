use eframe::{egui, epi};
use egui::{containers::*, widgets::*, *};
use rand::Rng; // 0.8.0

#[derive(PartialEq)]

/*
This file contains the code for recursively generating and then rendering the Sierpinski Carpet.
Fractal Explanation: https://en.wikipedia.org/wiki/Sierpi%C5%84ski_carpet
*/

pub struct SierpinskiCarpet {
    depth: usize,                // The level of the recursion when generating the fractal
    last_depth: usize,           // The depth of the last drawn fractal
    show_randomness: bool, // If true, the squares in the sierpinski carpet will be shown randomly,
    last_show_randomness: bool, // Used to determine if show_randomness was changed
    randomness_probability: f64, // When showing randomness, this represents the probability that
    last_randomness_probability: f64, // Same as last_show_randomness
    show_blue_shades: bool, // If true, changes the Carpet's color to ROY G BIV
    last_show_blue_shades: bool, // Same as last_show_randomness
    shapes: Vec<Shape>,    // A vector containing the shapes that will be painted on the screen
}

/*
Small Aside: The reason why there are a lot of last_show... is because egui doesn't support callbacks.
Thus, there is no way of telling whether a slider or checkbox's value was changed without using a second variable or similar means.
*/

impl Default for SierpinskiCarpet {
    fn default() -> Self {
        Self {
            depth: 1,
            last_depth: 0,
            show_randomness: false,
            last_show_randomness: false,
            randomness_probability: 0.5,
            last_randomness_probability: 0.5,
            show_blue_shades: false,
            last_show_blue_shades: false,
            shapes: Vec::new(),
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
    }

    // An options window for setting the depth of the fractal generation
    fn options_ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.depth, 1..=6).text("Depth"));
        ui.checkbox(&mut self.show_randomness, "Include Randomness");

        if self.show_randomness {
            ui.add(
                Slider::new(&mut self.randomness_probability, 0.0..=1.0)
                    .text("Randomness Probability"),
            );
        }

        ui.checkbox(&mut self.show_blue_shades, "Shades of Blue");
        egui::reset_button(ui, self);
    }

    // Paints a rectangular given its center and size
    fn paint_rect(&mut self, center: Pos2, size: Vec2, color: Color32) {
        self.shapes.push(Shape::rect_filled(
            Rect::from_center_size(center, size),
            0.0,
            color,
        ));
    }

    // A recursive function for creatng the sierpinski carpet
    fn sierpinski_carpet(
        &mut self,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
        n: i64,
        level: i64,
    ) {
        let mut can_draw = true;

        if self.show_randomness {
            let num: f64 = rand::thread_rng().gen();
            can_draw = num <= self.randomness_probability;
        }

        if can_draw {
            // let colors = vec![Color32::RED, Color32::from_rgb(255, 165, 0), Color32::YELLOW, Color32::GREEN, Color32::BLUE, Color32::from_rgb(75, 0, 130), Color32::from_rgb(143, 0, 255)];
            let blue_colors = vec![
                Color32::from_rgb(144, 224, 239),
                Color32::from_rgb(72, 202, 228),
                Color32::from_rgb(90, 180, 216),
                Color32::from_rgb(0, 150, 199),
                Color32::from_rgb(0, 119, 182),
                Color32::from_rgb(2, 62, 138),
                Color32::from_rgb(3, 4, 94),
            ];

            // draw in middle square
            if self.show_blue_shades {
                if level >= 0 && level <= 6 {
                    let color = blue_colors[level as usize];
                    self.paint_rect(
                        pos2(center_x, center_y),
                        vec2(width / 3.0, height / 3.0),
                        color,
                    );
                } else {
                    self.paint_rect(
                        pos2(center_x, center_y),
                        vec2(width / 3.0, height / 3.0),
                        Color32::WHITE,
                    );
                }
            } else {
                self.paint_rect(
                    pos2(center_x, center_y),
                    vec2(width / 3.0, height / 3.0),
                    Color32::BLUE,
                );
            }
        }

        // recurse on 8 other squares until you hit the base case n = 0
        if n > 0 {
            for row in -1..=1 {
                for col in -1..=1 {
                    if row != 0 || col != 0 {
                        let new_center_x = center_x + (row as f32) * width / 3.0;
                        let new_center_y = center_y + (col as f32) * height / 3.0;
                        self.sierpinski_carpet(
                            new_center_x,
                            new_center_y,
                            width / 3.0,
                            height / 3.0,
                            n - 1,
                            level + 1,
                        );
                    }
                }
            }
        }
    }

    fn paint(&mut self, painter: &Painter) {
        let rect = painter.clip_rect();

        let need_to_recalculate = self.depth != self.last_depth
            || self.show_blue_shades != self.last_show_blue_shades
            || self.show_randomness != self.last_show_randomness
            || self.randomness_probability != self.last_randomness_probability;

        // Need to recalculate due to a change in one of the settings
        if need_to_recalculate {
            self.shapes.clear();
            // Calculates the fractal in the center of the screen
            let num_levels = self.depth as i64;
            self.sierpinski_carpet(
                rect.width() / 2.0,
                rect.height() / 2.0,
                500.0,
                500.0,
                num_levels,
                0,
            );
        }

        // Update the the markers in order to prevent further recalculations
        self.last_depth = self.depth;
        self.last_show_randomness = self.show_randomness;
        self.last_show_blue_shades = self.show_blue_shades;
        self.last_randomness_probability = self.randomness_probability;

        // Copy over shapes since painter.extend doesn't take a reference
        let cloned_shapes: std::vec::Vec<Shape> = self.shapes.clone();

        // Draws the fractal
        painter.extend(cloned_shapes);
    }
}
