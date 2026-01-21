//! A lightweight RAD GUI builder for `egui` written in Rust.

pub mod app;
mod project;
mod widget;

use eframe::egui;

/*
fn initial_inner_size() -> egui::Vec2 {
    // Mirror your defaults
    let project = project::Project::default();

    // Base: canvas
    let mut w = project.canvas_size.x;
    let mut h = project.canvas_size.y;

    // Right inspector (default width = 260)
    w += 260.0;

    // Left palette is open by default in RadBuilderApp::default()
    w += 220.0;

    // Small padding for menubar + side padding
    h += 40.0;
    w += 16.0;

    egui::vec2(w, h)
}
*/
