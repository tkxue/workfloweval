use super::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Capp_Present {}

pub trait Capp_Present_Slide_T: Send + Sync {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl Default for Capp_Present {
    fn default() -> Self {
        Self {}
    }
}

impl c_egui_base::egui_demo_lib::Demo for Capp_Present {
    fn name(&self) -> &'static str {
        "Present"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use c_egui_base::egui_demo_lib::View as _;
        egui::Window::new(self.name())
            .open(open)
            .fixed_size([1200.0, 800.0])
            .resizable(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl c_egui_base::egui_demo_lib::View for Capp_Present {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_min_size(egui::vec2(1200.0, 800.0));

        let x = G_Capp_Present::read(|x| {
            let slide = x.slides.get(&x.cur_slide).unwrap();
            slide.update(|s| s.ui(ui));
        });
    }
}
