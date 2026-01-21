use super::*;

#[derive(Serialize, Deserialize, Debug, JsData)]
pub enum N_Counter_ToS {
    Inc,
    Dec,
    Get,
    Set(i128),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum N_Counter_ToC {
    Value(i128),
}
