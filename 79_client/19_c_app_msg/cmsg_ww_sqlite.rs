use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_WwSqlite {
    ReplEval { cmd: String },
}
