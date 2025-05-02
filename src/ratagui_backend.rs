//! This module provides the `RataguiBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use egui::style::ScrollStyle;
use egui::text::TextWrapping;
use egui::{
    epaint::{
        text::{LayoutJob, TextFormat},
        Color32, FontFamily, FontId, Fonts,
    },
    Margin,
};
use egui::{ColorImage, ImageSource, Label, Response, Stroke, TextureHandle, TextureOptions, Ui};

use ratatui::{
    layout::Position,
    style::{Color, Modifier},
};
use soft_ratatui::SoftBackend;

use std::io;
use std::path::Path;
use web_time::Instant;

use ratatui::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
};

//use egui::Label as TerminalLine;

///The RataguiBackend is the widget+backend itself , from which you can make a ratatui terminal ,
/// then you can do ui.add(terminal.backend_mut()) inside an egui context    .
/// Spawn with RataguiBackend::new() or RataguiBackend::new_with_fonts()   .
/// See the hello_world_web example for custom font usage

/*    let image: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(
    self.soft_backend.get_pixmap_width() as u32,
    self.soft_backend.get_pixmap_height() as u32,
    self.soft_backend.get_pixmap_data(),
)
.expect("Buffer size does not match width * height * 3");
println!("text size is {}", texture.size_vec2());
// Save the image as a PNG
image
    .save(Path::new("my_imagik.png"))
    .expect("Failed to save image");
 */

pub struct RataguiBackend {
    soft_backend: SoftBackend,

    text_handle: Option<TextureHandle>,
}
impl egui::Widget for &mut RataguiBackend {
    fn ui(self, ui: &mut Ui) -> Response {
        /*      let max_width = self.soft_backend.char_width * 250;
        let max_height = self.soft_backend.char_height * 250;

        let av_size = ui.available_size();

        let av_width = (av_size.x).clamp(1.0, max_width as f32); //- 2.0 * char_width
        let av_height = (av_size.y).clamp(1.0, max_height as f32); //- 2.0 * char_height
        let available_chars_width = (av_width / (self.soft_backend.char_width as f32)) as u16;

        let available_chars_height = (av_height / (self.soft_backend.char_height as f32)) as u16;
        let cur_size = self.size().expect("COULD NOT GET CURRENT BACKEND SIZE");
        if (cur_size.width != available_chars_width) || (cur_size.height != available_chars_height)
        {
            self.resize(available_chars_width, available_chars_height);
        } */

        let cur_w = self.soft_backend.buffer.area().width + 1;
        let cur_h = self.soft_backend.buffer.area().height + 1;

        if cur_w < 30 {
            self.resize(cur_w, cur_h);
        }

        let texture = ui.ctx().load_texture(
            "arrr", // texture ID (can be anything)
            self.to_egui(),
            //  self.to_egui(),   // your ColorImage
            TextureOptions::LINEAR,
        );
        self.text_handle = Some(texture.clone());

        println!("HI");

        //  let sizeik = texture.size_vec2();
        // ui.ctx().texture_ui(ui);
        ui.image((texture.id(), texture.size_vec2()))

        /*  ui.image(egui::include_image!("../assets/icon-1024.png"))
        .on_hover_text_at_pointer("WebP") */
    }
}

impl RataguiBackend {
    /// Creates a new `RataguiBackend` with the specified width and height, and default font.
    pub fn new(width: u16, height: u16) -> Self {
        let font_size = 16;
        let backend = SoftBackend::new(width, height, "../assets/fonts/Iosevka-Bold.ttf");

        Self {
            soft_backend: backend,

            text_handle: None,
        }
    }
    /// Resizes the `RataguiBackend` to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.soft_backend.resize(width, height);
    }

    pub fn to_egui(&self) -> ColorImage {
        egui::ColorImage::from_rgb(
            [
                self.soft_backend.get_pixmap_width(),
                self.soft_backend.get_pixmap_height(),
            ],
            self.soft_backend.get_pixmap_data(),
        )
    }

    /// Returns a reference to the internal buffer of the `RataguiBackend`.
    pub const fn buffer(&self) -> &Buffer {
        &self.soft_backend.buffer()
    }
}

impl Backend for RataguiBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.soft_backend.draw(content)?;

        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        //  todo!();
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        //  todo!();
        Ok(())
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        todo!();
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        todo!();
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.soft_backend.clear()?;
        Ok(())
    }

    fn size(&self) -> io::Result<Size> {
        Ok(Size {
            width: self.soft_backend.buffer.area().width,
            height: self.soft_backend.buffer.area().height,
        })
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        todo!();
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
