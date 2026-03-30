use egui::{InnerResponse, Style, Ui, Visuals};
use extension_traits::extension;

#[extension(pub trait UiExt)]
impl Ui {
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
}
