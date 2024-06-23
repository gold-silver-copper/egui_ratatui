#![warn(clippy::all, rust_2018_idioms)]

mod ratagui_backend;

pub use ratagui_backend::RataguiBackend;


mod terminal_line;

pub use terminal_line::TerminalLine;