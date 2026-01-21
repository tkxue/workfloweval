#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use e_api::byteorder::{LittleEndian, WriteBytesExt};
use e_api::js_sys::Uint8Array;
use e_api::*;
use serde::{Deserialize, Serialize};

mod n_client;
mod n_counter;
mod n_server;
pub use n_client::*;
pub use n_counter::*;
pub use n_server::*;

#[derive(Deserialize, Serialize, JsData, Debug)]
pub enum Sa_ErrType {
    NotFound,
    DeserializeFail,
    Fdb(String),
}

#[derive(Deserialize, Serialize, JsData, Debug)]
pub struct Sa_Err {
    pub ctxs: Vec<String>,
    pub err: Sa_ErrType,
}
