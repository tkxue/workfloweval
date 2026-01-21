use super::*;

mod cgfx_app_main_ptr;
mod cgfx_app_util;
mod repl_python;
mod repl_rune;
mod repl_sqlite;
mod sheet;
mod street;
pub use cgfx_app_main_ptr::*;
pub use cgfx_app_util::*;
pub use repl_python::*;
pub use repl_rune::*;
pub use repl_sqlite::*;
pub use sheet::*;
pub use street::*;

pub struct Cgfx_App_Main {
    pub repl_rune: Cgfx_Repl_Rune,
    pub repl_python: Cgfx_Repl_Python,
    pub repl_sqlite: Cgfx_Repl_Sqlite,
    pub sheet: Cgfx_Sheet,
    pub street: Cgfx_Street,
}

impl Cgfx_App_Main {
    pub fn get(&self, id: Cgfx_App_Id) -> &dyn Cgfx_App_T {
        match id {
            Cgfx_App_Id::Repl_Rune => &self.repl_rune,
            Cgfx_App_Id::Repl_Python => &self.repl_python,
            Cgfx_App_Id::Repl_Sqlite => &self.repl_sqlite,
            Cgfx_App_Id::Sheet => &self.sheet,
            Cgfx_App_Id::Street => &self.street,
        }
    }

    pub fn get_mut(&mut self, id: Cgfx_App_Id) -> &mut dyn Cgfx_App_T {
        match id {
            Cgfx_App_Id::Repl_Rune => &mut self.repl_rune,
            Cgfx_App_Id::Repl_Python => &mut self.repl_python,
            Cgfx_App_Id::Repl_Sqlite => &mut self.repl_sqlite,
            Cgfx_App_Id::Sheet => &mut self.sheet,
            Cgfx_App_Id::Street => &mut self.street,
        }
    }

    pub fn process_h_gfx(&mut self, msg: Cmsg_HGfx) {
        match msg {
            Cmsg_HGfx::Repl_Python(cmsg_repl_python) => {
                self.repl_python.process_msg(cmsg_repl_python);
            }
            Cmsg_HGfx::LogViewer(cmsg_log_viewer) => todo!(),
            Cmsg_HGfx::Repl_Rune(cmsg_repl_rune) => todo!(),
            Cmsg_HGfx::Repl_Sqlite(cmsg_repl_sqlite) => {
                self.repl_sqlite.process_msg(cmsg_repl_sqlite)
            }
            Cmsg_HGfx::Sheet(cmsg_sheet) => self.sheet.process_msg(cmsg_sheet),
        }
    }
}
