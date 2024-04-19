use std::{error::Error, io};
use web_time::{Duration, Instant, SystemTime};

use eframe::egui::{self, Context, Label};

use ratatui::prelude::*;
use ratframe::*;
mod app;
mod ui;

use crate::app::RatApp;

#[cfg(not(target_arch = "wasm32"))]
use ratframe::native_setup;

#[cfg(target_arch = "wasm32")]
use ratframe::wasm_setup;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_setup(DemoApp::default());
}
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    native_setup(DemoApp::default())
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

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Regular".into()),
        vec!["Regular".to_owned()],
    );
    fonts.font_data.insert(
        "Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/Iosevka-Bold.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Bold".into()),
        vec!["Bold".to_owned()],
    );

    fonts.font_data.insert(
        "Oblique".to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/Iosevka-Oblique.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Oblique".into()),
        vec!["Oblique".to_owned()],
    );

    fonts.font_data.insert(
        "BoldOblique".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../../assets/fonts/Iosevka-BoldOblique.ttf"
        )),
    );
    fonts.families.insert(
        egui::FontFamily::Name("BoldOblique".into()),
        vec!["BoldOblique".to_owned()],
    );

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub struct DemoApp {
    terminal: Terminal<RataguiBackend>,
    app: RatApp<'static>,
    tick_rate: Duration,
    last_tick: Instant,
}

impl NewCC for DemoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

impl Default for DemoApp {
    fn default() -> Self {
        //   setup_custom_fonts(&cc.egui_ctx);
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
