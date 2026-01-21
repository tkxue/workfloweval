#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{CodeEditor, ColorTheme, Completer, Syntax, Token};
use c_egui_base::egui_demo_lib::Demo;
use eframe::{self, CreationContext, egui};
use egui::{Context, TextEdit};
// use egui_code_editor::{self, CodeEditor, ColorTheme, Completer, Syntax, highlighting::Token};

#[derive(Default)]
pub struct CodeEditorDemo {
    code: String,
    text: String,
    completer: Completer,
}
impl CodeEditorDemo {
    pub fn new() -> Self {
        CodeEditorDemo {
            code: "".to_string(),
            text: String::default(),
            completer: Completer::new_with_syntax(&Syntax::rust()).with_user_words(),
        }
    }
}

impl Demo for CodeEditorDemo {
    fn name(&self) -> &'static str {
        "CodeEditorDemo"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use c_egui_base::egui_demo_lib::View as _;
        egui::Window::new(self.name())
            .open(open)
            .max_height(600.0)
            .default_height(600.0)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl CodeEditorDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
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

        editor.show_with_completer(ui, &mut self.code, &mut self.completer); // .response

        ui.separator();
        ui.horizontal(|h| {
            h.label("Auto-complete TextEdit::singleLine");
            self.completer
                .show_on_text_widget(h, &Syntax::simple("#"), &ColorTheme::default(), |ui| {
                    TextEdit::singleline(&mut self.text).lock_focus(true).show(ui)
                });
            if h.button("add words").clicked() {
                for word in self.text.split_whitespace() {
                    let word = word.replace(|c: char| !(c.is_alphanumeric() || c == '_'), "");
                    self.completer.push_word(&word);
                }
            }
        });
        ui.separator();
    }
}
