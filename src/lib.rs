#![warn(clippy::all, rust_2018_idioms)]

mod ratagui_backend;

pub use ratagui_backend::RataguiBackend;
#[cfg(feature = "eframe")]
mod wasm_runner;
#[cfg(all(not(target_arch = "wasm32"),feature="eframe"))]
pub use wasm_runner::native_setup;

#[cfg(feature = "eframe")]
pub use wasm_runner::NewCC;

#[cfg(all(target_arch = "wasm32", feature = "eframe"))]
pub use wasm_runner::wasm_setup;


