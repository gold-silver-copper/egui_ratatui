use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use egui_ratatui::RataguiBackend;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use soft_ratatui::embedded_graphics_unicodefonts::{
    mono_8x13_atlas, mono_8x13_bold_atlas, mono_8x13_italic_atlas,
};
use soft_ratatui::{EmbeddedGraphics, SoftBackend};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<EguiTerminal>()
        .add_systems(Startup, setup_camera_system)
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, ui_example_system)
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
                    .on_blue()
                    .wrap(Wrap { trim: false }),
                area,
            );
        })
        .expect("epic fail");

    egui::Window::new("Hello").show(contexts.ctx_mut().unwrap(), |ui| {
        ui.add(termres.backend_mut());
    });
}

#[derive(Resource, Deref, DerefMut)]
struct EguiTerminal(Terminal<RataguiBackend<EmbeddedGraphics>>);

impl Default for EguiTerminal {
    fn default() -> Self {
        let font_regular = mono_8x13_atlas();
        let font_italic = mono_8x13_italic_atlas();
        let font_bold = mono_8x13_bold_atlas();
        let soft_backend = SoftBackend::<EmbeddedGraphics>::new(
            100,
            50,
            font_regular,
            Some(font_bold),
            Some(font_italic),
        );
        let backend = RataguiBackend::new("soft_rat", soft_backend);
        Self(Terminal::new(backend).unwrap())
    }
}
fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}
