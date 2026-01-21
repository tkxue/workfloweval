use super::*;

#[derive(Clone)]
pub struct Cgfx_App_Main_Ptr {
    pub inner: ArcState<Cgfx_App_Main>,
}

impl Cgfx_App_Main_Ptr {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            inner: ArcState::new(Cgfx_App_Main {
                repl_rune: Cgfx_Repl_Rune::new(),
                repl_python: Cgfx_Repl_Python::new(),
                repl_sqlite: Cgfx_Repl_Sqlite::new(),
                sheet: Cgfx_Sheet::new(),
                street: Cgfx_Street::new(egui_ctx),
            }),
        }
    }
}

impl eframe::App for Cgfx_App_Main_Ptr {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)
                ..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );
        let color = egui::Color32::from(color);
        color.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("workfloweval panel")
            .resizable(false)
            .default_width(270.0)
            .min_width(270.)
            .show(ctx, |ui| {
                let total_height = ui.ctx().viewport_rect().height();
                let desired_height = total_height - 270.0;
                ui.set_max_height(desired_height);

                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ Workflow-Eval");
                });
                ui.separator();
            });

        self.inner.update(|obj| {
            egui::CentralPanel::default().show(ctx, |ui| {
                obj.repl_python.update(ui);
                obj.repl_sqlite.update(ui);
                obj.sheet.update(ui);
                obj.street.update(ui);
            });
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(&mut *self)
    }
}
