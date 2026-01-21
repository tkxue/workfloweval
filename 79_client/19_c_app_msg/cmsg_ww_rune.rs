use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_WwRune {
    ReplEval { cmd: String },
}
