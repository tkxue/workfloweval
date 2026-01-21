#[allow(unused_imports)]
use super::*;

#[derive(Debug)]
pub enum Xos_Err {
    Mailbox_Timeout,
    Json_Err(JsData_Err),
}

impl From<JsData_Err> for Xos_Err {
    fn from(x: JsData_Err) -> Self {
        Xos_Err::Json_Err(x)
    }
}
