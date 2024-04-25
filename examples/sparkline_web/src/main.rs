use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    SeedableRng,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Sparkline},
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
        let mut app = App::new();

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
        let mut backend = RataguiBackend::new(100, 100);
        backend.set_font_size(16);

        let mut terminal = Terminal::new(backend).unwrap();

        let tick_rate = Duration::from_millis(250);
        let mut app = App::new();

        let mut last_tick = Instant::now();

        Self {
            terminal,
            tick_rate,
            app,
            last_tick,
        }
    }
}

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal.draw(|f| ui(f, &self.app)).unwrap();

        let timeout = self.tick_rate.saturating_sub(self.last_tick.elapsed());
        if self.last_tick.elapsed() >= self.tick_rate {
            self.app.on_tick();
            self.last_tick = Instant::now();
        }

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

#[derive(Clone)]
struct RandomSignal {
    distribution: Uniform<u64>,
    rng: SmallRng,
}

impl RandomSignal {
    fn new(lower: u64, upper: u64) -> Self {
        Self {
            distribution: Uniform::new(lower, upper),
            rng: SmallRng::seed_from_u64(17),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

impl App {
    fn new() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        Self {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn on_tick(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(f.size());
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data1")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data1)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[0]);
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data2")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data2)
        .style(Style::default().bg(Color::Green));
    f.render_widget(sparkline, chunks[1]);
    // Multiline
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data3")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data3)
        .style(Style::default().fg(Color::Red));
    f.render_widget(sparkline, chunks[2]);
}
