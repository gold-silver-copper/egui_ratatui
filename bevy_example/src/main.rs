use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Paragraph,Block,Borders,Wrap},
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

fn ui_example_system(mut contexts: EguiContexts, mut myterm: ResMut<BevyTerminal<RataguiBackend>>,) {
    // Normally you do not want to create a new terminal every frame, but I am doing it here for simplicity
    // I reccomend creating either a resource or an entity to hold the terminal instead in a real program
   
    

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
       
        ui.add(myterm.terminal.backend_mut());
    });
    myterm.terminal
    .draw(|frame| {
        let area = frame.size();
        let textik = format!("Hello bevy ! the window area is {}",area);
        frame.render_widget(Paragraph::new(textik).white().on_blue().wrap(Wrap { trim: true }), area);
    })
    .expect("epic fail");
}



#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal: Terminal<RataguiBackend>,
   
}

// custom implementation for unusual values
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let backend = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    BevyTerminal{terminal}
    }
}