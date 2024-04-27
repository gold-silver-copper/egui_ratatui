use itertools::Itertools;
use ratatui::{
    layout::Constraint::*,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
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
    fn canvas_id() -> String {
        "layout".into()
    }
}

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.terminal.draw(ui).expect("epic fail");

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

#[allow(clippy::too_many_lines)]
fn ui(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Length(4),  // text
        Length(50), // examples
        Min(0),     // fills remaining space
    ]);
    let [text_area, examples_area, _] = vertical.areas(frame.size());

    // title
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Horizontal Layout Example. Press q to quit".dark_gray()).centered(),
            Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),
            Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),
            Line::from("Note: constraint labels that don't fit are truncated"),
        ]),
        text_area,
    );

    let example_rows = Layout::vertical([
        Length(9),
        Length(9),
        Length(9),
        Length(9),
        Length(9),
        Min(0), // fills remaining space
    ])
    .split(examples_area);
    let example_areas = example_rows
        .iter()
        .flat_map(|area| {
            Layout::horizontal([
                Length(14),
                Length(14),
                Length(14),
                Length(14),
                Length(14),
                Min(0), // fills remaining space
            ])
            .split(*area)
            .iter()
            .copied()
            .take(5) // ignore Min(0)
            .collect_vec()
        })
        .collect_vec();

    // the examples are a cartesian product of the following constraints
    // e.g. Len/Len, Len/Min, Len/Max, Len/Perc, Len/Ratio, Min/Len, Min/Min, ...
    let examples = [
        (
            "Len",
            vec![
                Length(0),
                Length(2),
                Length(3),
                Length(6),
                Length(10),
                Length(15),
            ],
        ),
        (
            "Min",
            vec![Min(0), Min(2), Min(3), Min(6), Min(10), Min(15)],
        ),
        (
            "Max",
            vec![Max(0), Max(2), Max(3), Max(6), Max(10), Max(15)],
        ),
        (
            "Perc",
            vec![
                Percentage(0),
                Percentage(25),
                Percentage(50),
                Percentage(75),
                Percentage(100),
                Percentage(150),
            ],
        ),
        (
            "Ratio",
            vec![
                Ratio(0, 4),
                Ratio(1, 4),
                Ratio(2, 4),
                Ratio(3, 4),
                Ratio(4, 4),
                Ratio(6, 4),
            ],
        ),
    ];

    for (i, (a, b)) in examples
        .iter()
        .cartesian_product(examples.iter())
        .enumerate()
    {
        let (name_a, examples_a) = a;
        let (name_b, examples_b) = b;
        let constraints = examples_a
            .iter()
            .copied()
            .zip(examples_b.iter().copied())
            .collect_vec();
        render_example_combination(
            frame,
            example_areas[i],
            &format!("{name_a}/{name_b}"),
            constraints,
        );
    }
}

/// Renders a single example box
fn render_example_combination(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    constraints: Vec<(Constraint, Constraint)>,
) {
    let block = Block::default()
        .title(title.gray())
        .style(Style::reset())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let layout = Layout::vertical(vec![Length(1); constraints.len() + 1]).split(inner);
    for (i, (a, b)) in constraints.into_iter().enumerate() {
        render_single_example(frame, layout[i], vec![a, b, Min(0)]);
    }
    // This is to make it easy to visually see the alignment of the examples
    // with the constraints.
    frame.render_widget(Paragraph::new("123456789012"), layout[6]);
}

/// Renders a single example line
fn render_single_example(frame: &mut Frame, area: Rect, constraints: Vec<Constraint>) {
    let red = Paragraph::new(constraint_label(constraints[0])).on_red();
    let blue = Paragraph::new(constraint_label(constraints[1])).on_blue();
    let green = Paragraph::new("·".repeat(12)).on_green();
    let horizontal = Layout::horizontal(constraints);
    let [r, b, g] = horizontal.areas(area);
    frame.render_widget(red, r);
    frame.render_widget(blue, b);
    frame.render_widget(green, g);
}

fn constraint_label(constraint: Constraint) -> String {
    match constraint {
        Constraint::Ratio(a, b) => format!("{a}:{b}"),
        Constraint::Length(n)
        | Constraint::Min(n)
        | Constraint::Max(n)
        | Constraint::Percentage(n)
        | Constraint::Fill(n) => format!("{n}"),
    }
}
