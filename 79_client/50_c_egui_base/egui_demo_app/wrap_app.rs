use crate::egui_demo_lib::{Demo, is_mobile};

use c_app_msg::G_CmsgQ;
use core::any::Any;
use e_api::wlog;
use eframe::CreationContext;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct EasyMarkApp {
    editor: crate::egui_demo_lib::easy_mark::EasyMarkEditor,
}

impl eframe::App for EasyMarkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.editor.panels(ctx);
    }
}

// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct FractalClockApp {
    fractal_clock: crate::egui_demo_app::apps::FractalClock,
    pub mock_time: Option<f64>,
}

impl eframe::App for FractalClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::dark_canvas(&ctx.style())
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(0),
            )
            .show(ctx, |ui| {
                self.fractal_clock.ui(
                    ui,
                    self.mock_time.or(Some(crate::egui_demo_app::seconds_since_midnight())),
                );
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ColorTestApp {
    color_test: crate::egui_demo_lib::ColorTest,
}

impl eframe::App for ColorTestApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if frame.is_web() {
                ui.label("NOTE: Some old browsers stuck on WebGL1 without sRGB support will not pass the color test.");
                ui.separator();
            }
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                self.color_test.ui(ui);
            });
        });
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Anchor {
    #[default]
    Demo,

    EasyMarkEditor,

    #[cfg(feature = "http")]
    Http,

    #[cfg(feature = "image_viewer")]
    ImageViewer,

    Clock,

    Custom3d,

    /// Rendering test
    Rendering,
}

impl Anchor {
    fn all() -> Vec<Self> {
        vec![
            Self::Demo,
            Self::EasyMarkEditor,
            #[cfg(feature = "http")]
            Self::Http,
            Self::Clock,
            #[cfg(any(feature = "glow", feature = "wgpu"))]
            Self::Custom3d,
            Self::Rendering,
        ]
    }

    pub fn from_str_case_insensitive(anchor: &str) -> Option<Self> {
        let anchor = anchor.to_lowercase();
        Self::all().into_iter().find(|x| x.to_string() == anchor)
    }
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = format!("{self:?}");
        name.make_ascii_lowercase();
        f.write_str(&name)
    }
}

impl From<Anchor> for egui::WidgetText {
    fn from(value: Anchor) -> Self {
        Self::from(value.to_string())
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
#[must_use]
pub enum Command {
    Nothing,
    ResetEverything,
}

// ----------------------------------------------------------------------------
