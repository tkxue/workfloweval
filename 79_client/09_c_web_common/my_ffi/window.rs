use super::*;

#[wasm_bindgen]
extern "C" {
    pub type Window;




    #[wasm_bindgen(js_name = window)]
    pub static WINDOW: Window;


    #[wasm_bindgen(method, getter, js_name = innerWidth)]
    pub fn inner_width(this: &Window) -> f64;

    // Returns the content height of the window including scrollbars
    #[wasm_bindgen(method, getter, js_name = innerHeight)]
    pub fn inner_height(this: &Window) -> f64;


    // Access the .parent property
    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &Window) -> Option<Window>;

    // The postMessage method with transferables
    // Signature: postMessage(message, targetOrigin, transfer)
    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message_with_transfer(
        this: &Window,
        message: &JsValue,
        target_origin: &str,
        transfer: &js_sys::Array,
    );


    #[wasm_bindgen(method, setter, js_name = onmessage)]
    pub fn set_onmessage(this: &Window, callback: Option<&js_sys::Function>);



    // --- Worker Environment ---
    pub type DedicatedWorkerGlobalScope;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn worker_post_message_with_transfer(
        this: &DedicatedWorkerGlobalScope,
        message: &JsValue,
        transfer: &js_sys::Array,
    );




    pub type Document;

    #[wasm_bindgen(js_name = document)]
    pub static DOCUMENT: Document;

    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Document) -> Option<HtmlElement>;

    // --- HTMLElement ---
    pub type HtmlElement;

    #[wasm_bindgen(method, setter, js_name = innerHTML)]
    pub fn set_inner_html(this: &HtmlElement, html: &str);



    #[wasm_bindgen(method, getter)]
    pub fn location(this: &Window) -> Location;

    pub type Location;

    // Location.href returns the full URL as a string
    #[wasm_bindgen(method, getter)]
    pub fn href(this: &Location) -> String;


}