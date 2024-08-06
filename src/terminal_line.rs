use std::sync::Arc;

use egui::*;

use egui::text_selection::LabelSelectionState;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct TerminalLine {
    text: WidgetText,
}

impl TerminalLine {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }
}

impl Widget for TerminalLine {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut layout_job =
            self.text
                .into_layout_job(ui.style(), FontSelection::Default, egui::Align::Min); //FontSelection::Style(egui::TextStyle::Monospace)

        let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));
        let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::click_and_drag());

 //  let rect = ui.cursor();

        let boop = ui.allocate_ui(galley.size(), |ui|  ui.painter().add(epaint::TextShape::new(
            rect.left_top(),
            galley.clone(),
            ui.style().visuals.text_color(),
        )));
        let response = boop.response;



        if ui.is_rect_visible(response.rect) {
            if galley.elided {
                // Show the full (non-elided) text on hover:
            //    response = response.on_hover_text(galley.text());
                println!("WTF");
            }

         
        }

        response
    }
}
