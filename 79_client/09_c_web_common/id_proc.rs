use super::*;

use e_api::*;
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Hash, Clone, Copy, Eq, PartialEq, BigEnum, Serialize, Deserialize)]
pub enum Id_Proc {
    H_Gfx,
    H_Index,
    H_Vid,
    Ww_Net,
    Ww_Rune,
    Ww_Python,
    Ww_Sqlite,
    Ww_Sheet,
}

impl Id_Proc {
    pub fn to_name(&self) -> &str {
        match self {
            Id_Proc::H_Gfx => "h_gfx",
            Id_Proc::H_Index => "h_index",
            Id_Proc::H_Vid => "h_vid",
            Id_Proc::Ww_Net => "ww_net",
            Id_Proc::Ww_Rune => "ww_rune",
            Id_Proc::Ww_Python => "ww_python",
            Id_Proc::Ww_Sqlite => "ww_sqlite",
            Id_Proc::Ww_Sheet => "ww_sheet",
        }
    }
}

impl T_BigEnum for Id_Proc {}
