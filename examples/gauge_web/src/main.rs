use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::Title, Block, Borders, Gauge, Padding, Paragraph},
};

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
    fn canvas_id() -> String {
        "gauge".into()
    }
}

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.app.draw(&mut self.terminal);

        self.app
            .update(self.terminal.size().expect("cant get term size").width);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::Space)) {
                self.app.start();
            }
            //KeyCode::Char(c) => app.on_key(c),
        });
    }
}

const GAUGE1_COLOR: Color = tailwind::RED.c800;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;
const GAUGE3_COLOR: Color = tailwind::BLUE.c800;
const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress_columns: u16,
    progress1: u16,
    progress2: f64,
    progress3: f64,
    progress4: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Started,
    Quitting,
}

impl App {
    fn draw(&self, terminal: &mut Terminal<impl Backend>) {
        terminal.draw(|f| f.render_widget(self, f.size()));
    }

    fn update(&mut self, terminal_width: u16) {
        if self.state != AppState::Started {
            return;
        }

        // progress1 and progress2 help show the difference between ratio and percentage measuring
        // the same thing, but converting to either a u16 or f64. Effectively, we're showing the
        // difference between how a continuous gauge acts for floor and rounded values.
        self.progress_columns = (self.progress_columns + 1).clamp(0, terminal_width);
        self.progress1 = self.progress_columns * 100 / terminal_width;
        self.progress2 = f64::from(self.progress_columns) * 100.0 / f64::from(terminal_width);

        // progress3 and progress4 similarly show the difference between unicode and non-unicode
        // gauges measuring the same thing.
        self.progress3 = (self.progress3 + 0.1).clamp(40.0, 100.0);
        self.progress4 = (self.progress4 + 0.1).clamp(40.0, 100.0);
    }

    fn start(&mut self) {
        self.state = AppState::Started;
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    #[allow(clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let layout = Layout::vertical([Length(2), Min(0), Length(1)]);
        let [header_area, gauge_area, footer_area] = layout.areas(area);

        let layout = Layout::vertical([Ratio(1, 4); 4]);
        let [gauge1_area, gauge2_area, gauge3_area, gauge4_area] = layout.areas(gauge_area);

        render_header(header_area, buf);
        render_footer(footer_area, buf);

        self.render_gauge1(gauge1_area, buf);
        self.render_gauge2(gauge2_area, buf);
        self.render_gauge3(gauge3_area, buf);
        self.render_gauge4(gauge4_area, buf);
    }
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Ratatui Gauge Example")
        .bold()
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Press SPACE to start")
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .bold()
        .render(area, buf);
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with percentage");
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE1_COLOR)
            .percent(self.progress1)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio and custom label");
        let label = Span::styled(
            format!("{:.1}/100", self.progress2),
            Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
        );
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE2_COLOR)
            .ratio(self.progress2 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (no unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE3_COLOR)
            .ratio(self.progress3 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress4 / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::default()
        .title(title)
        .borders(Borders::NONE)
        .fg(CUSTOM_LABEL_COLOR)
        .padding(Padding::vertical(1))
}
