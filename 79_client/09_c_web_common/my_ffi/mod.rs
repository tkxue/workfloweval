use super::*;

use std::task::Poll;

pub mod blob;
pub mod message_event;
pub mod window;

use wb::prelude::*;


use super::*;

pub mod message_channel {
    use crate::js_sys;
    use crate::wb::JsValue;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(extends = js_sys::Object, js_name = MessageChannel)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type MessageChannel;

        #[wasm_bindgen(constructor, js_class = "MessageChannel")]
        pub fn new() -> MessageChannel;

        #[wasm_bindgen(method, getter, js_name = port1)]
        pub fn port1(this: &MessageChannel) -> MessagePort;

        #[wasm_bindgen(method, getter, js_name = port2)]
        pub fn port2(this: &MessageChannel) -> MessagePort;

        #[wasm_bindgen(extends = js_sys::Object, js_name = MessagePort)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub type MessagePort;

        #[wasm_bindgen(method, js_name = postMessage)]
        pub fn post_message(this: &MessagePort, message: &JsValue);

        #[wasm_bindgen(method, js_name = start)]
        pub fn start(this: &MessagePort);

        #[wasm_bindgen(method, js_name = close)]
        pub fn close(this: &MessagePort);

        /// Sets the `onmessage` callback
        #[wasm_bindgen(method, setter)]
        pub fn set_onmessage(this: &MessagePort, cb: &js_sys::Function);

        // Overloaded postMessage with Transferables
        // In JS: port.postMessage(message, [transferable])
        #[wasm_bindgen(method, js_name = postMessage)]
        pub fn post_message_with_transfer(this: &MessagePort, message: &JsValue, transfer: &js_sys::Array);

    }
}


mod raw {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        // This binds to the global console.log function
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &JsValue);

        #[wasm_bindgen(js_name = WorkerGlobalScope)]
        type WorkerGlobalScope;

        #[wasm_bindgen(js_name = globalThis)]
        pub static GLOBAL: JsValue;

        #[wasm_bindgen(js_name = Window)]
        type Window;

        // 3. Bind to the host property via the window.location namespace
        #[wasm_bindgen(js_namespace = ["window", "location"], js_name = host)]
        pub static WINDOW_HOST: JsValue;

    }
}

pub fn log(s: &JsValue) {
    raw::log(s)
}

pub fn is_web_worker() -> bool {
    // Check if WorkerGlobalScope is defined in the global scope
    !crate::js_sys::Reflect::get(&raw::GLOBAL, &"WorkerGlobalScope".into())
        .unwrap_or(crate::wb::JsValue::UNDEFINED)
        .is_undefined()
}

pub fn get_window_host_safe() -> Option<String> {
    // Check if 'Window' exists in the global scope
    let is_window_available = !crate::js_sys::Reflect::get(&raw::GLOBAL, &"Window".into())
        .unwrap_or(crate::wb::JsValue::UNDEFINED)
        .is_undefined();

    if is_window_available {
        // as_string() handles the conversion from JsValue to Rust String
        raw::WINDOW_HOST.as_string()
    } else {
        None
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(handler: &js_sys::Function, timeout: i32) -> f64;
}

pub fn sleep(ms: usize) -> impl Future<Output = ()> {
    // We use a simple "Once" closure that wakes the future
    let mut was_called = false;
    futures_lite::future::poll_fn(move |cx| {
        if !was_called {
            let waker = cx.waker().clone();
            let closure = Closure::once(move || {
                waker.wake();
            });
            set_timeout(closure.as_ref().unchecked_ref(), ms as i32);
            closure.forget();
            was_called = true;
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
}
