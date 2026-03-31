use derive_builder::Builder;
use egui::{Align, Frame, Layout, Response, Sense, TextStyle, Ui, Widget};

use crate::uiext::UiExt as _;

/// A frame with a "physical card" appearance
///
/// A card has two sections for "metadata" and "content". Cards provide hover and click interaction as a whole.
pub fn card<S, M, C>() -> CardBuilder<S, M, C>
where
    S: Widget,
    M: Widget,
    C: Widget,
{
    CardBuilder::default()
}

/// A [Card] widget
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Card<S, M, C>
where
    S: Widget,
    M: Widget,
    C: Widget,
{
    /// The display mode of the [Card]
    mode: CardMode,
    /// The summary widget, typically a single line
    summary: S,
    /// The metadata widget, typically a single line
    metadata: M,
    /// The full content widget, typically a superset of the summary
    content: C,
}

/// The display mode for a [Card]
#[derive(Copy, Clone, Debug)]
pub enum CardMode {
    /// Display only the summary in a streamlined fashion
    Summary,
    /// Display the summary with metadata
    Metadata,
    /// Display the full content
    Content,
}

impl<S, M, C> Widget for Card<S, M, C>
where
    S: Widget,
    M: Widget,
    C: Widget,
{
    fn ui(self, ui: &mut Ui) -> Response {
        use CardMode::*;

        let Card {
            mode,
            summary,
            metadata,
            content,
        } = self;

        let mut prep = Frame::group(ui.style()).begin(ui);

        prep.content_ui
            .with_layout(Layout::top_down(Align::Max), |ui| match mode {
                Summary => {
                    ui.scoped_style(
                        |_style| {},
                        |visuals| {
                            visuals.override_text_color =
                                Some(visuals.widgets.inactive.fg_stroke.color);
                        },
                        summary,
                    );
                }
                Metadata => {
                    ui.scoped_style(
                        |style| {
                            style.override_text_style = Some(TextStyle::Small);
                        },
                        |visuals| {
                            visuals.override_text_color =
                                Some(visuals.widgets.noninteractive.fg_stroke.color);
                        },
                        metadata,
                    );
                    ui.scoped_style(
                        |_style| {},
                        |visuals| {
                            visuals.override_text_color =
                                Some(visuals.widgets.inactive.fg_stroke.color);
                        },
                        summary,
                    );
                }
                Content => {
                    ui.scoped_style(
                        |style| {
                            style.override_text_style = Some(TextStyle::Small);
                        },
                        |visuals| {
                            visuals.override_text_color =
                                Some(visuals.widgets.noninteractive.fg_stroke.color);
                        },
                        metadata,
                    );
                    ui.scoped_style(
                        |_style| {},
                        |visuals| {
                            visuals.override_text_color =
                                Some(visuals.widgets.inactive.fg_stroke.color);
                        },
                        content,
                    );
                }
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
