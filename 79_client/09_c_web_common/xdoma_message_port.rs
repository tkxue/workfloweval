use super::*;

/*
mod message_event;

/*
#[wasm_bindgen]
extern "C" {
    // Define the subset of MessageEvent you actually need
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MessageEvent;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &MessageEvent) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn origin(this: &MessageEvent) -> String;
}

#[wasm_bindgen]
pub fn handle_message(event: MessageEvent) {
    let data = event.data();

    // Check if it's a string, object, etc., using js_sys
    if let Some(s) = data.as_string() {
        // Handle string data
    }
}

 */

use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct XdomA_Message_Port {
    pub inner: my_ffi::message_channel::MessagePort,
}

impl T_JsData_ for XdomA_Message_Port {
    fn write_to_js(&self, _writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let t = transfers.pop_front().unwrap();
        Ok(XdomA_Message_Port::new(t))
    }
}

impl XdomA_Message_Port {
    pub fn new(v: wb::JsValue) -> XdomA_Message_Port {
        let t = my_ffi::message_channel::MessagePort::from(v);
        if t.is_null() || t.is_undefined() {
            damn_it!("Failed conversion to Xdom_Message_Port")
        }
        XdomA_Message_Port {
            inner: t, // Rc::new(Xdom_Message_Port_Impl { inner: t }),
        }
    }

    pub fn new_message_channel() -> (XdomA_Message_Port, XdomA_Message_Port) {
        let message_channel = my_ffi::message_channel::MessageChannel::new();
        let p1 = message_channel.port1();
        let p2 = message_channel.port2();

        (XdomA_Message_Port { inner: p1 }, XdomA_Message_Port { inner: p2 })
    }

    pub fn into_cb(self, cb: Arc<dyn Fn(XdomA_Message_Event)>) -> XdomA_Message_Port_Cb {
        use wasm_bindgen::JsCast;

        let cb = wasm_bindgen::closure::Closure::wrap(
            Box::new(move |e: my_ffi::message_event::MessageEvent| cb(XdomA_Message_Event { inner: e.into() }))
                as Box<dyn FnMut(my_ffi::message_event::MessageEvent)>,
        );

        let v = self.inner;

        v.set_onmessage(cb.as_ref().unchecked_ref());

        XdomA_Message_Port_Cb { v: v.clone(), _cb: cb }
    }
}

#[derive(Clone)]
pub struct XdomA_Message_Event {
    pub inner: wb::JsValue,
}

pub struct XdomA_Message_Port_Cb {
    pub v: my_ffi::message_channel::MessagePort,
    pub _cb: wasm_bindgen::prelude::Closure<dyn FnMut(my_ffi::message_event::MessageEvent)>,
}

impl Drop for XdomA_Message_Port_Cb {
    fn drop(&mut self) {
        damn_it!("")
        // self.v.set_onmessage(None);
    }
}


 */
