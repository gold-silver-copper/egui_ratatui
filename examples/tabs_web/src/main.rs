use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use ratframe::NewCC;
use ratframe::RataguiBackend;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
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
        let mut backend = RataguiBackend::new(100, 100);

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

impl eframe::App for HelloApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //call repaint here so that app runs continuously, remove if you dont need that
        ctx.request_repaint();
        self.app.draw(&mut self.terminal);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.terminal.backend_mut());

            if ui.input(|i| i.key_released(egui::Key::Q)) {
                panic!("HAVE A NICE WEEK");
            }
            if ui.input(|i| i.key_released(egui::Key::L)) {
                self.app.next_tab();
            }
            if ui.input(|i| i.key_released(egui::Key::H)) {
                self.app.previous_tab();
            }
        });
    }
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Tab 1")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Tab 3")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}

impl App {
    fn draw(&self, terminal: &mut Terminal<impl Backend>) {
        terminal.draw(|frame| frame.render_widget(self, frame.size()));
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::BLUE,
            Self::Tab2 => tailwind::EMERALD,
            Self::Tab3 => tailwind::INDIGO,
            Self::Tab4 => tailwind::RED,
        }
    }
}
