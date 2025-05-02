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
use egui::{ImageSource, Label, Response, Stroke, Ui};

use image::{ImageBuffer, Rgb};
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
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RataguiBackend {
    soft_backend: SoftBackend,
}
impl egui::Widget for &mut RataguiBackend {
    fn ui(self, ui: &mut Ui) -> Response {
        let colorik = egui::ColorImage::from_rgb(
            [
                self.soft_backend.get_pixmap_width(),
                self.soft_backend.get_pixmap_height(),
            ],
            self.soft_backend.get_pixmap_data(),
        );

        let texture = ui.ctx().load_texture(
            "my-color-image", // texture ID (can be anything)
            colorik.clone(),  // your ColorImage
            Default::default(),
        );
        println!("HI");
        let image: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(
            self.soft_backend.get_pixmap_width() as u32,
            self.soft_backend.get_pixmap_height() as u32,
            self.soft_backend.get_pixmap_data(),
        )
        .expect("Buffer size does not match width * height * 4");

        // Save the image as a PNG
        image
            .save(Path::new("my_imagik.png"))
            .expect("Failed to save image");
        ui.image((texture.id(), texture.size_vec2()))
        /*  ui.image(egui::include_image!("../assets/icon-1024.png"))
        .on_hover_text_at_pointer("WebP") */
    }
}

impl RataguiBackend {
    /// Creates a new `RataguiBackend` with the specified width and height, and default font.
    pub fn new(width: u16, height: u16) -> Self {
        let font_size = 16;
        Self {
            soft_backend: SoftBackend::new(100, 50, "../assets/fonts/Iosevka-Bold.ttf"),
        }
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
        println!("DRAWWW");

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
            width: 10,
            height: 10,
        })
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        todo!();
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
