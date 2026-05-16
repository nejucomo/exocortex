//! The [WidgetWith] trait and [WidgetWithParam] type to simplify threading state to widgets
use egui::{Response, Ui, Widget};

/// Something which makes a widget when combined with a parameter `P`
pub trait WidgetWith<P>: Sized {
    /// Combine `self` with `param`
    fn with(self, param: P) -> WidgetWithParam<Self, P> {
        WidgetWithParam {
            widget: self,
            param,
        }
    }

    /// Render `self` to `ui` with `param`
    fn ui_with(self, ui: &mut Ui, param: P) -> Response;
}

impl<F, P> WidgetWith<P> for F
where
    F: FnOnce(&mut Ui, P) -> Response,
{
    fn ui_with(self, ui: &mut Ui, param: P) -> Response {
        self(ui, param)
    }
}

/// A [WidgetWith] along with its param `P`
#[derive(Copy, Clone, Debug)]
pub struct WidgetWithParam<W, P> {
    widget: W,
    param: P,
}

impl<W, P> Widget for WidgetWithParam<W, P>
where
    W: WidgetWith<P>,
{
    fn ui(self, ui: &mut Ui) -> Response {
        self.widget.ui_with(ui, self.param)
    }
}
