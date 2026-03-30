use egui::{Align, Frame, Layout, Response, Sense, TextStyle, Ui, Widget};

use crate::uiext::UiExt as _;

/// A frame with a "physical card" appearance
///
/// A card has two sections for "metadata" and "content". Cards provide hover and click interaction as a whole.
pub fn card<M, C>(render_metadata: M, render_content: C) -> impl Widget
where
    M: FnOnce(&mut Ui),
    C: FnOnce(&mut Ui),
{
    Card {
        render_metadata,
        render_content,
    }
}

pub struct Card<M, C>
where
    M: FnOnce(&mut Ui),
    C: FnOnce(&mut Ui),
{
    render_metadata: M,
    render_content: C,
}

impl<M, C> Widget for Card<M, C>
where
    M: FnOnce(&mut Ui),
    C: FnOnce(&mut Ui),
{
    fn ui(self, ui: &mut Ui) -> Response {
        let Card {
            render_metadata,
            render_content,
        } = self;

        let mut prep = Frame::group(ui.style()).begin(ui);

        prep.content_ui
            .with_layout(Layout::top_down(Align::Max), |ui| {
                // Add metadata to the top:
                ui.scoped_style(
                    |style| {
                        style.override_text_style = Some(TextStyle::Small);
                    },
                    |visuals| {
                        visuals.override_text_color =
                            Some(visuals.widgets.noninteractive.fg_stroke.color);
                    },
                    render_metadata,
                );

                // Now add content:
                ui.scoped_style(
                    |_style| {},
                    |visuals| {
                        visuals.override_text_color =
                            Some(visuals.widgets.inactive.fg_stroke.color);
                    },
                    render_content,
                );
            });

        let resp = prep.allocate_space(ui).interact(Sense::click());

        {
            let widget_visuals = ui.visuals().widgets.style(&resp);
            prep.frame.fill = widget_visuals.bg_fill;
            prep.frame.stroke = widget_visuals.bg_stroke;
            prep.paint(ui);
        }

        resp
    }
}
