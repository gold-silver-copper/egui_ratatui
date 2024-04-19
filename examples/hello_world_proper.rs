use eframe::egui::{self, Context, Label};
use eframe::epaint::{
    text::{LayoutJob, TextFormat},
    Color32, FontFamily, FontId, Fonts,
};

use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};
use ratframe::RataguiBackend;

pub fn main() {
    run_app();
}

struct CustomApp {
    terminal: Terminal<RataguiBackend>,
}

impl CustomApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        //Creating the Ratatui backend/ Egui widget here
        //pass in font names after initializing fonts through egui
        let backend = RataguiBackend::new_with_fonts(
            100,
            100,
            "Regular".into(),
            "Bold".into(),
            "Oblique".into(),
            "BoldOblique".into(),
        );
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

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
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

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Iosevka-Regular.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Regular".into()),
        vec!["Regular".to_owned()],
    );
    fonts.font_data.insert(
        "Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Iosevka-Bold.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Bold".into()),
        vec!["Bold".to_owned()],
    );

    fonts.font_data.insert(
        "Oblique".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Iosevka-Oblique.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Oblique".into()),
        vec!["Oblique".to_owned()],
    );

    fonts.font_data.insert(
        "BoldOblique".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Iosevka-BoldOblique.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("BoldOblique".into()),
        vec!["BoldOblique".to_owned()],
    );

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
