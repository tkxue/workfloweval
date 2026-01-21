use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Present {
    GotoSlide(Capp_Present__Id_Slide),
}

#[repr(u16)]
#[derive(BigEnum, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Capp_Present__Id_Slide {
    Info,
    Title_PythonRepl,
}

impl T_BigEnum for Capp_Present__Id_Slide {}
