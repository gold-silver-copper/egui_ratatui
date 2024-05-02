use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use ratframe::RataguiBackend;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}
// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(mut contexts: EguiContexts, mut termres: ResMut<BevyTerminal<RataguiBackend>>) {
    termres
        .terminal
        .draw(|frame| {
            let area = frame.size();
            let textik = format!("Hello bevy ! the window area is {}", area);
            frame.render_widget(
                Paragraph::new(textik)
                    .white()
                    .on_blue()
                    .wrap(Wrap { trim: false }),
                area,
            );
        })
        .expect("epic fail");

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.add(termres.terminal.backend_mut());
    });
}
// Create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let backend = RataguiBackend::new(100, 50);
        let mut terminal = Terminal::new(backend).unwrap();
        BevyTerminal { terminal }
    }
}
