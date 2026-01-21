#[derive(Default)]
pub struct CursorTest {}

impl crate::egui_demo_lib::Demo for CursorTest {
    fn name(&self) -> &'static str {
        "Cursor Test"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use crate::egui_demo_lib::View as _;
            self.ui(ui);
        });
    }
}

impl crate::egui_demo_lib::View for CursorTest {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Hover to switch cursor icon:");
            for &cursor_icon in &egui::CursorIcon::ALL {
                let _ = ui
                    .button(format!("{cursor_icon:?}"))
                    .on_hover_cursor(cursor_icon);
            }
            ui.add(crate::egui_github_link_file!());
        });
    }
}
