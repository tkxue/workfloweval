#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use e_api::*;
use e_include::MsgQueue;
use js_sys::wasm_bindgen::JsValue;
use js_sys::Function;
use n_msg::N_ToS_Full;
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::sync::{Arc, Mutex, OnceLock};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// [0] = tag JsArrayBuffer
// [1] = data: JsArrayBuffer
// [2 .. ] = transfers: JsArrayBuffer

mod untyped_msgq;
pub use untyped_msgq::*;

mod cmsg;
mod cmsg_h_gfx;
mod cmsg_h_index;
mod cmsg_h_vid;
mod cmsg_present;
mod cmsg_ww_net;
mod cmsg_ww_python;
mod cmsg_ww_rune;
mod cmsg_ww_sheet;
mod cmsg_ww_sqlite;
pub use cmsg::*;
pub use cmsg_h_gfx::*;
pub use cmsg_h_index::*;
pub use cmsg_h_vid::*;
pub use cmsg_present::*;
pub use cmsg_ww_net::*;
pub use cmsg_ww_python::*;
pub use cmsg_ww_rune::*;
pub use cmsg_ww_sheet::*;
pub use cmsg_ww_sqlite::*;

mod cond_var;
pub use cond_var::*;
