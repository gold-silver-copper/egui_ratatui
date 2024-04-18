use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use eframe::egui::{self, Context, Label};

use ratatui::prelude::*;
use ratframe::*;

use crate::{app::RatApp, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // create app and run it
    //

    let res = run_app();

    Ok(())
}

pub struct DemoApp {
    terminal: Terminal<RataguiBackend>,
    app: RatApp<'static>,
    tick_rate: Duration,
    last_tick: Instant,
}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        // setup terminal

        let backend = RataguiBackend::new_with_fonts(
            100,
            100,
            "Regular".into(),
            "Bold".into(),
            "Oblique".into(),
            "BoldOblique".into(),
        );
        let mut terminal = Terminal::new(backend).unwrap();
        Self {
            terminal: terminal,
            app: RatApp::new("WASM Demo", true),
            tick_rate: Duration::from_millis(80),
            last_tick: Instant::now(),
        }
    }
}
impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.terminal.draw(|f| ui::draw(f, &mut self.app));

        let timeout = self.tick_rate.saturating_sub(self.last_tick.elapsed());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());
            if ui.input(|i| i.key_released(egui::Key::H)) {
                self.app.on_left()
            }
            if ui.input(|i| i.key_released(egui::Key::K)) {
                self.app.on_up()
            }
            if ui.input(|i| i.key_released(egui::Key::L)) {
                self.app.on_right()
            }
            if ui.input(|i| i.key_released(egui::Key::J)) {
                self.app.on_down()
            }
            if ui.input(|i| i.key_released(egui::Key::Q)) {
                self.app.on_key('q')
            }
            if ui.input(|i| i.key_released(egui::Key::T)) {
                self.app.on_key('t')
            }
            //KeyCode::Char(c) => app.on_key(c),
        });

        if self.last_tick.elapsed() >= timeout {
            self.app.on_tick();
            self.last_tick = Instant::now();
        }

        if self.app.should_quit {
            panic!("a wonderful way to quit");
        }
    }
}
fn run_app() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui example: custom font",
        options,
        Box::new(|cc| Box::new(DemoApp::new(cc))),
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
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Iosevka-Regular.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Regular".into()),
        vec!["Regular".to_owned()],
    );
    fonts.font_data.insert(
        "Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Iosevka-Bold.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Bold".into()),
        vec!["Bold".to_owned()],
    );

    fonts.font_data.insert(
        "Oblique".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Iosevka-Oblique.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Oblique".into()),
        vec!["Oblique".to_owned()],
    );

    fonts.font_data.insert(
        "BoldOblique".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Iosevka-BoldOblique.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("BoldOblique".into()),
        vec!["BoldOblique".to_owned()],
    );

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
