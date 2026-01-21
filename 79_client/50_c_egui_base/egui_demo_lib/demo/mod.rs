//! Demo-code for showing how egui is used.
//!
//! The demo-code is also used in benchmarks and tests.

// ----------------------------------------------------------------------------

pub mod about;
pub mod code_editor;
pub mod code_example;
pub mod dancing_strings;
pub mod demo_app_windows;
pub mod drag_and_drop;
pub mod extra_viewport;
pub mod font_book;
pub mod frame_demo;
pub mod highlighting;
pub mod interactive_container;
pub mod misc_demo_window;
pub mod modals;
pub mod multi_touch;
pub mod paint_bezier;
pub mod painting;
pub mod panels;
pub mod password;
mod popups;
pub mod scene;
pub mod screenshot;
pub mod scrolling;
pub mod sliders;
pub mod strip_demo;
pub mod table_demo;
pub mod tests;
pub mod text_edit;
pub mod text_layout;
pub mod toggle_switch;
pub mod tooltips;
pub mod undo_redo;
pub mod widget_gallery;
pub mod window_options;

pub use {
    about::About, misc_demo_window::MiscDemoWindow,
    widget_gallery::WidgetGallery,
};

// ----------------------------------------------------------------------------

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Demo {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
