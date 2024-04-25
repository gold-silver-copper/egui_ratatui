use indoc::indoc;
use itertools::izip;
use ratatui::{prelude::*, widgets::Paragraph};
use ratframe::NewCC;
use ratframe::RataguiBackend;
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
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        Self { terminal: terminal }
    }
}

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal
            .draw(|frame| {
                frame.render_widget(Paragraph::new(logo()), frame.size());
            })
            .unwrap();

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

/// A fun example of using half block characters to draw a logo
#[allow(clippy::many_single_char_names)]
fn logo() -> String {
    let r = indoc! {"
            ▄▄▄
            █▄▄▀
            █  █
        "};
    let a = indoc! {"
             ▄▄
            █▄▄█
            █  █
        "};
    let t = indoc! {"
            ▄▄▄
             █
             █
        "};
    let u = indoc! {"
            ▄  ▄
            █  █
            ▀▄▄▀
        "};
    let i = indoc! {"
            ▄
            █
            █
        "};
    izip!(r.lines(), a.lines(), t.lines(), u.lines(), i.lines())
        .map(|(r, a, t, u, i)| format!("{r:5}{a:5}{t:4}{a:5}{t:4}{u:5}{i:5}"))
        .collect::<Vec<_>>()
        .join("\n")
}
