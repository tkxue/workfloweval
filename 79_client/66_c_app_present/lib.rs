#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use c_app_msg::*;
use c_egui_base::{C_LogEntry, Cmsg_Inner, Cmsg_Present, Cmsg_WwRune, G_CmsgQ};
use e_api::*;
use egui::{PointerButton, ScrollArea};

mod present;
mod slides;
pub use present::*;
pub use slides::*;

pub struct G_Capp_Present {
    cur_slide: Capp_Present__Id_Slide,
    slides: HashMap<Capp_Present__Id_Slide, ArcState<Box<dyn Capp_Present_Slide_T>>>,
}

pub static _G_Capp_Present: OnceLock<ArcState<G_Capp_Present>> = OnceLock::new();

impl G_Capp_Present {
    pub fn new() -> G_Capp_Present {
        G_Capp_Present {
            cur_slide: Capp_Present__Id_Slide::Info,
            slides: vec![
                (
                    Capp_Present__Id_Slide::Info,
                    ArcState::new(Box::new(Capp_Present_Slide__Info::new()) as Box<dyn Capp_Present_Slide_T>),
                ),
                (
                    Capp_Present__Id_Slide::Title_PythonRepl,
                    ArcState::new(Box::new(Capp_Present_Slide__Title::new())),
                ),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        }
    }

    pub fn __init() {
        let _ = _G_Capp_Present.set(ArcState::new(G_Capp_Present::new()));
    }

    pub fn read<T, F: FnOnce(&G_Capp_Present) -> T>(f: F) -> T {
        _G_Capp_Present.get().unwrap().read(f)
    }

    pub fn update<T, F: FnOnce(&mut G_Capp_Present) -> T>(f: F) -> T {
        _G_Capp_Present.get().unwrap().update(f)
    }

    pub fn process(&mut self, msg: Cmsg_Present) {
        match msg {
            Cmsg_Present::GotoSlide(capp_present_id_slide) => self.cur_slide = capp_present_id_slide,
        }
    }
}

wasm_init! {
G_Capp_Present::__init();
}
