use super::*;

#[repr(u16)]
#[derive(Eq, PartialEq, Hash, BigEnum, Copy, Clone)]
pub enum Cgfx_App_Id {
    Repl_Rune,
    Repl_Python,
    Repl_Sqlite,
    Sheet,
    Street,
}

impl T_BigEnum for Cgfx_App_Id {}

pub trait Cgfx_App_T {
    fn name(&self) -> &'static str;
    fn update(&mut self, ctx: &egui::Ui);
}
