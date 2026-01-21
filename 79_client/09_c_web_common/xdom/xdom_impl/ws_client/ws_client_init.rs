use super::*;

pub struct Ws_Client_Init {
    pub url: String,
    pub cb: CbArc<Ws_Event>,
}
