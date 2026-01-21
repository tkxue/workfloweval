use super::*;

use e_api::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

mod g_ws_msgq;
pub use g_ws_msgq::*;
use web_common::XdomA;

pub struct Ww_Net {}

#[derive(Debug)]
pub struct Foo {
    v: i128,
}

impl Ww_Net {
    pub fn start_loop() {
        G_WsMsgQ::send_oneshot(N_ToS_Aux {}, N_ToS_Inner::Counter(N_Counter_ToS::Inc));
        XdomA::spawn_local(Box::pin(async {
            G_WsMsgQ::wait_on().await;
            let t = G_WsMsgQ::take_all();
            wlog!("{:?}", t);
        }));
    }
}
