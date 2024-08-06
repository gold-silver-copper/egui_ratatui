//! This module provides the `RataguiBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use egui::epaint::{
    text::{LayoutJob, TextFormat},
    Color32, FontFamily, FontId, Fonts,
};
use egui::text::TextWrapping;
use egui::{Label, Response, Stroke, Ui};

use ratatui::style::{Color, Modifier};
use std::f32::INFINITY;
use std::io;
use web_time::Instant;

use ratatui::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
};

use crate::TerminalLine;
//use egui::Label as TerminalLine;

///The RataguiBackend is the widget+backend itself , from which you can make a ratatui terminal ,
/// then you can do ui.add(terminal.backend_mut()) inside an egui context    .
/// Spawn with RataguiBackend::new() or RataguiBackend::new_with_fonts()   .
/// See the hello_world_web example for custom font usage
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RataguiBackend {
    width: u16,
    buffer: Buffer,
    height: u16,
    cursor: bool,
    font_size: u16,
    pos: (u16, u16),
    regular_font: FontId,
    bold_font: FontId,
    italic_font: FontId,
    bolditalic_font: FontId,
    timestamp: Instant,
    blinking_slow: bool,
    blinking_fast: bool,
}
impl egui::Widget for &mut RataguiBackend {
    fn ui(self, ui: &mut Ui) -> Response {
        let spacik = egui::style::Spacing {
            item_spacing: egui::vec2(0.0, 0.0),
            ..Default::default()
        };
        *ui.spacing_mut() = spacik;
        let elpsd = self.timestamp.elapsed().as_millis();

        if elpsd > 1200 {
            self.timestamp = Instant::now();
            self.blinking_fast = false;
            self.blinking_slow = false;
        } else if elpsd > 1000 {
            self.blinking_fast = true;
        } else if elpsd > 800 {
            self.blinking_slow = true;
            self.blinking_fast = false;
        } else if elpsd > 600 {
            self.blinking_fast = true;
        } else if elpsd > 400 {
            self.blinking_fast = false;
        } else if elpsd > 200 {
            self.blinking_fast = true;
        }

        let char_height = ui.fonts(|fx| fx.row_height(&self.regular_font));
        let char_width = ui.fonts(|fx| self.get_font_width(fx));

        // it is limited to this because the ratatui buffer is cast to u8 somewhere

        let max_width = char_width * 250.0;
        let max_height = char_height * 250.0;

        let av_size = ui.available_size();

        let av_width = (av_size.x).clamp(1.0, max_width);
        let av_height = (av_size.y).clamp(1.0, max_height);

        // there are weird issues with high dpi displays relating to native pixels per point and zoom factor
        let available_chars_width = ((av_width / (char_width)) as u16);

        let available_chars_height = (av_height / (char_height)) as u16;
        //println!("av chars width: {:#?}",available_chars_width);
        /*
        if available_chars_width >55 {available_chars_width-=available_chars_width/60;}
        if available_chars_height >40 {available_chars_height-=available_chars_height/60;}

         */

        let cur_size = self.size().expect("COULD NOT GET CURRENT BACKEND SIZE");

        if (cur_size.width != available_chars_width) || (cur_size.height != available_chars_height)
        {
            self.resize(available_chars_width, available_chars_height);
        }
        let cur_buf = self.buffer();

        let singular_wrapping = TextWrapping {
            max_width: f32::INFINITY,
            max_rows: 1,
            break_anywhere: false,
            overflow_character: None,
        };

        for y in 0..available_chars_height {
            let mut job = LayoutJob {
                text: Default::default(),
                sections: Default::default(),
                wrap: singular_wrapping.clone(),
                first_row_min_height: 0.0,
                break_on_newline: false,
                halign: egui::Align::LEFT,
                justify: false,
                round_output_size_to_nearest_ui_point: false,
            };
            for x in 0..available_chars_width {
                let cur_cell = cur_buf.get(x, y);

                let is_bold = cur_cell.modifier.contains(Modifier::BOLD);
                let is_italic = cur_cell.modifier.contains(Modifier::ITALIC);
                let is_underlined = cur_cell.modifier.contains(Modifier::UNDERLINED);
                let is_slowblink = cur_cell.modifier.contains(Modifier::SLOW_BLINK);
                let is_rapidblink = cur_cell.modifier.contains(Modifier::RAPID_BLINK);
                let is_reversed = cur_cell.modifier.contains(Modifier::REVERSED);
                let is_dim = cur_cell.modifier.contains(Modifier::DIM);
                let is_hidden = cur_cell.modifier.contains(Modifier::HIDDEN);
                let is_crossed_out = cur_cell.modifier.contains(Modifier::CROSSED_OUT);

                let tf_font = if is_bold && is_italic {
                    self.bolditalic_font.to_owned()
                } else if is_bold {
                    self.bold_font.to_owned()
                } else if is_italic {
                    self.italic_font.to_owned()
                } else {
                    self.regular_font.to_owned()
                };

                let mut tf_fg_color = RataguiBackend::rat_to_egui_color(&cur_cell.fg, true);
                let mut tf_bg_color = RataguiBackend::rat_to_egui_color(&cur_cell.bg, false);

                if is_slowblink {
                    if self.blinking_slow {
                        tf_fg_color = tf_bg_color.clone();
                    }
                }
                if is_rapidblink {
                    if self.blinking_fast {
                        tf_fg_color = tf_bg_color.clone();
                    }
                }

                if is_dim {
                    tf_fg_color = tf_fg_color.gamma_multiply(0.7);
                    tf_bg_color = tf_bg_color.gamma_multiply(0.7);
                }

                if is_reversed {
                    let holder = tf_bg_color;
                    tf_bg_color = tf_fg_color;
                    tf_fg_color = holder;
                }
                if is_hidden {
                    tf_fg_color = tf_bg_color.clone();
                }

                let tf_stroke = if is_crossed_out {
                    Stroke::new(char_height / 8.0, tf_fg_color)
                } else {
                    Stroke::NONE
                };

                let tf_underline = if is_underlined {
                    Stroke::new(char_height / 3.0, tf_fg_color)
                } else {
                    Stroke::NONE
                };

                let tf = TextFormat {
                    font_id: tf_font,
                    color: tf_fg_color,
                    background: tf_bg_color,
                    strikethrough: tf_stroke,
                    underline: tf_underline,
                    //     valign: egui::Align::Min,
                    //  line_height: Some(char_height - 0.01),
                    ..Default::default()
                };

                job.append(cur_cell.symbol(), 0.0, tf.clone());

                if x == (available_chars_width - 1) {
                    let end = ui.add(TerminalLine::new(job.clone()));
                    if y == (available_chars_height - 1) {
                        return end;
                    }
                }
            }
        }

        let emd = Label::new("IF YOU SEE THIS  THAT IS AN ERROR");

        ui.add(emd)
    }
}

