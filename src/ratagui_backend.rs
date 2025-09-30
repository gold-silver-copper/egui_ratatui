//! This module provides the `RataguiBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use egui::{ColorImage, Response, TextureHandle, TextureOptions, Ui, Vec2};

use ratatui::layout::Position;
use soft_ratatui::{RasterBackend, SoftBackend};

use std::io;

use ratatui::{
    backend::{Backend, WindowSize},
    buffer::Cell,
    layout::Size,
};

//use egui::Label as TerminalLine;

/// RataguiBackend is an egui widget and a ratatui backend, see examples and [`soft_ratatui`](https://github.com/gold-silver-copper/soft_ratatui) documentation for more information
///```rust
/// let soft_backend = SoftBackend::<EmbeddedGraphics>::new(100,50,font_regular,Some(font_bold),Some(font_italic),);
/// let mut backend = RataguiBackend::new("soft_rat", soft_backend);
/// let mut terminal = Terminal::new(backend).unwrap();
/// ui.add(terminal.backend_mut())
/// ```
///
///

pub struct RataguiBackend<R: RasterBackend> {
    pub soft_backend: SoftBackend<R>,
    pub cur_size: Vec2,
    pub name: String,
    pub text_handle: Option<TextureHandle>,
}
impl<R: RasterBackend> egui::Widget for &mut RataguiBackend<R> {
    fn ui(self, ui: &mut Ui) -> Response {
        let av_size = ui.available_size();

        if self.cur_size != av_size {
            self.cur_size = av_size;

            let av_width = (av_size.x).clamp(1.0, 10000.0);
            let av_height = (av_size.y).clamp(1.0, 10000.0);
            let available_chars_width = (av_width / (self.soft_backend.char_width as f32)) as u16;

            let available_chars_height =
                (av_height / (self.soft_backend.char_height as f32)) as u16;
            let cur_size = self.size().expect("COULD NOT GET CURRENT BACKEND SIZE");
            if (cur_size.width != available_chars_width)
                || (cur_size.height != available_chars_height)
            {
                self.soft_backend
                    .resize(available_chars_width, available_chars_height);
            }
        }

        let texture =
            ui.ctx()
                .load_texture(&self.name, self.to_egui_image(), TextureOptions::NEAREST);
        self.text_handle = Some(texture.clone());

        ui.image((texture.id(), texture.size_vec2()))
    }
}

impl<R: RasterBackend> RataguiBackend<R> {
    /// Creates a new `RataguiBackend` with the given soft_ratatui backend, see [`soft_ratatui`](https://github.com/gold-silver-copper/soft_ratatui) documentation for more information. WASM compatible.
    pub fn new(name: &str, soft_backend: SoftBackend<R>) -> Self {
        let name = name.to_string();

        Self {
            soft_backend,
            cur_size: Vec2::new(1.0, 1.0),
            name,

            text_handle: None,
        }
    }

    ///Creates an Egui ColorImage from the terminal buffer.
    pub fn to_egui_image(&self) -> ColorImage {
        egui::ColorImage::from_rgb(
            [
                self.soft_backend.get_pixmap_width(),
                self.soft_backend.get_pixmap_height(),
            ],
            self.soft_backend.get_pixmap_data(),
        )
    }
}

impl<R: RasterBackend> Backend for RataguiBackend<R> {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.soft_backend.draw(content)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.soft_backend.hide_cursor()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.soft_backend.show_cursor()
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        self.soft_backend.get_cursor_position()
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        self.soft_backend.set_cursor_position(position)
    }

    fn clear(&mut self) -> io::Result<()> {
        self.soft_backend.clear()
    }

    fn size(&self) -> io::Result<Size> {
        self.soft_backend.size()
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        self.soft_backend.window_size()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.soft_backend.flush()
    }
}
