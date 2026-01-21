use super::*;

use c_app_repl::*;
use c_egui_base::{Cmsg_Full, Cmsg_Inner, Cmsg_Type, Cmsg_WwRune, G_CmsgQ};
use e_api::*;
use egui::scroll_area::ScrollBarVisibility;
use egui::{PointerButton, ScrollArea, TextEdit};

pub struct Cgfx_Repl_Rune {
    is_open: bool,
    language: String,
    code: String,
    text: String,
    completer: Completer,
    repl: Capp_Repl,
}

impl Cgfx_App_T for Cgfx_Repl_Rune {
    fn name(&self) -> &'static str {
        "Rune Repl"
    }

    fn update(&mut self, ui: &egui::Ui) {
        use c_egui_base::egui_demo_lib::View as _;
        let ctx = ui.ctx();
        egui::Window::new(self.name())
            .open(&mut self.is_open)
            .max_height(600.0)
            .default_height(600.0)
            .show(ctx, |ui| 
                {

        let t = &self.repl;

        let Self { language, code, .. } = self;

        let theme = c_egui_base::egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

        let mut layouter =
            |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut layout_job =
                    c_egui_base::egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        ui.style(),
                        &theme,
                        buf.as_str(),
                        language,
                    );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts_mut(|f| f.layout_job(layout_job))
            };

        let history = t.entrys.get();

        let row_height = ui.spacing().interact_size.y;
        let total_rows = history.len();

        ui.label("Output History:");

        match &t.error {
            None => {
                ScrollArea::vertical()
                    .id_source("repl history")
                    .max_height(200.0)
                    .min_scrolled_height(200.0)
                    .max_width(800.0)
                    .stick_to_bottom(true)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                    .auto_shrink([false; 2])
                    .show_rows(ui, row_height, total_rows, |ui, row_range| {
                        ui.set_width(800.0);
                        for i in row_range {
                            for j in &history[i] {
                                ui.label(j);
                            }
                        }
                    });
            }
            Some(err) => {
                ScrollArea::vertical()
                    .id_source("repl history")
                    .max_height(200.0)
                    .min_scrolled_height(200.0)
                    .max_width(800.0)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.label("Error:");
                        ui.label(err);
                    });
            }
        }

        ui.separator();

        let mut editor = CodeEditor::default()
            .id_source("code editor")
            .with_rows(10)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX_LIGHT)
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .with_numlines_shift(0)
            .with_numlines_only_natural(false)
            .vscroll(true)
            .stick_to_bottom(true);

        let r = editor
            .show_with_completer(ui, &mut self.code, &mut self.completer)
            .response; // .response

        if r.has_focus() {
            if ui.input_mut(|i| {
                i.consume_key(egui::Modifiers::COMMAND, egui::Key::Enter)
            }) {
                G_CmsgQ::send_oneshot(Cmsg_Inner::Ww_rune(
                    Cmsg_WwRune::ReplEval {
                        cmd: format!(
                            "pub fn main(sdata, env) {{\n{}\n}}\n",
                            self.code.to_string()
                        ),
                    },
                ))
            }
        }

        ui.separator();
        ui.horizontal(|h| {
            h.label("Auto-complete TextEdit::singleLine");
            self.completer.show_on_text_widget(
                h,
                &Syntax::simple("#"),
                &ColorTheme::default(),
                |ui| {
                    TextEdit::singleline(&mut self.text)
                        .lock_focus(true)
                        .show(ui)
                },
            );
            if h.button("add words").clicked() {
                for word in self.text.split_whitespace() {
                    let word = word.replace(
                        |c: char| !(c.is_alphanumeric() || c == '_'),
                        "",
                    );
                    self.completer.push_word(&word);
                }
            }
        });
        ui.separator();
                }
                
            );
    }
}

impl Cgfx_Repl_Rune {
    pub fn new() -> Self {
        Self {
            is_open: true,
            language: "rs".into(),
            code: "2 + 99".into(),
            text: String::default(),
            completer: Completer::new_with_syntax(&Syntax::rust())
                .with_user_words(),
            repl: Capp_Repl::new(),
        }
    }

}
