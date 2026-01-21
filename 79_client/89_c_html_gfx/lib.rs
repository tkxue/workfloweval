#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use c_app_msg::*;
use c_app_present::*;
use c_egui_base::demos::{Push_Take, Tea_App_T};
use c_egui_base::egui_demo_app;
use c_egui_base::egui_demo_app::wrap_app::*;
use c_egui_base::egui_demo_lib::{is_mobile, Demo};
use e_api::*;
use eframe::{App, CreationContext};
use egui::Context;
use std::collections::BTreeSet;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_common::XdomA;

pub mod cgfx_app_main;

mod blah;
mod demo_app;
pub use blah::*;
pub use demo_app::*;

use crate::cgfx_app_main::Cgfx_App_Main;

#[wasm_bindgen]
pub struct Rust_Gfx_Ffi {}

#[wasm_bindgen]
impl Rust_Gfx_Ffi {
    pub fn rust_gfx_ffi__create(
        name: String,
        msg_code: wb::JsValue,
        args: wb::JsValue,
    ) -> Rust_Gfx_Ffi {
        console_error_panic_hook::set_once();
        Xdom_Logger::__set_loggers__wasm(name.clone());
        wasm_init();
        let main = Rust_Gfx_Main::main(name, msg_code, args);
        Rust_Gfx_Ffi {}
    }

    pub fn rust_gfx_ffi__from_js(&self, msg: wb::JsValue) {}
}

pub struct Rust_Gfx_Main {}

impl Rust_Gfx_Main {}

impl Rust_Gfx_Main {
    pub fn main(
        name: String,
        msg_code: wb::JsValue,
        _args: wb::JsValue,
    ) -> Rust_Gfx_Main {
        Xdom_Logger::__set_loggers__wasm("h_gfx.html".to_string());

        let (mut sender, recver) =
            async_oneshot::oneshot::<cgfx_app_main::Cgfx_App_Main_Ptr>();
        let need_redraw = AsyncCondVar::new();

        XdomA::spawn_local(Box::pin({
            let need_redraw = need_redraw.clone();
            async move {
                let app_main = recver.await.unwrap();
                loop {
                    let _ = G_CmsgQ::wait_on().await;
                    let msgs = G_CmsgQ::take_all();
                    for msg in msgs {
                        match msg.inner {
                            Cmsg_Inner::H_gfx(msg) => {
                                app_main.inner.update(|x| {
                                    x.process_h_gfx(msg);
                                })
                            }
                            Cmsg_Inner::Present(msg) => {
                                wlog!("Cmsg_Inner::Present");
                            }
                            _ => {
                                wlog!("ww_rune: msg: wrong msg delivery");
                                continue;
                            }
                        }
                        need_redraw.notify();
                    }
                }
            }
        }));

        {
            let need_redraw = need_redraw.clone();
            c_egui_base::demos::run(move |cc: &CreationContext| {
                let app_main =
                    cgfx_app_main::Cgfx_App_Main_Ptr::new(cc.egui_ctx.clone());
                sender.send(app_main.clone()).unwrap();
                {
                    let t = cc.egui_ctx.clone();
                    let need_redraw = need_redraw.clone();
                    XdomA::spawn_local(Box::pin(async move {
                        loop {
                            need_redraw.wait_on().await;
                            t.request_repaint();
                        }
                    }));
                }
                c_egui_base::egui_extras::install_image_loaders(&cc.egui_ctx);
                app_main.clone()
            });
        }

        Rust_Gfx_Main {}
    }
}
