use bevy::prelude::*;
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use egui_ratatui::RataguiBackend;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Block, Borders, Paragraph, Wrap},
};

static FONT_DATA: &[u8] = include_bytes!("../../assets/iosevka.ttf");
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<EguiTerminal>()
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(EguiContextPass, ui_example_system)
        .run();
}
// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(mut contexts: EguiContexts, mut termres: ResMut<EguiTerminal>) {
    termres
        .draw(|frame| {
            let area = frame.area();
            let textik = format!("Hello bevy! The window area is {}", area);
            frame.render_widget(
                Paragraph::new(textik)
                    .block(Block::new().title("Ratatui").borders(Borders::ALL))
                    .white()
                    .on_blue(),
                area,
            );
        })
        .expect("epic fail");

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.add(termres.backend_mut());
    });
}

#[derive(Resource, Deref, DerefMut)]
struct EguiTerminal(Terminal<RataguiBackend>);

impl Default for EguiTerminal {
    fn default() -> Self {
        let backend = RataguiBackend::new(10, 10, 16, FONT_DATA);
        //backend.set_font_size(12);
        Self(Terminal::new(backend).unwrap())
    }
}
