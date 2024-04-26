use ratatui::{
    prelude::*,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
};
use ratframe::RataguiBackend;
use web_time::{Duration, Instant};

use ratframe::NewCC;

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
    app: App<'static>,
    last_tick: Instant,
}

//l
impl Default for HelloApp {
    fn default() -> Self {
        //Creating the Ratatui backend/ Egui widget here
        let backend = RataguiBackend::new(100, 100);
        let mut terminal = Terminal::new(backend).unwrap();
        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new();
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
        backend.set_font_size(30);
        let mut terminal = Terminal::new(backend).unwrap();

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new();
        let mut last_tick = Instant::now();
        Self {
            terminal,
            tick_rate,
            app,
            last_tick,
        }
    }

    fn canvas_id() -> String {
        "barchart".into()
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

struct Company<'a> {
    revenue: [u64; 4],
    label: &'a str,
    bar_style: Style,
}

struct App<'a> {
    data: Vec<(&'a str, u64)>,
    months: [&'a str; 4],
    companies: [Company<'a>; 3],
}

const TOTAL_REVENUE: &str = "Total Revenue";

impl<'a> App<'a> {
    fn new() -> Self {
        App {
            data: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
                ("B15", 4),
                ("B16", 6),
                ("B17", 4),
                ("B18", 7),
                ("B19", 13),
                ("B20", 8),
                ("B21", 11),
                ("B22", 9),
                ("B23", 3),
                ("B24", 5),
            ],
            companies: [
                Company {
                    label: "Comp.A",
                    revenue: [9500, 12500, 5300, 8500],
                    bar_style: Style::default().fg(Color::Green),
                },
                Company {
                    label: "Comp.B",
                    revenue: [1500, 2500, 3000, 500],
                    bar_style: Style::default().fg(Color::Yellow),
                },
                Company {
                    label: "Comp.C",
                    revenue: [10500, 10600, 9000, 4200],
                    bar_style: Style::default().fg(Color::White),
                },
            ],
            months: ["Mars", "Apr", "May", "Jun"],
        }
    }

    fn on_tick(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [top, bottom] = vertical.areas(frame.size());
    let [left, right] = horizontal.areas(bottom);

    let barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    frame.render_widget(barchart, top);
    draw_bar_with_group_labels(frame, app, left);
    draw_horizontal_bars(frame, app, right);
}

#[allow(clippy::cast_precision_loss)]
fn create_groups<'a>(app: &'a App, combine_values_and_labels: bool) -> Vec<BarGroup<'a>> {
    app.months
        .iter()
        .enumerate()
        .map(|(i, &month)| {
            let bars: Vec<Bar> = app
                .companies
                .iter()
                .map(|c| {
                    let mut bar = Bar::default()
                        .value(c.revenue[i])
                        .style(c.bar_style)
                        .value_style(
                            Style::default()
                                .bg(c.bar_style.fg.unwrap())
                                .fg(Color::Black),
                        );

                    if combine_values_and_labels {
                        bar = bar.text_value(format!(
                            "{} ({:.1} M)",
                            c.label,
                            (c.revenue[i] as f64) / 1000.
                        ));
                    } else {
                        bar = bar
                            .text_value(format!("{:.1}", (c.revenue[i] as f64) / 1000.))
                            .label(c.label.into());
                    }
                    bar
                })
                .collect();
            BarGroup::default()
                .label(Line::from(month).centered())
                .bars(&bars)
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
fn draw_bar_with_group_labels(f: &mut Frame, app: &App, area: Rect) {
    const LEGEND_HEIGHT: u16 = 6;

    let groups = create_groups(app, false);

    let mut barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .bar_width(7)
        .group_gap(3);

    for group in groups {
        barchart = barchart.data(group);
    }

    f.render_widget(barchart, area);

    if area.height >= LEGEND_HEIGHT && area.width >= TOTAL_REVENUE.len() as u16 + 2 {
        let legend_width = TOTAL_REVENUE.len() as u16 + 2;
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: legend_width,
            y: area.y,
            x: area.right() - legend_width,
        };
        draw_legend(f, legend_area);
    }
}

#[allow(clippy::cast_possible_truncation)]
fn draw_horizontal_bars(f: &mut Frame, app: &App, area: Rect) {
    const LEGEND_HEIGHT: u16 = 6;

    let groups = create_groups(app, true);

    let mut barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .bar_width(1)
        .group_gap(1)
        .bar_gap(0)
        .direction(Direction::Horizontal);

    for group in groups {
        barchart = barchart.data(group);
    }

    f.render_widget(barchart, area);

    if area.height >= LEGEND_HEIGHT && area.width >= TOTAL_REVENUE.len() as u16 + 2 {
        let legend_width = TOTAL_REVENUE.len() as u16 + 2;
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: legend_width,
            y: area.y,
            x: area.right() - legend_width,
        };
        draw_legend(f, legend_area);
    }
}

fn draw_legend(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(Span::styled(
            TOTAL_REVENUE,
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        )),
        Line::from(Span::styled(
            "- Company A",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "- Company B",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(vec![Span::styled(
            "- Company C",
            Style::default().fg(Color::White),
        )]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
