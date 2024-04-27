use egui::Ui;
use ratatui::{prelude::*, symbols::scrollbar, widgets::*};
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
    tick_rate: Duration,
    app: App,
    last_tick: Instant,
}

//l
impl Default for HelloApp {
    fn default() -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        let tick_rate = Duration::from_millis(250);
        let mut app = App::default();

        let mut last_tick = Instant::now();

        Self {
            terminal,
            tick_rate,
            app,
            last_tick,
        }
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
        let tick_rate = Duration::from_millis(250);
        let mut app = App::default();

        let mut last_tick = Instant::now();

        Self {
            terminal,
            tick_rate,
            app,
            last_tick,
        }
    }
    fn canvas_id() -> String {
        "scrollbar".into()
    }
}

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal.draw(|f| ui(f, &mut self.app)).unwrap();

        let timeout = self.tick_rate.saturating_sub(self.last_tick.elapsed());
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            self.app.handle_events(ui);
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

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl App {
    fn handle_events(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_released(egui::Key::Q)) {
            panic!("HAVE A NICE WEEK");
        }
        if ui.input(|i| i.key_released(egui::Key::J)) {
            self.vertical_scroll = self.vertical_scroll.saturating_add(1);
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        }
        if ui.input(|i| i.key_released(egui::Key::H)) {
            self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
            self.horizontal_scroll_state = self
                .horizontal_scroll_state
                .position(self.horizontal_scroll);
        }
        if ui.input(|i| i.key_released(egui::Key::K)) {
            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
        }
        if ui.input(|i| i.key_released(egui::Key::L)) {
            self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
            self.horizontal_scroll_state = self
                .horizontal_scroll_state
                .position(self.horizontal_scroll);
        }
    }
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(size);

    let text = vec![
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.clone()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
        ]),
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.clone()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(Masked::new("password", '*'), Style::new().fg(Color::Red)),
        ]),
    ];
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(text.len());
    app.horizontal_scroll_state = app.horizontal_scroll_state.content_length(long_line.len());

    let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());

    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title("Use h j k l or ◄ ▲ ▼ ► to scroll ".bold());
    f.render_widget(title, chunks[0]);

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block("Vertical scrollbar with arrows"))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[1]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Vertical scrollbar without arrows, without track symbol and mirrored",
        ))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[2]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        chunks[2].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar with only begin arrow & custom thumb symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[3]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("🬋")
            .end_symbol(None),
        chunks[3].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar without arrows & custom thumb and track symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[4]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("░")
            .track_symbol(Some("─")),
        chunks[4].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );
}
