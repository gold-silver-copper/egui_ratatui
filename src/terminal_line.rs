use std::sync::Arc;

use egui::*;

use egui::text::LayoutJob;
use egui::text_selection::LabelSelectionState;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct TerminalLine {
    job: LayoutJob,
}

impl TerminalLine {
    pub fn new(job: LayoutJob) -> Self {
        Self { job}
    }

    pub fn job(self) -> LayoutJob {
        self.job
    }
}

impl Widget for TerminalLine {
    fn ui(self, ui: &mut Ui) -> Response {

        
    

        let galley = ui.fonts(|fonts| fonts.layout_job(self.job()));

       /* let boop = ui.allocate_ui(galley.size(), |ui| {  ui.painter().add(
            epaint::TextShape::new(galley.rect.left_top(), galley.clone(), ui.style().visuals.text_color())
              
        );});
 */

// let bigger = galley.size() + vec2(300.0, -1.0);
      


        let (response, painter) = ui.allocate_painter(galley.size(),Sense::hover() );

        painter.galley(
            response.rect.left_top(),
            galley.clone(),
            ui.style().visuals.text_color(),
        );
      
       

        
        response
    }
}
