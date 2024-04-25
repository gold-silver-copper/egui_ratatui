use egui::Ui;
use ratatui::{
    layout::{Constraint::*, Flex},
    prelude::*,
    style::palette::tailwind,
    symbols::line,
    widgets::{block::Title, *},
};
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
        // assuming the user changes spacing about a 100 times or so
        Layout::init_cache(EXAMPLE_DATA.len() * SelectedTab::iter().len() * 100);
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
        // assuming the user changes spacing about a 100 times or so
        Layout::init_cache(EXAMPLE_DATA.len() * SelectedTab::iter().len() * 100);

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

const EXAMPLE_DATA: &[(&str, &[Constraint])] = &[
    (
        "Min(u16) takes any excess space always",
        &[Length(10), Min(10), Max(10), Percentage(10), Ratio(1,10)],
    ),
    (
        "Fill(u16) takes any excess space always",
        &[Length(20), Percentage(20), Ratio(1, 5), Fill(1)],
    ),
    (
        "Here's all constraints in one line",
        &[Length(10), Min(10), Max(10), Percentage(10), Ratio(1,10), Fill(1)],
    ),
    (
        "",
        &[Max(50), Min(50)],
    ),
    (
        "",
        &[Max(20), Length(10)],
    ),
    (
        "",
        &[Max(20), Length(10)],
    ),
    (
        "Min grows always but also allows Fill to grow",
        &[Percentage(50), Fill(1), Fill(2), Min(50)],
    ),
    (
        "In `Legacy`, the last constraint of lowest priority takes excess space",
        &[Length(20), Length(20), Percentage(20)],
    ),
    ("", &[Length(20), Percentage(20), Length(20)]),
    ("A lowest priority constraint will be broken before a high priority constraint", &[Ratio(1,4), Percentage(20)]),
    ("`Length` is higher priority than `Percentage`", &[Percentage(20), Length(10)]),
    ("`Min/Max` is higher priority than `Length`", &[Length(10), Max(20)]),
    ("", &[Length(100), Min(20)]),
    ("`Length` is higher priority than `Min/Max`", &[Max(20), Length(10)]),
    ("", &[Min(20), Length(90)]),
    ("Fill is the lowest priority and will fill any excess space", &[Fill(1), Ratio(1, 4)]),
    ("Fill can be used to scale proportionally with other Fill blocks", &[Fill(1), Percentage(20), Fill(2)]),
    ("", &[Ratio(1, 3), Percentage(20), Ratio(2, 3)]),
    ("Legacy will stretch the last lowest priority constraint\nStretch will only stretch equal weighted constraints", &[Length(20), Length(15)]),
    ("", &[Percentage(20), Length(15)]),
    ("`Fill(u16)` fills up excess space, but is lower priority to spacers.\ni.e. Fill will only have widths in Flex::Stretch and Flex::Legacy", &[Fill(1), Fill(1)]),
    ("", &[Length(20), Length(20)]),
    (
        "When not using `Flex::Stretch` or `Flex::Legacy`,\n`Min(u16)` and `Max(u16)` collapse to their lowest values",
        &[Min(20), Max(20)],
    ),
    (
        "",
        &[Max(20)],
    ),
    ("", &[Min(20), Max(20), Length(20), Length(20)]),
    ("", &[Fill(0), Fill(0)]),
    (
        "`Fill(1)` can be to scale with respect to other `Fill(2)`",
        &[Fill(1), Fill(2)],
    ),
    (
        "",
        &[Fill(1), Min(10), Max(10), Fill(2)],
    ),
    (
        "`Fill(0)` collapses if there are other non-zero `Fill(_)`\nconstraints. e.g. `[Fill(0), Fill(0), Fill(1)]`:",
        &[
            Fill(0),
            Fill(0),
            Fill(1),
        ],
    ),
];

#[derive(Default, Clone, Copy)]
struct App {
    selected_tab: SelectedTab,
    scroll_offset: u16,
    spacing: u16,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Example {
    constraints: Vec<Constraint>,
    description: String,
    flex: Flex,
    spacing: u16,
}

/// Tabs for the different layouts
///
/// Note: the order of the variants will determine the order of the tabs this uses several derive
/// macros from the `strum` crate to make it easier to iterate over the variants.
/// (`FromRepr`,`Display`,`EnumIter`).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, FromRepr, Display, EnumIter)]
enum SelectedTab {
    #[default]
    Legacy,
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
}

impl App {
    fn draw(self, terminal: &mut Terminal<impl Backend>) {
        terminal.draw(|frame| frame.render_widget(self, frame.size()));
    }

