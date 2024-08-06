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
        let (rect, mut response) = ui.allocate_exact_size(galley.size(), Sense::hover());
        let galley_pos =  rect.left_top();
       

        if ui.is_rect_visible(response.rect) {
            if galley.elided {
                // Show the full (non-elided) text on hover:
                response = response.on_hover_text(galley.text());
                println!("WTF");
            }

            ui.painter().add(epaint::TextShape::new(
                galley_pos,
                galley.clone(),
                ui.style().visuals.text_color(),
            ));
        }

        response
    }
}
