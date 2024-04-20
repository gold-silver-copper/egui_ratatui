use eframe::egui::{self};
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};
use ratframe::RataguiBackend;

pub fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let boop = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(boop).unwrap();
    terminal.clear().expect("epic fail");
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(Paragraph::new("Hello Rataguiii").white().on_blue(), area);
            })
            .expect("epic fail");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(terminal.backend_mut());
            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
        });
    })
    .expect("epic fail");
}
