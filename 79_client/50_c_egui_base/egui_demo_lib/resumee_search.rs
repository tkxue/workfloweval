use super::*;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Resumee_Search {}

impl crate::egui_demo_lib::Demo for Resumee_Search {
    fn name(&self) -> &'static str {
        "Resumee Search"
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

impl crate::egui_demo_lib::View for Resumee_Search {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("Resumee Search");
        ui.label(format!("workflow-eval"));
    }
}

fn about_immediate_mode(ui: &mut egui::Ui) {
    ui.style_mut().spacing.interact_size.y = 0.0; // hack to make `horizontal_wrapped` work better with text.

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Immediate mode is a GUI paradigm that lets you create a GUI with less code and simpler control flow. For example, this is how you create a ");
        let _ = ui.small_button("button");
        ui.label(" in egui:");
    });

    ui.add_space(8.0);

    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("There are no callbacks or messages, and no button state to store. ");
        ui.label("Read more about immediate mode ");
        ui.hyperlink_to("here", "https://github.com/emilk/egui#why-immediate-mode");
        ui.label(".");
    });
}
