use super::*;

use std::default::Default;

use c_app_ssheet::shared::spreadsheet::ui::SpreadsheetState;
use c_app_ssheet::shared::{
    sparse::ui::SparseTableState, spreadsheet::SpreadsheetSource,
};
use c_egui_base::egui_ltree_examples::large_tree::LTree_Node;
use egui::{Rect, Ui, UiBuilder, ViewportBuilder};

mod left_sidebar;
pub use left_sidebar::*;

pub struct Cgfx_Sheet {
    is_open: bool,
    state: SpreadsheetState,
    // fnames: Vec<&'static str>,
    ltree: c_egui_base::egui_ltree_examples::large_tree::MyApp,
}

impl Cgfx_Sheet {
    pub fn process_msg(&mut self, msg: Cmsg_Sheet) {
        use c_app_ssheet::shared::spreadsheet::value::*;

        wlog!("processing a msg");

        match msg {
            Cmsg_Sheet::Loaded(Err(x)) => {
                wlog!("Err: {:?}", x);
            }
            Cmsg_Sheet::Loaded(Ok(v)) => {
                self.state.data_source = SpreadsheetSource::new();
                let ds = &mut self.state.data_source;

                for cd in v.into_iter() {
                    if cd.row < 100 && cd.column < 100 {
                        let msg = match cd.data {
                            Ok(v) => v,
                            Err(e) => format!("err: {:?}", e),
                        };

                        ds.data[cd.row as usize][cd.column as usize] =
                            CellValue::Value(Value::Text(msg));
                    }
                }
            }
        }
    }

    pub fn new() -> Self {
        let node = Vfs_Node::build_all().to_ltree_node();
        let ltree =
            c_egui_base::egui_ltree_examples::large_tree::MyApp::new(node);
        Self {
            is_open: true,
            state: SpreadsheetState::default(),
            ltree,
        }
    }

    pub fn draw_content(&mut self, ui: &mut Ui) {
        let state = &mut self.state;
        let (_response, actions) =
            c_app_ssheet::shared::spreadsheet::ui::show_table(ui, state);
        c_app_ssheet::shared::spreadsheet::ui::handle_actions(actions, state);
        ui.separator();
        ui.label("content below");
    }

    pub fn draw_left(&mut self, ui: &mut Ui) {
        self.ltree.update(ui);
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.set_min_size(ui.available_size());

        let rect = ui.max_rect();

        let (left, right) = rect.split_left_right_at_x(rect.min.x + 300.);

        ui.scope_builder(UiBuilder::new().max_rect(left), |ui| {
            self.draw_left(ui);
        });

        ui.scope_builder(UiBuilder::new().max_rect(right), |ui| {
            self.draw_content(ui);
        });
    }
}

impl Cgfx_App_T for Cgfx_Sheet {
    fn name(&self) -> &'static str {
        "My_Calc"
    }

    fn update(&mut self, ui: &egui::Ui) {
        let max_rect = ui.max_rect();
        let mut open = self.is_open;
        egui::Window::new(self.name())
            .open(&mut open)
            .min_size((1300.0, 800.0))
            .max_size((1300.0, 800.0))
            .default_size((1300.0, 800.0))
            .constrain_to(max_rect)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(ui.available_size());
                self.ui(ui);
            });
        self.is_open = open;
    }
}
