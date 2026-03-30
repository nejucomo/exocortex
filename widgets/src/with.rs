//! The [WidgetWith] trait and [WidgetWithParam] type to simplify threading state to widgets
use egui::{Response, Ui, Widget};

/// Something which makes a widget when combined with a parameter `P`
pub trait WidgetWith<P>: Sized {
    /// Combine `self` with `param`
    ///
    /// # Repeated `with` calls
    /// If `Q` is `P`, this is a [Widget] impl. However, we do not constrain `param` to be `P` so that we can nest [WidgetWith::with] calls:
    ///
    /// ```
    /// use egui::Widget;
    /// use exocortex_widgets::with::WidgetWith;
    ///
    /// fn make_widget<W, P, Q>(widget: W, p: P, q: Q) -> impl Widget
    ///    where W: WidgetWith<(P, Q)>,
    /// {
    ///    widget.with(p).with(q)
    /// }
    /// ```
    fn with<Q>(self, param: Q) -> WidgetWithParam<Self, Q> {
        WidgetWithParam {
            widget: self,
            param,
        }
    }

    /// Render `self` to `ui` with `param`
    fn ui_with(self, ui: &mut Ui, param: P) -> Response;
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

/// Enable nested [WidgetWithParam]s; see [WidgetWith::with]
impl<W, P, Q> WidgetWith<Q> for WidgetWithParam<W, P>
where
    W: WidgetWith<(P, Q)>,
{
    fn ui_with(self, ui: &mut Ui, param: Q) -> Response {
        self.widget.ui_with(ui, (self.param, param))
    }
}
