use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::Paragraph,
};
use ratframe::RataguiBackend;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    // Normally you do not want to create a new terminal every frame, but I am doing it here for simplicity
    // I reccomend creating either a resource or an entity to hold the terminal instead in a real program
    let boop = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(boop).unwrap();
    terminal
    .draw(|frame| {
        let area = frame.size();
        frame.render_widget(Paragraph::new("Hello Rataguiii").white().on_blue(), area);
    })
    .expect("epic fail");

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
       
        ui.add(terminal.backend_mut());
    });
}
