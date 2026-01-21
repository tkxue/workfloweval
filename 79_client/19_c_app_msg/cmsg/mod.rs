use super::*;
use js_sys::Uint8Array;
use n_msg::N_ToC_Inner;
use web_common::{Id_Proc, Xos_Js_Array};

mod cmsg_full;
pub use cmsg_full::*;

#[derive(
    Serialize, Deserialize, JsData, Debug, Hash, PartialEq, Eq, Copy, Clone,
)]
pub struct Cmsg_MailboxId {
    pub id_proc: Id_Proc,
    pub idx: u128,
}

#[derive(
    Serialize, Deserialize, JsData, Debug, Hash, PartialEq, Eq, Copy, Clone,
)]
pub enum Cmsg_Type {
    Oneshot,
    NeedReply(Cmsg_MailboxId),
    ReplyTo(Cmsg_MailboxId),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Inner {
    H_vid(Cmsg_HVid),
    H_index(Cmsg_HIndex),
    H_gfx(Cmsg_HGfx),
    Ww_net(Cmsg_WwNet),
    Ww_rune(Cmsg_WwRune),
    Ww_python(Cmsg_WwPython),
    Ww_sqlite(Cmsg_WwSqlite),
    Ww_sheet(Cmsg_WwSheet),
    To_server(N_ToS_Full),
    Present(Cmsg_Present),
}

impl Cmsg_Inner {
    pub fn id_proc(&self) -> Id_Proc {
        match self {
            Cmsg_Inner::H_vid(_) => Id_Proc::H_Vid,
            Cmsg_Inner::H_index(_) => Id_Proc::H_Index,
            Cmsg_Inner::H_gfx(_) => Id_Proc::H_Gfx,
            Cmsg_Inner::Ww_net(_) => Id_Proc::Ww_Net,
            Cmsg_Inner::Ww_rune(_) => Id_Proc::Ww_Rune,
            Cmsg_Inner::Ww_python(_) => Id_Proc::Ww_Python,
            Cmsg_Inner::Ww_sqlite(_) => Id_Proc::Ww_Sqlite,
            Cmsg_Inner::Ww_sheet(_) => Id_Proc::Ww_Sheet,
            Cmsg_Inner::To_server(_) => Id_Proc::Ww_Net,
            Cmsg_Inner::Present(_) => Id_Proc::H_Gfx,
        }
    }
}
