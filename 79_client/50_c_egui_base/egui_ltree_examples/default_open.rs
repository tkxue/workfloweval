#[path = "data.rs"]
mod data;

use crate::egui_ltree_lib::{NodeBuilder, TreeView};
use egui::ThemePreference;

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TreeView::new(ui.make_persistent_id("Names tree view")).show(ui, |builder| {
                builder.node(NodeBuilder::dir(0).default_open(false).label("root"));

                builder.node(NodeBuilder::dir(1).default_open(false).label("Foo"));
                builder.leaf(2, "Ava");
                builder.node(NodeBuilder::dir(3).default_open(false).label("Bar"));
                builder.leaf(4, "Benjamin");
                builder.leaf(5, "Charlotte");
                builder.close_dir();
                builder.close_dir();
                builder.leaf(6, "Daniel");
                builder.leaf(7, "Emma");
                builder.node(NodeBuilder::dir(8).default_open(false).label("Baz"));
                builder.leaf(9, "Finn");
                builder.leaf(10, "Grayson");
                builder.close_dir();
                builder.close_dir();
            });
        });
    }
}
