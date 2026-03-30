use egui::{InnerResponse, Response, Style, Ui, Visuals};
use extension_traits::extension;

use crate::Orientation;

/// Extentions to [Ui]
#[extension(pub trait UiExt)]
impl Ui {
    /// Add contents with scoped style/visuals
    fn scoped_style<S, V, F, R>(
        &mut self,
        modify_style: S,
        modify_visuals: V,
        add_contents: F,
    ) -> InnerResponse<R>
    where
        S: FnOnce(&mut Style),
        V: FnOnce(&mut Visuals),
        F: FnOnce(&mut Ui) -> R,
    {
        self.scope(|ui| {
            modify_style(ui.style_mut());
            modify_visuals(ui.visuals_mut());
            add_contents(ui)
        })
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
