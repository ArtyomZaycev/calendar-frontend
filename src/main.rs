#![feature(associated_type_defaults)]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod table;
mod app;
mod app_local_storage;
mod config;
mod db;
mod local_storage;
mod requests;
mod state;
mod ui;
mod utils;

use app::CalendarApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dotenv::dotenv().ok();

    let rt = tokio::runtime::Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Calendar",
        native_options,
        Box::new(|cc| Box::new(CalendarApp::new(cc))),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(CalendarApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
