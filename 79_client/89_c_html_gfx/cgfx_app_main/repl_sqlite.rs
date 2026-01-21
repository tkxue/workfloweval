use super::*;

use c_app_repl::*;
use c_egui_base::{Cmsg_Full, Cmsg_Inner, Cmsg_Type, Cmsg_WwSqlite, G_CmsgQ};
use e_api::*;
use egui::scroll_area::ScrollBarVisibility;
use egui::{PointerButton, ScrollArea, TextEdit};

pub struct Cgfx_Repl_Sqlite {
    is_open: bool,
    language: String,
    code: String,
    completer: Completer,
    repl: Capp_Repl,
}

impl Cgfx_App_T for Cgfx_Repl_Sqlite {
    fn name(&self) -> &'static str {
        "Sqlite Repl"
    }

    fn update(&mut self, ui: &egui::Ui) {
        use c_egui_base::egui_demo_lib::View as _;
        let ctx = ui.ctx();
        let max_rect = ui.max_rect();
        egui::Window::new(self.name())
            .open(&mut self.is_open)
            .constrain_to(max_rect)
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

        ui.label("Sqlite Output History:");

        match &t.error {
            None => {
                ScrollArea::vertical()
                    .id_salt("python repl history")
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
                    .id_salt("python repl history")
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
            .id_source("sqlite code editor")
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
                G_CmsgQ::send_oneshot(Cmsg_Inner::Ww_sqlite(
                    Cmsg_WwSqlite::ReplEval {
                        cmd: self.code.to_string(),
                    },
                ))
            }
        }
                }
                
            );
    }
}

impl Cgfx_Repl_Sqlite {
    pub fn new() -> Self {
        Self {
            is_open: true,
            language: "sql".into(),
            code: "2 + 3".into(),
            completer: Completer::new_with_syntax(&Syntax::rust())
                .with_user_words(),
            repl: Capp_Repl::new(),
        }
    }

    pub fn process_msg(&mut self, msg: Cmsg_Repl_Sqlite) {
        match msg {
            Cmsg_Repl_Sqlite::Output(x) => {
                self.repl.push(x);
            }
        }
    }

}


/*

CREATE TABLE students
  (id INTEGER PRIMARY KEY,
   name TEXT,
   grade INTEGER,
   gpa FLOAT,
   tardies INTEGER);



INSERT INTO students (name, grade, gpa, tardies)
  VALUES ("Jake", 12, 3.5, 4);
INSERT INTO students (name, grade, gpa, tardies)
  VALUES ("Emily", 10, 3.7, 2);
INSERT INTO students (name, grade, gpa, tardies)
  VALUES ("Sam", 11, 3.5, 3);
INSERT INTO students (name, grade, gpa, tardies)
  VALUES ("Jordan", 10, 3.6, 2);
INSERT INTO students (name, grade, gpa, tardies)
  VALUES ("Victoria", 9, 3.2, 3);



SELECT * FROM students









*/