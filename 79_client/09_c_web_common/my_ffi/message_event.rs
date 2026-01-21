use super::*;


use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object, js_name = MessageEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MessageEvent;

    #[wasm_bindgen(method, getter, js_name = data)]
    pub fn data(this: &MessageEvent) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = lastEventId)]
    pub fn last_event_id(this: &MessageEvent) -> String;

    #[wasm_bindgen(method, getter, js_name = origin)]
    pub fn origin(this: &MessageEvent) -> String;

    #[wasm_bindgen(method, getter, js_name = ports)]
    pub fn ports(this: &MessageEvent) -> js_sys::Array;
}
