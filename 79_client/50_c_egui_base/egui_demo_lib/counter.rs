use super::*;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct AppC_Counter {}

impl crate::egui_demo_lib::Demo for AppC_Counter {
    fn name(&self) -> &'static str {
        "Counter"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .resizable([true, false])
            .scroll(false)
            .show(ctx, |ui| {
                use crate::egui_demo_lib::View as _;
                self.ui(ui);
            });
    }
}

impl crate::egui_demo_lib::View for AppC_Counter {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("Counter");
        ui.label(format!("workflow-eval"));

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(
                "Immediate mode is a GUI paradigm that lets you create a GUI with less code and simpler control flow. For example, this is how you create a ",
            );
            let _ = ui.button("Inc");
            let _ = ui.button("Get");
            let _ = ui.button("Dec");
        });
    }
}
