use super::*;

#[derive(Debug)]
pub enum Ws_Event {
    Open,
    Error(String),
    Close,
    // Close(web_sys::CloseEvent),
    Msg(Ws_Msg_In),
}
