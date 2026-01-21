use super::*;

use web_common::Id_Proc;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_HGfx {
    LogViewer(Cmsg_LogViewer),
    Repl_Rune(Cmsg_Repl_Rune),
    Repl_Python(Cmsg_Repl_Python),
    Repl_Sqlite(Cmsg_Repl_Sqlite),
    Sheet(Cmsg_Sheet),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct C_LogEntry {
    pub id_proc: Id_Proc,
    pub msg: String,
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_LogViewer {
    LogViewer(C_LogEntry),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Repl_Rune {
    Output(Result<Vec<String>, String>),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Repl_Python {
    Output(Result<Vec<String>, String>),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Repl_Sqlite {
    Output(Result<Vec<String>, String>),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct Cmsg_Sheet__Cell_Data {
    pub index: u32,
    pub row: i32,
    pub column: i32,
    pub data: Result<String, String>,
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_Sheet {
    Loaded(Result<Vec<Cmsg_Sheet__Cell_Data>, String>),
}
