use eframe::egui::{Color32, Stroke, StrokeKind, Ui};

#[allow(dead_code)]
pub(crate) fn debug_ui_rects(ui: &Ui) {
    let painter = ui.painter();

    // Space already used by widgets:
    painter.rect_filled(
        ui.min_rect(),
        0.0,
        Color32::from_rgba_unmultiplied(0, 255, 0, 40),
    );

    // Full target/layout area for this Ui:
    painter.rect_stroke(
        ui.max_rect(),
        0.0,
        Stroke::new(1.0, Color32::RED),
        StrokeKind::Inside,
    );

    // Remaining free area before wrap:
    painter.rect_filled(
        ui.available_rect_before_wrap(),
        0.0,
        Color32::from_rgba_unmultiplied(255, 255, 0, 25),
    );
}
