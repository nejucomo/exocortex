use egui::{Response, Sense, Ui, Vec2, Widget};

use crate::with::WidgetWith;

/// A [Widget] which simply adds all of its inner widgets, unioning the response
pub fn many<I, W>(widgets: I) -> Many<I, W>
where
    I: IntoIterator<Item = W>,
{
    Many { widgets }
}

#[derive(Debug)]
pub struct Many<I, W>
where
    I: IntoIterator<Item = W>,
{
    widgets: I,
}

impl<I, W> Widget for Many<I, W>
where
    I: IntoIterator<Item = W>,
    W: Widget,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());

        for w in self.widgets {
            r |= ui.add(w);
        }

        r
    }
}

impl<I, W, P> WidgetWith<P> for Many<I, W>
where
    I: IntoIterator<Item = W>,
    W: WidgetWith<P>,
    P: Clone,
{
    fn ui_with(self, ui: &mut Ui, param: P) -> Response {
        let mut r = ui.allocate_response(Vec2::ZERO, Sense::hover());

        for w in self.widgets {
            r |= ui.add(w.with(param.clone()));
        }

        r
    }
}
