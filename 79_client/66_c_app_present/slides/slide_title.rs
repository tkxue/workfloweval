use c_app_repl::{CodeEditor, demo::CodeEditorDemo};

use super::*;

pub struct Capp_Present_Slide__Title {
    // python_repl: C_Python_Repl,
}

impl Capp_Present_Slide_T for Capp_Present_Slide__Title {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // 2. Add flexible space above the text
            ui.add_space(50.); // ui.available_height() / 2.0 - 36.0);

            // 3. Create the label with custom size

            ui.label(egui::RichText::new("Title Slide").size(72.0).strong());
            ui.label(egui::RichText::new("This is not PowerPoint.").size(24.0).strong());
            ui.label(egui::RichText::new("This is not Google Slides.").size(24.0).strong());
            ui.label(egui::RichText::new("This is powered by OodaDB.").size(24.0).strong());

            ui.label(
                egui::RichText::new("There is a fucking python interpreter embedded in this slide.")
                    .size(24.0)
                    .strong(),
            );
        });
        ui.add_space(50.);
        // self.python_repl.ui(ui);
    }
}

impl Capp_Present_Slide__Title {
    pub fn new() -> Self {
        Self {
            // python_repl: C_Python_Repl::default(),
        }
    }
}
