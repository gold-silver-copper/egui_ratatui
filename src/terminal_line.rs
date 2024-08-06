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


        let (response, painter) = ui.allocate_painter(galley.size(),Sense::hover() );

        painter.galley(
            response.rect.left_top(),
            galley.clone(),
            ui.style().visuals.text_color(),
        );
      

        
        response
    }
}