    fn handle_events(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_released(egui::Key::Q)) {
            panic!("HAVE A NICE WEEK");
        }
        if ui.input(|i| i.key_released(egui::Key::L)) {
            self.next();
        }
        if ui.input(|i| i.key_released(egui::Key::H)) {
            self.previous();
        }
        if ui.input(|i| i.key_released(egui::Key::J)) {
            self.down();
        }
        if ui.input(|i| i.key_released(egui::Key::K)) {
            self.up();
        }
        if ui.input(|i| i.key_released(egui::Key::G)) {
            self.top();
        }
        if ui.input(|i| i.key_released(egui::Key::F)) {
            self.bottom();
        }
        if ui.input(|i| i.key_released(egui::Key::Plus)) {
            self.increment_spacing();
        }
        if ui.input(|i| i.key_released(egui::Key::Minus)) {
            self.decrement_spacing();
        }
        // Char('-') => self.decrement_spacing(),
    }

    fn next(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn down(&mut self) {
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(1)
            .min(max_scroll_offset());
    }

    fn top(&mut self) {
        self.scroll_offset = 0;
    }

    fn bottom(&mut self) {
        self.scroll_offset = max_scroll_offset();
    }

    fn increment_spacing(&mut self) {
        self.spacing = self.spacing.saturating_add(1);
    }

    fn decrement_spacing(&mut self) {
        self.spacing = self.spacing.saturating_sub(1);
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

// when scrolling, make sure we don't scroll past the last example
fn max_scroll_offset() -> u16 {
    example_height()
        - EXAMPLE_DATA
            .last()
            .map_or(0, |(desc, _)| get_description_height(desc) + 4)
}

/// The height of all examples combined
///
/// Each may or may not have a title so we need to account for that.
fn example_height() -> u16 {
    EXAMPLE_DATA
        .iter()
        .map(|(desc, _)| get_description_height(desc) + 4)
        .sum()
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(3), Length(1), Fill(0)]);
        let [tabs, axis, demo] = layout.areas(area);
        self.tabs().render(tabs, buf);
        let scroll_needed = self.render_demo(demo, buf);
        let axis_width = if scroll_needed {
            axis.width.saturating_sub(1)
        } else {
            axis.width
        };
        Self::axis(axis_width, self.spacing).render(axis, buf);
    }
}

impl App {
    fn tabs(self) -> impl Widget {
        let tab_titles = SelectedTab::iter().map(SelectedTab::to_tab_title);
        let block = Block::new()
            .title(Title::from("Flex Layouts ".bold()))
            .title(" Use ◄ ► to change tab, ▲ ▼  to scroll, - + to change spacing ");
        Tabs::new(tab_titles)
            .block(block)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_tab as usize)
            .divider(" ")
            .padding("", "")
    }

    /// a bar like `<----- 80 px (gap: 2 px)? ----->`
    fn axis(width: u16, spacing: u16) -> impl Widget {
        let width = width as usize;
        // only show gap when spacing is not zero
        let label = if spacing != 0 {
            format!("{width} px (gap: {spacing} px)")
        } else {
            format!("{width} px")
        };
        let bar_width = width.saturating_sub(2); // we want to `<` and `>` at the ends
        let width_bar = format!("<{label:-^bar_width$}>");
        Paragraph::new(width_bar.dark_gray()).centered()
    }

    /// Render the demo content
    ///
    /// This function renders the demo content into a separate buffer and then splices the buffer
    /// into the main buffer. This is done to make it possible to handle scrolling easily.
    ///
    /// Returns bool indicating whether scroll was needed
    #[allow(clippy::cast_possible_truncation)]
    fn render_demo(self, area: Rect, buf: &mut Buffer) -> bool {
        // render demo content into a separate buffer so all examples fit we add an extra
        // area.height to make sure the last example is fully visible even when the scroll offset is
        // at the max
        let height = example_height();
        let demo_area = Rect::new(0, 0, area.width, height);
        let mut demo_buf = Buffer::empty(demo_area);

        let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
        let content_area = if scrollbar_needed {
            Rect {
                width: demo_area.width - 1,
                ..demo_area
            }
        } else {
            demo_area
        };

        let mut spacing = self.spacing;
        self.selected_tab
            .render(content_area, &mut demo_buf, &mut spacing);

        let visible_content = demo_buf
            .content
            .into_iter()
            .skip((area.width * self.scroll_offset) as usize)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            *buf.get_mut(area.x + x, area.y + y) = cell;
        }

        if scrollbar_needed {
            let area = area.intersection(buf.area);
            let mut state = ScrollbarState::new(max_scroll_offset() as usize)
                .position(self.scroll_offset as usize);
            Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
        }
        scrollbar_needed
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

    /// Convert a `SelectedTab` into a `Line` to display it by the `Tabs` widget.
    fn to_tab_title(value: Self) -> Line<'static> {
        use tailwind::*;
        let text = value.to_string();
        let color = match value {
            Self::Legacy => ORANGE.c400,
            Self::Start => SKY.c400,
            Self::Center => SKY.c300,
            Self::End => SKY.c200,
            Self::SpaceAround => INDIGO.c400,
            Self::SpaceBetween => INDIGO.c300,
        };
        format!(" {text} ").fg(color).bg(Color::Black).into()
    }
}

