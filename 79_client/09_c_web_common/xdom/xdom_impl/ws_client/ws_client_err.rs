use super::*;

#[derive(Debug)]
pub enum Ws_Client_Err {
    Send_On_Closed,
    Send_On_Trying,
    Not_Ready,
    Unknown(String),
}

impl Display for Ws_Client_Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
