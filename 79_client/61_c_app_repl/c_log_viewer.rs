use super::*;

use c_egui_base::{C_LogEntry, Cmsg_Inner, Cmsg_WwRune, G_CmsgQ};
use e_api::wlog;
use egui::{PointerButton, ScrollArea};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Capp_LogViewer {
    lines: Vec<C_LogEntry>,
}

impl Default for Capp_LogViewer {
    fn default() -> Self {
        Self { lines: vec![] }
    }
}

impl c_egui_base::egui_demo_lib::Demo for Capp_LogViewer {
    fn name(&self) -> &'static str {
        "LogViewer"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use c_egui_base::egui_demo_lib::View as _;
        egui::Window::new(self.name())
            .open(open)
            .default_height(800.0)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl c_egui_base::egui_demo_lib::View for Capp_LogViewer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let row_height = ui.spacing().interact_size.y;
        let total_rows = self.lines.len();
        let items = &self.lines;

        ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
            // 3. egui provides 'row_range', which tells you exactly which
            // indices are currently visible.
            for i in row_range {
                ui.label(format!("{:?}: {}", items[i].id_proc, items[i].msg));
            }
        });
    }
}
