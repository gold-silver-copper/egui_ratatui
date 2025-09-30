use eframe::egui;
use egui_ratatui::RataguiBackend;
use ratatui::Terminal;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use soft_ratatui::embedded_graphics_unicodefonts::{
    mono_8x13_atlas, mono_8x13_bold_atlas, mono_8x13_italic_atlas,
};
use soft_ratatui::{EmbeddedGraphics, SoftBackend};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let font_regular = mono_8x13_atlas();
    let font_italic = mono_8x13_italic_atlas();
    let font_bold = mono_8x13_bold_atlas();
    let soft_backend = SoftBackend::<EmbeddedGraphics>::new(
        100,
        50,
        font_regular,
        Some(font_bold),
        Some(font_italic),
    );
    let mut backend = RataguiBackend::new("soft_rat", soft_backend);
    //backend.set_font_size(12);
    let mut terminal = Terminal::new(backend).unwrap();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        terminal
            .draw(|frame| {
                let area = frame.area();
                let textik = format!("Hello eframe! The window area is {}", area);
                frame.render_widget(
                    Paragraph::new(textik)
                        .block(Block::new().title("Ratatui").borders(Borders::ALL))
                        .white()
                        .on_blue()
                        .wrap(Wrap { trim: false }),
                    area,
                );
            })
            .expect("epic fail");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(terminal.backend_mut());
        });
    })
}
