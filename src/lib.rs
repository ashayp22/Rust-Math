// Imports and Settings
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod apps;
mod wrap_app;

pub use wrap_app::WrapApp;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/* 
This is the entry-point for all Web-Assembly project.
It is called once from the HTML. 
It loads the app, installs some callbacks, then returns.
*/

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = WrapApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}