#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use n_msg::*;
use s_shared::_G_S_Ffi;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

mod s_doc_search;
mod s_resumee;
pub use s_doc_search::*;
pub use s_resumee::*;

mod s_app_counter;
pub use s_app_counter::*;

pub struct S_App {}

pub static _G_S_App: OnceLock<S_App> = OnceLock::new();

pub struct G_S_App {}

impl S_App {
    pub fn in_to_path(x: &N_ToS_Inner) -> &'static str {
        match x {
            N_ToS_Inner::Counter(_) => "test-counter",
        }
    }

    pub async fn handle(&self, aux: &N_ToS_Aux, inner: &N_ToS_Inner) -> Result<N_ToC_Inner, Sa_Err> {
        match inner {
            N_ToS_Inner::Counter(msg) => {
                let res = SApp_Counter::handle(Self::in_to_path(inner), aux, msg).await?;
                Ok(N_ToC_Inner::Counter(res))
            }
        }
    }
}

impl G_S_App {
    pub fn __init_once() {
        if _G_S_App.get().is_none() {
            let g = S_App {};
            let _ = _G_S_App.set(g);
        }
    }

    pub async fn handle(x: N_ToS_Full) -> Result<N_ToC_Inner, Sa_Err> {
        println!("G_S_App::handle: {:?}", x);
        _G_S_App.get().unwrap().handle(&x.aux, &x.inner).await
    }
}
