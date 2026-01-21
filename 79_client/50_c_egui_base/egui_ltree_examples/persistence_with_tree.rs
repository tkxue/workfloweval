//! This example has the persistence feature enable on both eframe and therfore also egui
//! The feature _is_ enabled on egui_ltreeview which means that the tree view state
//! is persisted.
//! This also requires the node to be serializable.

#[path = "data.rs"]
mod data;

use crate::egui_ltree_lib::TreeView;
use egui::ThemePreference;
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct MyApp;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Node(i32);

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TreeView::new(ui.make_persistent_id("Names tree view")).show(ui, |builder| {
                builder.dir(Node(0), "Root");
                builder.dir(Node(1), "Foo");
                builder.leaf(Node(2), "Ava");
                builder.dir(Node(3), "Bar");
                builder.leaf(Node(4), "Benjamin");
                builder.leaf(Node(5), "Charlotte");
                builder.close_dir();
                builder.close_dir();
                builder.leaf(Node(6), "Daniel");
                builder.leaf(Node(7), "Emma");
                builder.dir(Node(8), "Baz");
                builder.leaf(Node(9), "Finn");
                builder.leaf(Node(10), "Grayson");
                builder.close_dir();
                builder.close_dir();
            });
        });
    }
}
