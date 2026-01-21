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
                builder.dir(0, "root");

                // Sometimes you want to a section of the tree to behave like a dir
                // without incrasing the depth of the tree. In that case you can flatten
                // the dir. This will not render the dir but still register it in the tree.
                builder.node(NodeBuilder::dir(1).flatten(true).label("Foo"));
                builder.leaf(2, "Ava");
                builder.node(NodeBuilder::dir(3).flatten(true).label("Bar"));
                builder.leaf(4, "Benjamin");
                builder.leaf(5, "Charlotte");
                builder.close_dir();
                builder.close_dir();
                builder.leaf(6, "Daniel");
                builder.leaf(7, "Emma");
                builder.node(NodeBuilder::dir(8).flatten(true).label("Baz"));
                builder.leaf(9, "Finn");
                builder.leaf(10, "Grayson");
                builder.close_dir();
                builder.close_dir();
            });
        });
    }
}
