fn rounded_rect<R>(
    ui: &mut egui::Ui, 
    rounding: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(245, 245, 250))
        .rounding(rounding)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 220)))
        .inner_margin(egui::style::Margin::symmetric(16.0, 12.0))
        .show(ui, add_contents)
        .inner
}