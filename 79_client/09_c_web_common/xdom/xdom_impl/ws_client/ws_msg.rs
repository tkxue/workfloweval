use super::*;

#[derive(Debug, Clone)]
pub enum Ws_Msg_In {
    Evt_Open,
    Text(Rc<String>),
    Binary(Rc<Vec<u8>>),
    // Unknown(web_sys::MessageEvent),
    // Blob(web_sys::Blob),
}

#[derive(Debug)]
pub enum Ws_Msg_Out<'a> {
    Text(Rc<String>),
    Binary(&'a [u8]),
    // Unknown(web_sys::MessageEvent),
    // Blob(web_sys::Blob),
}
