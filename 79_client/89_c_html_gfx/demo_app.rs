use super::*;

use c_egui_base::egui_demo_lib::View as _;
use c_egui_base::egui_demo_lib::is_mobile;
use c_egui_base::egui_demo_lib::{Demo, ssheet};
use eframe::CreationContext;
use egui::containers::menu;
use egui::style::StyleModifier;
use egui::{Context, Modifiers, ScrollArea, Ui};
use std::collections::BTreeSet;

// ----------------------------------------------------------------------------

struct DemoGroup {
    demos: Vec<Box<dyn Demo>>,
}

impl DemoGroup {
    pub fn new(demos: Vec<Box<dyn Demo>>) -> Self {
        Self { demos }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>) {
        /*
        let Self { demos } = self;
        for demo in demos {
            if demo.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(demo.name());
                ui.toggle_value(&mut is_open, demo.name());
                set_open(open, demo.name(), is_open);
            }
        }
        */
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        /*
        let Self { demos } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        } */
    }
}

// ----------------------------------------------------------------------------

pub struct DemoGroups {
    apps: DemoGroup,
}

impl DemoGroups {
    pub fn new(cc: &CreationContext, apps: Vec<Box<dyn Demo>>) -> Self {
        Self {
            // about: About::default(),
            apps: DemoGroup::new(apps),
        }
    }
}

impl DemoGroups {
    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>) {
        let Self {
            // about,
            apps,
            // demos,
            // tests,
        } = self;

        /*
        {
            let mut is_open = open.contains(about.name());
            ui.toggle_value(&mut is_open, about.name());
            set_open(open, about.name(), is_open);
        }
        */
        ui.separator();
        apps.checkboxes(ui, open);
        // ui.separator();
        // demos.checkboxes(ui, open);
        // ui.separator();
        // tests.checkboxes(ui, open);
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        let Self {
            // about,
            apps,
            // demos,
            // tests,
        } = self;
        /*
        {
            let mut is_open = open.contains(about.name());
            about.show(ctx, &mut is_open);

            set_open(open, about.name(), is_open);
        } */
        apps.windows(ctx, open);
        // demos.windows(ctx, open);
        // tests.windows(ctx, open);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
pub struct DemoWindows {}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        /*
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
        }
        */
        self.desktop_ui(ctx);
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::MenuBar::new()
                .config(menu::MenuConfig::new().style(StyleModifier::default()))
                .ui(ui, |ui| {
                    let font_size = 16.5;

                    ui.menu_button(
                        egui::RichText::new("⏷ demos").size(font_size),
                        |ui| {
                            self.demo_list_ui(ui);
                        },
                    );

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            use egui::special_emojis::GITHUB;
                            ui.hyperlink_to(
                            egui::RichText::new("🦋").size(font_size),
                            "https://bsky.app/profile/ernerfeldt.bsky.social",
                        );
                            ui.hyperlink_to(
                                egui::RichText::new(GITHUB).size(font_size),
                                "https://github.com/emilk/egui",
                            );
                        },
                    );
                });
        });
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ Workflow-Eval");
                });

                ui.separator();

                self.demo_list_ui(ui);
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
                file_menu_button(ui);
            });
        });

        // tkx
        // self.groups.windows(ctx, &mut self.open);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        // tkx
        /*
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::LEFT),
                |ui| {
                    self.groups.checkboxes(ui, &mut self.open);
                    ui.separator();
                    /*
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }*/
                },
            );
        });
        */
    }
}

// ----------------------------------------------------------------------------

fn file_menu_button(ui: &mut Ui) {
    let organize_shortcut = egui::KeyboardShortcut::new(
        Modifiers::CTRL | Modifiers::SHIFT,
        egui::Key::O,
    );
    let reset_shortcut = egui::KeyboardShortcut::new(
        Modifiers::CTRL | Modifiers::SHIFT,
        egui::Key::R,
    );

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("File", |ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui);
            ui.weak(format!("Current zoom: {:.0}%", 100.0 * ui.ctx().zoom_factor()))
                .on_hover_text("The UI zoom level, on top of the operating system's default value");
            ui.separator();
        }

        if ui
            .add(egui::Button::new("Organize Windows").shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)))
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        if ui
            .add(egui::Button::new("Reset egui memory").shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)))
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }
    });
}
