#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use c_app_base::*;
use c_egui_base::Cmsg_Repl_Rune;
use e_api::*;

mod c_log_viewer;
mod c_python_repl;
mod c_rune_repl;
mod code_editor;
pub use c_log_viewer::*;
pub use c_python_repl::*;
pub use c_rune_repl::*;
pub use code_editor::*;

// ----------------------------------------------------------------------------

pub struct Capp_Repl {
    pub entrys: CircularVec<Vec<String>>,
    pub error: Option<String>,
}

impl Capp_Repl {
    pub fn new() -> Capp_Repl {
        Capp_Repl {
            entrys: CircularVec::new(1000),
            error: None,
        }
    }

    pub fn push(&mut self, t: Result<Vec<String>, String>) {
        match t {
            Ok(v) => {
                self.error = None;
                self.entrys.push(v);
            }
            Err(e) => self.error = Some(e),
        }
    }
}
