use eframe::egui;
use egui_ratatui::RataguiBackend;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Block, Borders, Paragraph, Wrap},
};

static FONT_DATA: &[u8] = include_bytes!("../../assets/iosevka.ttf");

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    let backend = RataguiBackend::new("soft_rat", 16, FONT_DATA);
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
