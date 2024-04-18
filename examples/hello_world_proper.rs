use eframe::egui::{self, Context, Label};
use eframe::epaint::{
    text::{LayoutJob, TextFormat},
    Color32, FontFamily, FontId, Fonts,
};

use ratagui::RataguiBackend;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};

pub fn main() {
    run_app();
}

struct CustomApp {
    terminal: Terminal<RataguiBackend>,
}

impl CustomApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //  setup_custom_fonts(&cc.egui_ctx);

        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        Self { terminal: terminal }
    }
}
impl eframe::App for CustomApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(Paragraph::new("Hello Rataguiii").white().on_blue(), area);
            })
            .expect("epic fail");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());
            if ui.input(|i| i.key_released(egui::Key::H)) {
                ()
            }
            if ui.input(|i| i.key_released(egui::Key::K)) {
                ()
            }
            if ui.input(|i| i.key_released(egui::Key::L)) {
                ()
            }
            if ui.input(|i| i.key_released(egui::Key::J)) {
                ()
            }
            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::T)) {
                ()
            }
            //KeyCode::Char(c) => app.on_key(c),
        });
    }
}
fn run_app() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };
    _ = eframe::run_native(
        "hello _world",
        options,
        Box::new(|cc| Box::new(CustomApp::new(cc))),
    );
    ()
}
