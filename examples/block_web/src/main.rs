use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
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

type Terminal = ratatui::Terminal<RataguiBackend>;
pub struct HelloApp {
    terminal: Terminal,
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
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);

    let paragraph = placeholder_paragraph();

    render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
    render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
    render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
    render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
    render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
    render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);

    render_border_type(&paragraph, BorderType::Plain, frame, layout[3][0]);
    render_border_type(&paragraph, BorderType::Rounded, frame, layout[3][1]);
    render_border_type(&paragraph, BorderType::Double, frame, layout[4][0]);
    render_border_type(&paragraph, BorderType::Thick, frame, layout[4][1]);

    render_styled_block(&paragraph, frame, layout[5][0]);
    render_styled_borders(&paragraph, frame, layout[5][1]);
    render_styled_title(&paragraph, frame, layout[6][0]);
    render_styled_title_content(&paragraph, frame, layout[6][1]);
    render_multiple_titles(&paragraph, frame, layout[7][0]);
    render_multiple_title_positions(&paragraph, frame, layout[7][1]);
    render_padding(&paragraph, frame, layout[8][0]);
    render_nested_blocks(&paragraph, frame, layout[8][1]);
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::vertical([Constraint::Max(4); 9]);
    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect_vec();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_border_type(
    paragraph: &Paragraph,
    border_type: BorderType,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}
fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().blue().on_white().bold().italic())
        .title("Styled borders");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().blue().on_white().bold().italic())
        .title("Styled block");
    frame.render_widget(paragraph.clone().block(block), area);
}

// Note: this currently renders incorrectly, see https://github.com/ratatui-org/ratatui/issues/349
fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Styled title")
        .title_style(Style::new().blue().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        "Styled ".blue().on_white().bold().italic(),
        "title content".red().on_white().bold().italic(),
    ]);
    let block = Block::new().borders(Borders::ALL).title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Multiple".blue().on_white().bold().italic())
        .title("Titles".red().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title(
            Title::from("top left")
                .position(Position::Top)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("top center")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("top right")
                .position(Position::Top)
                .alignment(Alignment::Right),
        )
        .title(
            Title::from("bottom left")
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("bottom center")
                .position(Position::Bottom)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("bottom right")
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        );
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Padding")
        .padding(Padding::new(5, 10, 1, 2));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let outer_block = Block::new().borders(Borders::ALL).title("Outer block");
    let inner_block = Block::new().borders(Borders::ALL).title("Inner block");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    frame.render_widget(paragraph.clone().block(inner_block), inner);
}
