use egui::{ColorImage, Response, TextureHandle, TextureOptions, Ui, Vec2};

use ratatui::layout::Position;
use soft_ratatui::SoftBackend;

use std::io;

use ratatui::{
    backend::{Backend, WindowSize},
    buffer::Cell,
    layout::Size,
};

/// The `RataguiBackend` is the combined widget and backend used to render a `ratatui` terminal
/// within an `egui` context.
///
/// After creating a terminal, you can integrate it like this:
/// `ui.add(terminal.backend_mut())`
///
/// Spawn it using either:
/// * `RataguiBackend::new()`
/// * `RataguiBackend::new_with_system_fonts()`
///
/// For more information, see the `soft_ratatui` crate:
/// 📦 [GitHub](https://github.com/gold-silver-copper/soft_ratatui)
/// 📚 [Docs.rs](https://docs.rs/soft_ratatui/latest/soft_ratatui/)

pub struct RataguiBackend {
    soft_backend: SoftBackend,
    cur_size: Vec2,
    name: String,
    text_handle: Option<TextureHandle>,
}
impl egui::Widget for &mut RataguiBackend {
    fn ui(self, ui: &mut Ui) -> Response {
        let spacik = egui::style::Spacing {
            item_spacing: egui::vec2(0.0, 0.0),
            ..Default::default()
        };
        *ui.spacing_mut() = spacik;
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

impl RataguiBackend {
    /// Creates a new `RataguiBackend` with the specified name, font size, and font data.
    ///
    /// * name      : &str   - Name used to identify the terminal window or context
    /// * font-size : u16    - Size of the font in pixels
    /// * font-data : &[u8]  - Font bytes (e.g., from `include_bytes!`)
    ///
    /// ✅ This method is compatible with WASM/Web targets.
    ///
    /// # Examples
    /// ```rust
    /// static FONT_DATA: &[u8] = include_bytes!("../../assets/tc.ttf");
    /// let backend = RataguiBackend::new("My Terminal", 16, FONT_DATA);
    /// ```

    pub fn new(name: &str, font_size: u16, font_data: &[u8]) -> Self {
        let backend = SoftBackend::new_with_font(10, 10, font_size as i32, font_data);
        let name = name.to_string();

        Self {
            soft_backend: backend,
            cur_size: Vec2::new(1.0, 1.0),
            name,

            text_handle: None,
        }
    }
    /// Creates a new `RataguiBackend` with the specified name and font size using system fonts.
    ///
    /// * name      : &str  - Name used to identify the terminal window or context
    /// * font-size : u16   - Size of the font in pixels
    ///
    /// ⚠️ Not compatible with WASM/Web targets.
    ///
    /// # Examples
    /// ```rust
    /// let backend = RataguiBackend::new_with_system_fonts("My Terminal", 16);
    /// ```

    pub fn new_with_system_fonts(name: &str, font_size: u16) -> Self {
        let backend = SoftBackend::new_with_system_fonts(10, 10, font_size as i32);
        let name = name.to_string();
        Self {
            soft_backend: backend,
            cur_size: Vec2::new(1.0, 1.0),
            name,

            text_handle: None,
        }
    }

    ///Creates an Egui ColorImage from the terminal backend.
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

impl Backend for RataguiBackend {
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
