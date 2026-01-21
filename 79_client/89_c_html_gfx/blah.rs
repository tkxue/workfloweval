use super::*;

pub struct Blah {}

/*
fn mobile_ui(&mut self, ctx: &Context) {
    if self.about_is_open() {
        let mut close = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                self.groups.about.ui(ui);
                ui.add_space(12.0);
                ui.vertical_centered_justified(|ui| {
                    if ui
                        .button(egui::RichText::new("Continue to the demo!").size(20.0))
                        .clicked()
                    {
                        close = true;
                    }
                });
            });
        });
        if close {
            set_open(&mut self.open, About::default().name(), false);
        }
    } else {
        self.mobile_top_bar(ctx);
        self.groups.windows(ctx, &mut self.open);
    }
}
*/

/*
demos: DemoGroup::new(vec![
    Box::<super::paint_bezier::PaintBezier>::default(),
    Box::<super::code_editor::LispEditor>::default(),
    Box::<super::code_example::CodeExample>::default(),
    Box::<super::dancing_strings::DancingStrings>::default(),
    Box::<super::drag_and_drop::DragAndDropDemo>::default(),
    Box::<super::extra_viewport::ExtraViewport>::default(),
    Box::<super::font_book::FontBook>::default(),
    Box::<super::frame_demo::FrameDemo>::default(),
    Box::<super::highlighting::Highlighting>::default(),
    Box::<super::interactive_container::InteractiveContainerDemo>::default(),
    Box::<super::MiscDemoWindow>::default(),
    Box::<super::modals::Modals>::default(),
    Box::<super::multi_touch::MultiTouch>::default(),
    Box::<super::painting::Painting>::default(),
    Box::<super::panels::Panels>::default(),
    Box::<super::popups::PopupsDemo>::default(),
    Box::<super::scene::SceneDemo>::default(),
    Box::<super::screenshot::Screenshot>::default(),
    Box::<super::scrolling::Scrolling>::default(),
    Box::<super::sliders::Sliders>::default(),
    Box::<super::strip_demo::StripDemo>::default(),
    Box::<super::table_demo::TableDemo>::default(),
    Box::<super::text_edit::TextEditDemo>::default(),
    Box::<super::text_layout::TextLayoutDemo>::default(),
    Box::<super::tooltips::Tooltips>::default(),
    Box::<super::undo_redo::UndoRedoDemo>::default(),
    Box::<super::widget_gallery::WidgetGallery>::default(),
    Box::<super::window_options::WindowOptions>::default(),

]),
    */
/*
tests: DemoGroup::new(vec![
    Box::<super::tests::ClipboardTest>::default(),
    Box::<super::tests::CursorTest>::default(),
    Box::<super::tests::GridTest>::default(),
    Box::<super::tests::IdTest>::default(),
    Box::<super::tests::InputEventHistory>::default(),
    Box::<super::tests::InputTest>::default(),
    Box::<super::tests::LayoutTest>::default(),
    Box::<super::tests::ManualLayoutTest>::default(),
    Box::<super::tests::SvgTest>::default(),
    Box::<super::tests::TessellationTest>::default(),
    Box::<super::tests::WindowResizeTest>::default(),
]),
    */

/*
fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, cmd: &mut Command) {
    egui::widgets::global_theme_preference_switch(ui);

    ui.separator();

    if is_mobile(ui.ctx()) {
        ui.menu_button("💻 Backend", |ui| {
            ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
            self.backend_panel_contents(ui, frame, cmd);
        });
    } else {
        ui.toggle_value(&mut self.state.backend_panel.open, "💻 Backend");
    }

    ui.separator();

    let mut selected_anchor = self.state.selected_anchor;
    for (name, anchor, _app) in self.apps_iter_mut() {
        if ui.selectable_label(selected_anchor == anchor, name).clicked() {
            selected_anchor = anchor;
            if frame.is_web() {
                ui.ctx().open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
            }
        }
    }
    self.state.selected_anchor = selected_anchor;

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if false {
            // TODO(emilk): fix the overlap on small screens
            if clock_button(ui, c_egui_base::egui_demo_app::seconds_since_midnight()).clicked() {
                self.state.selected_anchor = Anchor::Clock;
                if frame.is_web() {
                    ui.ctx().open_url(egui::OpenUrl::same_tab("#clock"));
                }
            }
        }

        egui::warn_if_debug_build(ui);
    });
}
*/

