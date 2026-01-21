use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_WwPython {
    ReplEval { cmd: String },
}
