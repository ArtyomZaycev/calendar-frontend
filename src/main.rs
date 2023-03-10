#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod config;
mod db;
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
    eframe::run_native(
        "Calendar",
        native_options,
        Box::new(|cc| Box::new(CalendarApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(CalendarApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
