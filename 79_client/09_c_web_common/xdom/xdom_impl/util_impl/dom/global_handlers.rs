use super::*;

/*
pub struct Global_Handlers {
    on_message: Option<Rc<Closure<dyn FnMut(web_sys::MessageEvent)>>>,
    on_resize: Option<Rc<Closure<dyn FnMut(web_sys::UiEvent)>>>,
    on_pointer_lock_change: Option<Rc<Closure<dyn FnMut(web_sys::Event)>>>,
}

 */

/*
use super::*;


use wasm_bindgen::closure::Closure;

use wasm_bindgen::JsCast;

impl Global_Handlers {
    pub fn new() -> Global_Handlers {
        Global_Handlers {
            on_message: None,
            on_resize: None,
            on_pointer_lock_change: None,
        }
    }

    pub fn set_on_message(&mut self, cb: Option<Rc<Closure<dyn FnMut(web_sys::MessageEvent)>>>) {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        self.on_message = cb;
        match &self.on_message {
            None => {
                window.set_onmessage(None);
            }
            Some(f) => {
                window.set_onmessage(Some(f.as_ref().as_ref().unchecked_ref()));
            }
        }
    }

    pub fn set_on_resize(&mut self, callback: Option<Rc<Closure<dyn FnMut(web_sys::UiEvent)>>>) {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        self.on_resize = callback;
        match &self.on_resize {
            None => window.set_onresize(None),
            Some(v) => window.set_onresize(Some(v.as_ref().as_ref().unchecked_ref())),
        }
    }

    pub fn set_on_pointer_lock_change(&mut self, callback: Option<Rc<Closure<dyn FnMut(web_sys::Event)>>>) {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        let doc = window.document().un(mk_err!(""));
        self.on_pointer_lock_change = callback;
        match &self.on_pointer_lock_change {
            None => doc.set_onpointerlockchange(None),
            Some(v) => doc.set_onpointerlockchange(Some(v.as_ref().as_ref().unchecked_ref())),
        }
    }
}

impl Drop for Global_Handlers {
    fn drop(&mut self) {
        self.set_on_message(None);
        self.set_on_resize(None);
    }
}

/*
let msg_code = msg_code.clone();


self.on_message = cb;
match &self.on_message {
    None => window.set_onmessage(None),
    Some(v) => window.set_onmessage(Some(v.as_ref().as_ref().unchecked_ref())),
}

 */
/*
pub fn set_on_message(&mut self, cb: Option<Rc<Closure<dyn FnMut(web_sys::MessageEvent)>>>) {
    let window: web_sys::Window = web_sys::window().expect("can't get window");
    self.on_message = cb;
    match &self.on_message {
        None => window.set_onmessage(None),
        Some(v) => window.set_onmessage(Some(v.as_ref().as_ref().unchecked_ref())),
    }
}
 */

/*
Box::new(move |event: web_sys::MessageEvent| {
let incoming_msg: Result<Msg_0, String> = Msg_0::new(&event);
match incoming_msg {
    Err(err) => {
        web_sys::console::log_1(&format!("h_index: got error: {}", err).into());
    }
    Ok(Msg_0::Fetch_Code { port }) => {
        port.post_message(&msg_code);
    }
    Ok(Msg_0::Rust_Msg(x)) => match x {
        Msg_0_Rust::Send_Message_Ports { .. } => {
            let msg = Msg_0::Rust_Msg(x);
            let p = web_sys::window().un(mk_err!("")).parent().un(mk_err!("")).un(mk_err!(""));
            p.post_message_with_transfer(&msg.to_js(), "*", &msg.transfers())
                .un(mk_err!(""));
        }
    },
}
}) as Box<dyn FnMut(web_sys::MessageEvent)>

 */


 */
