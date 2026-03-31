use egui::{Response, Style, Ui, Visuals, Widget};
use extension_traits::extension;

use crate::Orientation;

/// Extentions to [Ui]
#[extension(pub trait UiExt)]
impl Ui {
    /// Add contents with scoped style/visuals
    fn scoped_style<S, V, W>(&mut self, modify_style: S, modify_visuals: V, contents: W) -> Response
    where
        S: FnOnce(&mut Style),
        V: FnOnce(&mut Visuals),
        W: Widget,
    {
        self.scope(|ui| {
            modify_style(ui.style_mut());
            modify_visuals(ui.visuals_mut());
            ui.add(contents)
        })
        .response
    }

    /// Add a scroll area
    fn scroll_area<F>(&mut self, orientation: Orientation, content: F) -> Response
    where
        F: FnOnce(&mut Ui) -> Response,
    {
        use Orientation::*;
        use egui::ScrollArea;

        let sa = match orientation {
            Horizontal => ScrollArea::horizontal(),
            Vertical => ScrollArea::vertical(),
        };

        sa.show(self, content).inner
    }
}