impl RataguiBackend {
    /// Creates a new `RataguiBackend` with the specified width and height.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor: false,
            font_size: 16,
            pos: (0, 0),
            regular_font: FontId::new(16.0, FontFamily::Monospace),
            bold_font: FontId::new(16.0, FontFamily::Monospace),
            italic_font: FontId::new(16.0, FontFamily::Monospace),
            bolditalic_font: FontId::new(16.0, FontFamily::Monospace),
            timestamp: Instant::now(),
            blinking_slow: false,
            blinking_fast: false,
        }
    }
    pub fn new_with_fonts(
        width: u16,
        height: u16,
        regular: String,
        bold: String,
        italic: String,
        bolditalic: String,
    ) -> Self {
        Self {
            width,
            height,
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor: false,
            font_size: 16,
            pos: (0, 0),
            regular_font: FontId::new(16.0, FontFamily::Name(regular.to_owned().into())),
            bold_font: FontId::new(16.0, FontFamily::Name(bold.to_owned().into())),
            italic_font: FontId::new(16.0, FontFamily::Name(italic.to_owned().into())),
            bolditalic_font: FontId::new(16.0, FontFamily::Name(bolditalic.to_owned().into())),
            timestamp: Instant::now(),
            blinking_slow: false,
            blinking_fast: false,
        }
    }

    pub fn get_font_size(&self) -> u16 {
        self.font_size.clone()
    }
    pub fn set_font_size(&mut self, desired: u16) {
        self.font_size = desired;

        self.regular_font = FontId::new(desired as f32, self.regular_font.family.to_owned());
        self.bold_font = FontId::new(desired as f32, self.bold_font.family.to_owned());
        self.italic_font = FontId::new(desired as f32, self.italic_font.family.to_owned());
        self.bolditalic_font = FontId::new(desired as f32, self.bolditalic_font.family.to_owned());
    }

    /// Returns a reference to the internal buffer of the `RataguiBackend`.
    pub const fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Resizes the `RataguiBackend` to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }
    pub fn get_font_width(&self, fontiki: &Fonts) -> f32 {
        let fid = self.regular_font.clone();
        let widik = fontiki.glyph_width(&fid, ' ');
        // println!("widik is {:#?}",widik);
        widik
    }

    pub fn rat_to_egui_color(rat_col: &ratatui::style::Color, is_a_fg: bool) -> Color32 {
        match rat_col {
            Color::Reset => {
                if is_a_fg {
                    Color32::from_rgb(204, 204, 255)
                } else {
                    Color32::from_rgb(15, 15, 112)
                }
            }
            Color::Black => Color32::BLACK,
            Color::Red => Color32::DARK_RED,
            Color::Green => Color32::DARK_GREEN,
            Color::Yellow => Color32::GOLD,
            Color::Blue => Color32::DARK_BLUE,
            Color::Magenta => Color32::from_rgb(99, 9, 99),
            Color::Cyan => Color32::BLUE,
            Color::Gray => Color32::GRAY,
            Color::DarkGray => Color32::DARK_GRAY,
            Color::LightRed => Color32::LIGHT_RED,
            Color::LightGreen => Color32::GREEN,
            Color::LightBlue => Color32::LIGHT_BLUE,
            Color::LightYellow => Color32::LIGHT_YELLOW,
            Color::LightMagenta => Color32::from_rgb(139, 0, 139),
            Color::LightCyan => Color32::from_rgb(224, 255, 255),
            Color::White => Color32::WHITE,
            Color::Indexed(i) => Color32::from_rgb(
                i.overflowing_mul(i.to_owned()).0,
                i.overflowing_add(i.to_owned()).0,
                i.to_owned(),
            ),
            Color::Rgb(r, g, b) => Color32::from_rgb(r.to_owned(), g.to_owned(), b.to_owned()),
        }
    }
}

impl Backend for RataguiBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        Ok(self.pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear()?,
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1) + 1;
                self.buffer.content[index..].fill(Cell::default());
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                self.buffer.content[..index].fill(Cell::default());
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[line_start_index..=line_end_index].fill(Cell::default());
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[index..=line_end_index].fill(Cell::default());
            }
        }
        Ok(())
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        let (cur_x, cur_y) = self.get_cursor()?;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(self.width.saturating_sub(1));

        let max_y = self.height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);
        if n > lines_after_cursor {
            let rotate_by = n.saturating_sub(lines_after_cursor).min(max_y);

            if rotate_by == self.height - 1 {
                self.clear()?;
            }

            self.set_cursor(0, rotate_by)?;
            self.clear_region(ClearType::BeforeCursor)?;
            self.buffer
                .content
                .rotate_left((self.width * rotate_by).into());
        }

        let new_cursor_y = cur_y.saturating_add(n).min(max_y);
        self.set_cursor(new_cursor_x, new_cursor_y)?;

        Ok(())
    }

    fn size(&self) -> io::Result<Rect> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        static WINDOW_PIXEL_SIZE: Size = Size {
            width: 640,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: WINDOW_PIXEL_SIZE,
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
