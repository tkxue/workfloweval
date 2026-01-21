use super::*;
// use steel::SteelVal;
// use steel::steel_vm::engine::Engine;
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{Id_Proc, Msg_Code, Web_Root, my_ffi};

pub struct Rust_Logic_Main {}

#[wasm_bindgen]
pub struct Rust_Logic_Ffi {}

#[wasm_bindgen]
impl Rust_Logic_Ffi {
    pub fn rust_logic_ffi__create(name: String, msg_code: wb::JsValue, args: wb::JsValue) -> Rust_Logic_Ffi {
        console_error_panic_hook::set_once();
        Xdom_Logger::__set_loggers__wasm(name.clone());
        wasm_init();
        Rust_Logic_Main::main(name, msg_code, args);
        Rust_Logic_Ffi {}
    }
}

impl Rust_Logic_Main {}

impl Rust_Logic_Main {
    pub fn main(name: String, msg_code: wb::JsValue, _args: wb::JsValue) -> Rust_Logic_Main {
        let mut msg_code = Msg_Code::new(msg_code.clone());
        let name = name.clone();

        if web_common::XdomA::is_iframe() {
            msg_code.window_location_host = Arc::new(my_ffi::get_window_host_safe().unwrap())
        };

        wlog!("Rust_Logic_Main::main: {:?}", name);

        match name.as_ref() {
            "h_vid" => {
                Xdom_Logger::__set_loggers__wasm("h_vid".to_string());
                Rust_Logic_Main {}
            }

            "ww_net" => {
                Xdom_Logger::__set_loggers__wasm("ww_net".to_string());
                Ww_Net::start_loop();
                Rust_Logic_Main {}
            }

            s => {
                wlog!("unknown: {:?}", s);
                damn_it!("")
            }
        }
    }
}
