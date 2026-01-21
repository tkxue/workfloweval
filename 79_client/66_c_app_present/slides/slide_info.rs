use super::*;

pub struct Capp_Present_Slide__Info {}

impl Capp_Present_Slide_T for Capp_Present_Slide__Info {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // 2. Add flexible space above the text
            ui.add_space(ui.available_height() / 2.0 - 144.0);

            ui.label(
                egui::RichText::new("Please click\n'CLICK ME'\nbutton on Left.")
                    .size(72.0)
                    .strong(),
            );
            ui.label(
                egui::RichText::new("lhs, rhs are two different iframes")
                    .size(24.0)
                    .strong(),
            );
            ui.label(egui::RichText::new("iframes can't auto play video").size(24.0).strong());

            ui.label(egui::RichText::new("until mouse event in iframe").size(24.0).strong());
        });
    }
}

impl Capp_Present_Slide__Info {
    pub fn new() -> Self {
        Self {}
    }
}
