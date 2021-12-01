// Imports and Settings
use eframe::{egui, epi};

#[derive(Default)]

/*
This file contains a wrapper app for the fractals. It allows the user to 
switch between the fractals through a nav bar at the top of the screen.

Source: https://github.com/emilk/egui/tree/master/egui_demo_lib 
*/


pub struct Apps {
    fibonacci: crate::apps::FibonacciWord,
    htree: crate::apps::HTree,
    sierpinski: crate::apps::SierpinskiCarpet,
    mandelbrot: crate::apps::Mandelbrot,
}

impl Apps {
    fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut dyn epi::App)> {
        vec![
            ("sierpinski", &mut self.sierpinski as &mut dyn epi::App),
            ("fibonacci", &mut self.fibonacci as &mut dyn epi::App),
            ("htree", &mut self.htree as &mut dyn epi::App),
            ("mandelbrot", &mut self.mandelbrot as &mut dyn epi::App),
        ]
        .into_iter()
    }
}

/// Wraps many demo/test apps into one.
#[derive(Default)]
pub struct WrapApp {
    selected_anchor: String,
    apps: Apps,
}

impl epi::App for WrapApp {
    fn name(&self) -> &str {
        "Fractals"
    }

    // Called every frame
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if let Some(web_info) = frame.info().web_info.as_ref() {
            if let Some(anchor) = web_info.web_location_hash.strip_prefix('#') {
                self.selected_anchor = anchor.to_owned();
            }
        }

        if self.selected_anchor.is_empty() {
            self.selected_anchor = self.apps.iter_mut().next().unwrap().0.to_owned();
        }

        // Navbar for choosing which fractal to show
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            self.bar_contents(ui, frame);
        });

        // Handles showing the chosen fractal to display
        for (anchor, app) in self.apps.iter_mut() {
            if anchor == self.selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
            }
        }
    }
}

impl WrapApp {
    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut epi::Frame<'_>) {
        // A menu-bar is a horizontal layout with some special styles applied.
        ui.horizontal_wrapped(|ui| {
            egui::widgets::global_dark_light_mode_switch(ui);

            ui.separator();

            ui.label("Fractals with Rust & WebAssembly");

            ui.separator();

            // Handles selecting the chosen fractal
            for (anchor, app) in self.apps.iter_mut() {
                if ui
                    .selectable_label(self.selected_anchor == anchor, app.name())
                    .clicked()
                {
                    self.selected_anchor = anchor.to_owned();
                    if frame.is_web() {
                        ui.output().open_url(format!("#{}", anchor));
                    }
                }
            }

        });
    }
}