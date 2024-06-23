use std::sync::Arc;

use egui::*;


#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct TerminalLine {
    text: WidgetText,
    wrap: Option<bool>,
    truncate: bool,
    sense: Option<Sense>,
    selectable: Option<bool>,
}

impl TerminalLine {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            truncate: false,
            sense: None,
            selectable: None,
        }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// If `true`, the text will wrap to stay within the max width of the [`Ui`].
    ///
    /// Calling `wrap` will override [`Self::truncate`].
    ///
    /// By default [`Self::wrap`] will be `true` in vertical layouts
    /// and horizontal layouts with wrapping,
    /// and `false` on non-wrapping horizontal layouts.
    ///
    /// Note that any `\n` in the text will always produce a new line.
    ///
    /// You can also use [`crate::Style::wrap`].
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self.truncate = false;
        self
    }

    /// If `true`, the text will stop at the max width of the [`Ui`],
    /// and what doesn't fit will be elided, replaced with `…`.
    ///
    /// If the text is truncated, the full text will be shown on hover as a tool-tip.
    ///
    /// Default is `false`, which means the text will expand the parent [`Ui`],
    /// or wrap if [`Self::wrap`] is set.
    ///
    /// Calling `truncate` will override [`Self::wrap`].
    #[inline]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.wrap = None;
        self.truncate = truncate;
        self
    }

    /// Can the user select the text with the mouse?
    ///
    /// Overrides [`crate::style::Interaction::selectable_labels`].
    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    ///
    /// ```
    /// # use egui::{TerminalLine, Sense};
    /// # egui::__run_test_ui(|ui| {
    /// if ui.add(TerminalLine::new("click me").sense(Sense::click())).clicked() {
    ///     /* … */
    /// }
    /// # });
    /// ```
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = Some(sense);
        self
    }
}

impl TerminalLine {
    /// Do layout and position the galley in the ui, without painting it or adding widget info.
    pub fn layout_in_ui(self, ui: &mut Ui) -> (Pos2, Arc<Galley>, Response) {
        let selectable = self
            .selectable
            .unwrap_or_else(|| ui.style().interaction.selectable_labels);

        let mut sense = self.sense.unwrap_or_else(|| {
            if ui.memory(|mem| mem.options.screen_reader) {
                // We only want to focus labels if the screen reader is on.
                Sense::focusable_noninteractive()
            } else {
                Sense::hover()
            }
        });

        
      

        if let WidgetText::Galley(galley) = self.text {
            // If the user said "use this specific galley", then just use it:
            let (rect, response) = ui.allocate_exact_size(galley.size(), sense);
            let pos = match galley.job.halign {
                Align::LEFT => rect.left_top(),
                Align::Center => rect.center_top(),
                Align::RIGHT => rect.right_top(),
            };
            return (pos, galley, response);
        }

        let valign = ui.layout().vertical_align();
        let mut layout_job = self
            .text
            .into_layout_job(ui.style(), FontSelection::Default, valign);

        let truncate = self.truncate;
        let wrap = !truncate && self.wrap.unwrap_or_else(|| ui.wrap_text());
        let available_width = ui.available_width();

        {
            if truncate {
                layout_job.wrap.max_width = available_width;
                layout_job.wrap.max_rows = 1;
                layout_job.wrap.break_anywhere = true;
            } else if wrap {
                layout_job.wrap.max_width = available_width;
            } else {
                layout_job.wrap.max_width = f32::INFINITY;
            };

            
                layout_job.halign = ui.layout().horizontal_placement();
                layout_job.justify = ui.layout().horizontal_justify();
          

            let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));
            let (rect, response) = ui.allocate_exact_size(galley.size(), sense);
            let galley_pos = 
                rect.left_top();
               
         
            (galley_pos, galley, response)
        }
    }
}

impl Widget for TerminalLine {
    fn ui(self, ui: &mut Ui) -> Response {
        // Interactive = the uses asked to sense interaction.
        // We DON'T want to have the color respond just because the text is selectable;
        // the cursor is enough to communicate that.
        let interactive = self.sense.map_or(false, |sense| sense != Sense::hover());

        let selectable = self.selectable;

        let (galley_pos, galley, mut response) = self.layout_in_ui(ui);
       // response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, galley.text()));

        if ui.is_rect_visible(response.rect) {
           

          

            let response_color = if interactive {
                ui.style().interact(&response).text_color()
            } else {
                ui.style().visuals.text_color()
            };

            ui.painter().add(
                epaint::TextShape::new(galley_pos, galley.clone(), response_color)
                  
            );

         
          
        }

        response
    }
}