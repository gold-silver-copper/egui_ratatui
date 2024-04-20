#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// Implement this trait when you want to setup ratatui to use custom fonts
/// Otherwise it will use the default egui font which is missing extra unicode characters
pub trait NewCC {
    fn new(cc: &eframe::CreationContext<'_>) -> Self;
}
/// When compiling natively this function generates an eframe::NativeOptions then
/// does eframe::run_native() on your eframe::App
#[cfg(not(target_arch = "wasm32"))]
pub fn native_setup<T: eframe::App + NewCC + 'static>(eapp: T) -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "gold silver copper",
        native_options,
        Box::new(|cc| Box::new(T::new(cc))),
    )
}

/// When compiling for wasm32 this function generates an eframe::WebOptions then
/// spawns an async wasm bindgen eframe::WebRunner::new() for your eframe::App
#[cfg(any(target_arch = "wasm32", doc))]
pub fn wasm_setup<T: eframe::App + NewCC + 'static>(eapp: T) {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(T::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
