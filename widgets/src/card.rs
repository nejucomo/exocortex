use derive_builder::Builder;
use egui::{Align, Frame, Layout, Response, Sense, Ui, Widget};

use crate::with::WidgetWith;

/// A frame with a "physical card" appearance
///
/// A card has two sections for "metadata" and "content". Cards provide hover and click interaction as a whole.
pub fn card<'a, M, C>() -> CardBuilder<'a, M, C>
where
    M: WidgetWith<CardMode>,
    C: WidgetWith<CardMode>,
{
    CardBuilder::default()
}

/// A [Card] widget
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Card<'a, M, C>
where
    M: WidgetWith<CardMode>,
    C: WidgetWith<CardMode>,
{
    /// The display mode of the [Card]
    mode: &'a mut CardMode,
    /// The metadata widget
    metadata: M,
    /// The full content widget
    content: C,
}

/// The display mode for a [Card]
#[derive(Copy, Clone, Debug, Default)]
pub enum CardMode {
    /// Display only the summary in a streamlined fashion
    #[default]
    Streamlined,
    /// Display the full content
    Expanded,
}

impl<'a, M, C> Widget for Card<'a, M, C>
where
    M: WidgetWith<CardMode>,
    C: WidgetWith<CardMode>,
{
    fn ui(self, ui: &mut Ui) -> Response {
        use CardMode::*;

        let Card {
            mode,
            metadata,
            content,
        } = self;

        let mut prep = Frame::group(ui.style()).begin(ui);

        prep.content_ui
            .with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.add(metadata.with(*mode));
                ui.add(content.with(*mode))
            });

        let resp = prep.allocate_space(ui).interact(Sense::click());

        {
            let widgets = &ui.visuals().widgets;

            let (fill, stroke) = if matches!(mode, Streamlined) && !resp.hovered() {
                let wv = widgets.noninteractive;
                let mut stroke = wv.bg_stroke;
                stroke.width = 0.1;
                (wv.bg_fill, stroke)
            } else {
                let wv = widgets.style(&resp);
                (wv.bg_fill, wv.bg_stroke)
            };

            prep.frame.fill = fill;
            prep.frame.stroke = stroke;
        }
        prep.paint(ui);

        *mode = if resp.clicked() || matches!(mode, Expanded) && resp.hovered() {
            Expanded
        } else {
            Streamlined
        };

        resp
    }
}
