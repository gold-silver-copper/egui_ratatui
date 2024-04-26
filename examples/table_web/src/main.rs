use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use ratframe::NewCC;
use ratframe::RataguiBackend;
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;
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
        self.terminal.draw(|f| ui(f, &mut self.app));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::H)) {
                self.app.previous_color();
            }
            if ui.input(|i| i.key_released(egui::Key::J)) {
                self.app.next();
            }
            if ui.input(|i| i.key_released(egui::Key::K)) {
                self.app.previous();
            }
            if ui.input(|i| i.key_released(egui::Key::L)) {
                self.app.next_color();
            }
        });
    }
}

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct Data {
    name: String,
    address: String,
    email: String,
}

impl Data {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.name, &self.address, &self.email]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }
}

struct App {
    state: TableState,
    items: Vec<Data>,
    longest_item_lens: (u16, u16, u16), // order is (name, address, email)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        let data_vec = generate_fake_names();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }
}

fn generate_fake_names() -> Vec<Data> {
    (0..20)
        .map(|ph| {
            let name = format!("Placeholder Name {ph}");
            let address = format!("Placeholder Address {ph}");
            let email = format!("email{ph}@meowmail.org");

            Data {
                name,
                address,
                email,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect_vec()
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    app.set_colors();

    render_table(f, app, rects[0]);

    render_scrollbar(f, app, rects[0]);

    render_footer(f, app, rects[1]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = ["Name", "Address", "Email"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(app.longest_item_lens.0 + 1),
            Constraint::Min(app.longest_item_lens.1 + 1),
            Constraint::Min(app.longest_item_lens.2),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(Data::name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(Data::address)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let email_len = items
        .iter()
        .map(Data::email)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, address_len as u16, email_len as u16)
}

fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, area);
}