impl StatefulWidget for SelectedTab {
    type State = u16;
    fn render(self, area: Rect, buf: &mut Buffer, spacing: &mut Self::State) {
        let spacing = *spacing;
        match self {
            Self::Legacy => Self::render_examples(area, buf, Flex::Legacy, spacing),
            Self::Start => Self::render_examples(area, buf, Flex::Start, spacing),
            Self::Center => Self::render_examples(area, buf, Flex::Center, spacing),
            Self::End => Self::render_examples(area, buf, Flex::End, spacing),
            Self::SpaceAround => Self::render_examples(area, buf, Flex::SpaceAround, spacing),
            Self::SpaceBetween => Self::render_examples(area, buf, Flex::SpaceBetween, spacing),
        }
    }
}

impl SelectedTab {
    fn render_examples(area: Rect, buf: &mut Buffer, flex: Flex, spacing: u16) {
        let heights = EXAMPLE_DATA
            .iter()
            .map(|(desc, _)| get_description_height(desc) + 4);
        let areas = Layout::vertical(heights).flex(Flex::Start).split(area);
        for (area, (description, constraints)) in areas.iter().zip(EXAMPLE_DATA.iter()) {
            Example::new(constraints, description, flex, spacing).render(*area, buf);
        }
    }
}

impl Example {
    fn new(constraints: &[Constraint], description: &str, flex: Flex, spacing: u16) -> Self {
        Self {
            constraints: constraints.into(),
            description: description.into(),
            flex,
            spacing,
        }
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_height = get_description_height(&self.description);
        let layout = Layout::vertical([Length(title_height), Fill(0)]);
        let [title, illustrations] = layout.areas(area);

        let (blocks, spacers) = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .spacing(self.spacing)
            .split_with_spacers(illustrations);

        if !self.description.is_empty() {
            Paragraph::new(
                self.description
                    .split('\n')
                    .map(|s| format!("// {s}").italic().fg(tailwind::SLATE.c400))
                    .map(Line::from)
                    .collect::<Vec<Line>>(),
            )
            .render(title, buf);
        }

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            Self::illustration(*constraint, block.width).render(*block, buf);
        }

        for spacer in spacers.iter() {
            Self::render_spacer(*spacer, buf);
        }
    }
}

impl Example {
    fn render_spacer(spacer: Rect, buf: &mut Buffer) {
        if spacer.width > 1 {
            let corners_only = symbols::border::Set {
                top_left: line::NORMAL.top_left,
                top_right: line::NORMAL.top_right,
                bottom_left: line::NORMAL.bottom_left,
                bottom_right: line::NORMAL.bottom_right,
                vertical_left: " ",
                vertical_right: " ",
                horizontal_top: " ",
                horizontal_bottom: " ",
            };
            Block::bordered()
                .border_set(corners_only)
                .border_style(Style::reset().dark_gray())
                .render(spacer, buf);
        } else {
            Paragraph::new(Text::from(vec![
                Line::from(""),
                Line::from("│"),
                Line::from("│"),
                Line::from(""),
            ]))
            .style(Style::reset().dark_gray())
            .render(spacer, buf);
        }
        let width = spacer.width;
        let label = if width > 4 {
            format!("{width} px")
        } else if width > 2 {
            format!("{width}")
        } else {
            String::new()
        };
        let text = Text::from(vec![
            Line::raw(""),
            Line::raw(""),
            Line::styled(label, Style::reset().dark_gray()),
        ]);
        Paragraph::new(text)
            .style(Style::reset().dark_gray())
            .alignment(Alignment::Center)
            .render(spacer, buf);
    }

    fn illustration(constraint: Constraint, width: u16) -> impl Widget {
        let main_color = color_for_constraint(constraint);
        let fg_color = Color::White;
        let title = format!("{constraint}");
        let content = format!("{width} px");
        let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(main_color).reversed())
            .style(Style::default().fg(fg_color).bg(main_color));
        Paragraph::new(text).centered().block(block)
    }
}

const fn color_for_constraint(constraint: Constraint) -> Color {
    use tailwind::*;
    match constraint {
        Constraint::Min(_) => BLUE.c900,
        Constraint::Max(_) => BLUE.c800,
        Constraint::Length(_) => SLATE.c700,
        Constraint::Percentage(_) => SLATE.c800,
        Constraint::Ratio(_, _) => SLATE.c900,
        Constraint::Fill(_) => SLATE.c950,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn get_description_height(s: &str) -> u16 {
    if s.is_empty() {
        0
    } else {
        s.split('\n').count() as u16
    }
}
