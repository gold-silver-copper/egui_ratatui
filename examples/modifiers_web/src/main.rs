use itertools::Itertools;
use ratatui::{prelude::*, widgets::Paragraph};
use ratframe::NewCC;
use ratframe::RataguiBackend;
use std::iter::once;
use web_time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use ratframe::native_setup;

#[cfg(target_arch = "wasm32")]
use ratframe::wasm_setup;

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_setup(HelloApp::default());
}
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    native_setup(HelloApp::default())
}
pub struct HelloApp {
    terminal: Terminal<RataguiBackend>,
}

//l
impl Default for HelloApp {
    fn default() -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        Self { terminal: terminal }
    }
}

impl NewCC for HelloApp {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        //Creating the Ratatui backend/ Egui widget here
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

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal.draw(ui).unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

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

fn ui(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let [text_area, main_area] = vertical.areas(frame.size());
    frame.render_widget(
        Paragraph::new("Note: not all terminals support all modifiers")
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        text_area,
    );
    let layout = Layout::vertical([Constraint::Length(1); 50])
        .split(main_area)
        .iter()
        .flat_map(|area| {
            Layout::horizontal([Constraint::Percentage(20); 5])
                .split(*area)
                .to_vec()
        })
        .collect_vec();

    let colors = [
        Color::Black,
        Color::DarkGray,
        Color::Gray,
        Color::White,
        Color::Red,
    ];
    let all_modifiers = once(Modifier::empty())
        .chain(Modifier::all().iter())
        .collect_vec();
    let mut index = 0;
    for bg in &colors {
        for fg in &colors {
            for modifier in &all_modifiers {
                let modifier_name = format!("{modifier:11?}");
                let padding = (" ").repeat(12 - modifier_name.len());
                let paragraph = Paragraph::new(Line::from(vec![
                    modifier_name.fg(*fg).bg(*bg).add_modifier(*modifier),
                    padding.fg(*fg).bg(*bg).add_modifier(*modifier),
                    // This is a hack to work around a bug in VHS which is used for rendering the
                    // examples to gifs. The bug is that the background color of a paragraph seems
                    // to bleed into the next character.
                    ".".black().on_black(),
                ]));
                frame.render_widget(paragraph, layout[index]);
                index += 1;
            }
        }
    }
}
