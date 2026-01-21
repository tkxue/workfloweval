#[path = "data.rs"]
mod data;

use crate::egui_ltree_lib::TreeView;
use egui::ThemePreference;

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TreeView::new(ui.make_persistent_id("Names tree view")).show(ui, |builder| {
                builder.dir(0, "Root");
                builder.dir(1, "Foo");
                builder.leaf(2, "Ava");
                builder.dir(3, "Bar");
                builder.leaf(4, "Benjamin");
                builder.leaf(5, "Charlotte");
                builder.close_dir();
                builder.close_dir();
                builder.leaf(6, "Daniel");
                builder.leaf(7, "Emma");
                builder.dir(8, "Baz");
                builder.leaf(9, "Finn");
                builder.leaf(10, "Grayson");
                builder.close_dir();
                builder.close_dir();
            });
        });
    }
}