/*
fn backend_panel_contents(
    &mut self,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    cmd: &mut Command,
) {
    self.state.backend_panel.ui(ui, frame);

    ui.separator();

    ui.horizontal(|ui| {
        if ui
            .button("Reset egui")
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
            ui.close();
        }

        if ui.button("Reset everything").clicked() {
            *cmd = Command::ResetEverything;
            ui.close();
        }
    });
}
*/

/*
fn backend_panel(
    &mut self,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
) -> Command {
    // The backend-panel can be toggled on/off.
    // We show a little animation when the user switches it.
    // let is_open = self.state.backend_panel.open || ctx.memory(|mem| mem.everything_is_visible());

    let mut cmd = Command::Nothing;

    egui::SidePanel::left("backend_panel")
        .resizable(false)
        .show_animated(ctx, is_open, |ui| {
            ui.add_space(4.0);
            ui.vertical_centered(|ui| {
                ui.heading("💻 Backend");
            });

            ui.separator();
            self.backend_panel_contents(ui, frame, &mut cmd);
        });

    cmd
}
*/

/*
pub struct Cgfx_App_Main {
    // pub apps: Vec<Box<dyn Demo>>,
    pub open: BTreeSet<String>,
    dropped_files: Vec<egui::DroppedFile>,
} */

/*

// self.update_events();
// wlog!("WrapApp::update");
// Give the area behind the floating windows a different color, because it looks better:

/*
#[cfg(target_arch = "wasm32")]
if let Some(anchor) = frame
    .info()
    .web_info
    .location
    .hash
    .strip_prefix('#')
    .and_then(Anchor::from_str_case_insensitive)
{
    self.state.selected_anchor = anchor;
}
*/

#[cfg(not(target_arch = "wasm32"))]
if ctx
    .input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11))
{
    let fullscreen =
        ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
        !fullscreen,
    ));
}

let mut cmd = Command::Nothing;
/*
egui::TopBottomPanel::top("wrap_app_top_bar")
    .frame(egui::Frame::new().inner_margin(4))
    .show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().button_frame = false;
            // self.bar_contents(ui, frame, &mut cmd);
        });
    });
    */

// self.state.backend_panel.update(ctx, frame);

// if !is_mobile(ctx) {
// cmd = self.backend_panel(ctx, frame);
// }

   */

/*
    self.run_cmd(ctx, cmd);
fn run_cmd(&mut self, ctx: &egui::Context, cmd: Command) {
    /*
    match cmd {
        Command::Nothing => {}
        Command::ResetEverything => {
            self.state = Default::default();
            ctx.memory_mut(|mem| *mem = Default::default());
        }
    }

     */
} */

/*
impl Cgfx_App_Main {
    /*
    fn show_selected_app(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        /*
        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor
                || ctx.memory(|mem| mem.everything_is_visible())
            {
                app.update(ctx, frame);
            }
        }
        */

        // tkx
        // self.demo.ui(ctx);
    }
    */

    /*
    fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
        use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
        use std::fmt::Write as _;

        // Preview hovering files:
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                for file in &i.raw.hovered_files {
                    if let Some(path) = &file.path {
                        write!(text, "\n{}", path.display()).ok();
                    } else if !file.mime.is_empty() {
                        write!(text, "\n{}", file.mime).ok();
                    } else {
                        text += "\n???";
                    }
                }
                text
            });

            let painter = ctx.layer_painter(LayerId::new(
                Order::Foreground,
                Id::new("file_drop_target"),
            ));

            let content_rect = ctx.content_rect();
            painter.rect_filled(
                content_rect,
                0.0,
                Color32::from_black_alpha(192),
            );
            painter.text(
                content_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }

        /*
        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
        */

        /*
        // Show dropped files (if any):
        if !self.dropped_files.is_empty() {
            let mut open = true;
            egui::Window::new("Dropped files").open(&mut open).show(
                ctx,
                |ui| {
                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info
                                .push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info
                                .push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info +=
                                &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                },
            );
            if !open {
                self.dropped_files.clear();
            }
        }
        */
    }
    */
}
*/
