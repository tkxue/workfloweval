#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub use c_app_msg::*;
use e_api::*;
use leptos::prelude::*;
use leptos::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::XdomA;

mod id_vid;
pub use id_vid::*;

#[wasm_bindgen]
pub struct Rust_Vid_Ffi {
    main: Rust_Vid_Main,
}

#[wasm_bindgen]
impl Rust_Vid_Ffi {
    pub fn rust_vid_ffi__create(
        name: String,
        msg_code: wb::JsValue,
        args: wb::JsValue,
    ) -> Rust_Vid_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        let main = Rust_Vid_Main::main(name, msg_code, args);
        Rust_Vid_Ffi { main }
    }

    pub fn rust_vid_ffi__from_js(&self, msg: wb::JsValue) {}
}

pub struct Rust_Vid_Main {}

impl Rust_Vid_Main {}

impl Rust_Vid_Main {
    pub fn main(
        name: String,
        msg_code: wb::JsValue,
        _args: wb::JsValue,
    ) -> Rust_Vid_Main {
        Xdom_Logger::__set_loggers__wasm("h_vid.html".to_string());
        vid_main();
        Rust_Vid_Main {}
    }
}

#[component]
pub fn SimpleCounter(initial_value: i32, step: i32) -> impl IntoView {
    use leptos::prelude::Write;
    let (value, set_value) = signal(initial_value);
    view! {
        <div>
            <button on:click=move |_| set_value.set(0)>"Clear"</button>
            <button on:click=move |_| *set_value.write() -= step>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)
             class:red=move || value.get() % 2 == 1
            >"+1"</button>
        </div>
    }
}

#[component]
pub fn MainStatus(vid_choice: ReadSignal<Cmsg_Vid__Id_Slide>) -> impl IntoView {
    let on_click = move |_| {
        G_CmsgQ::send_oneshot(Cmsg_Inner::Present(Cmsg_Present::GotoSlide(
            Capp_Present__Id_Slide::Title_PythonRepl,
        )));
        G_CmsgQ::send_oneshot(Cmsg_Inner::H_vid(Cmsg_HVid::PlayVid(
            Cmsg_Vid__Id_Slide::title_got_audio_permission,
        )));
    };

    view! {
        {
            move || {
                match vid_choice.get() {
                Cmsg_Vid__Id_Slide::None =>  view! { }.into_any(),
                Cmsg_Vid__Id_Slide::Start =>  view! {
                    <button style:display="flex"
                    style:align-items="center"
                    style:font-size="48"
                    style:width="100vw"
                    style:justify-content="center"
                    style:height="100vh"
                    on:click=on_click
                    >Enable<br/>Avatar</button>
                }.into_any() ,
                x =>  view! {
                    <div style:width="100%" style:height="100%">
                    <video autoplay=true style:width="100%" controls=true >
                    <source src={format!("/pub/data/vids/{}.mp4", x.get_name())} type="video/mp4" />
                    </video>
                    <div> { Capp_Vid__Id_Slide::to_anyview(x) } </div>
                    </div>
                }.into_any() ,
            }
        }
        }
    }
}

pub fn vid_main() {
    let (vid_choice, set_vid_choice) = signal(Cmsg_Vid__Id_Slide::Start);

    XdomA::spawn_local(Box::pin(async move {
        loop {
            G_CmsgQ::wait_on().await;
            let msgs = G_CmsgQ::take_all();
            for m in msgs {
                match m.inner {
                    Cmsg_Inner::H_vid(cmsg_hvid) => match cmsg_hvid {
                        Cmsg_HVid::PlayVid(capp_vid_id_vid) => {
                            set_vid_choice.set(capp_vid_id_vid);
                        }
                    },
                    _ => {
                        wlog!("h_vid processing: {:?}", m)
                    }
                }
            }
        }
    }));

    mount_to_body(move || {
        view! {
            <MainStatus vid_choice=vid_choice />
        }
    });
}
