use crate::egui_ltree_lib::TreeView;
use egui::{ScrollArea, ThemePreference};

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("tree panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                TreeView::new(ui.make_persistent_id("Names tree view")).show(ui, |builder| {
                    for val in 1..100 {
                        let width = 1 + val / 5;
                        let name = width.to_string().repeat(width);
                        builder.leaf(val, name);
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
