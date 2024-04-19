#![warn(clippy::all, rust_2018_idioms)]

mod ratagui_backend;

pub use ratagui_backend::RataguiBackend;

mod wasm_runner;
#[cfg(not(target_arch = "wasm32"))]
pub use wasm_runner::native_setup;
pub use wasm_runner::NewCC;

#[cfg(target_arch = "wasm32")]
pub use wasm_runner::wasm_setup;
