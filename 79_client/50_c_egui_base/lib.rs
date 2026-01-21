#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use e_api::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{Id_Proc, Msg_Code, Web_Root, my_ffi};

pub mod demos;

pub mod ws;

pub mod egui_demo_app;
pub mod egui_demo_lib;
pub mod egui_extras;
pub mod egui_ltree_examples;
pub mod egui_ltree_lib;
pub mod rad;

pub use c_app_msg::*;
