#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub use c_app_msg::*;
use e_api::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{Id_Proc, Msg_Code, Web_Root, my_ffi};

mod rune_repl;

mod ww_rune;
pub use ww_rune::*;

pub struct Rust_Rune_Main {
    // pub(crate) _po_cbs: Arc<PO>,
}

#[wasm_bindgen]
pub struct Rust_Rune_Ffi {}

#[wasm_bindgen]
impl Rust_Rune_Ffi {
    pub fn rust_rune_ffi__create(name: String, msg_code: wb::JsValue, args: wb::JsValue) -> Rust_Rune_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        Rust_Rune_Main::main(name, msg_code, args);
        Rust_Rune_Ffi {}
    }
}

// #[wasm_bindgen]
impl Rust_Rune_Main {}

impl Rust_Rune_Main {
    pub fn main(name: String, msg_code: wb::JsValue, _args: wb::JsValue) -> Rust_Rune_Main {
        let mut msg_code = Msg_Code::new(msg_code.clone());
        let name = name.clone();

        if web_common::XdomA::is_iframe() {
            msg_code.window_location_host = Arc::new(my_ffi::get_window_host_safe().unwrap())
        };

        let root_url = Arc::new(Web_Root::new(
            msg_code.window_location_protocol.clone(),
            msg_code.window_location_host.clone(),
            msg_code.is_dev(),
            msg_code.wasm_version.clone(),
        ));

        match name.as_ref() {
            "ww_rune" => {
                Xdom_Logger::__set_loggers__wasm("ww_rune".to_string());
                Ww_Rune::start_loop();
                Rust_Rune_Main {}
            }
            s => {
                wlog!("unknown: {:?}", s);
                damn_it!("")
            }
        }
    }
}
