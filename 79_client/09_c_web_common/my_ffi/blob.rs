use super::*;



#[wasm_bindgen]
extern "C" {
    // --- BlobPropertyBag (The options object) ---
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type BlobPropertyBag;

    #[wasm_bindgen(method, setter, js_name = type)]
    pub fn set_mime_type(this: &BlobPropertyBag, val: &str);

    // --- Blob ---
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Blob;

    #[wasm_bindgen(constructor, js_class = "Blob")]
    pub fn new_with_parts(
        parts: &js_sys::Array,
        options: &BlobPropertyBag
    ) -> Blob;




    // --- URL ---
    pub type URL;

    #[wasm_bindgen(static_method_of = URL, js_name = createObjectURL)]
    pub fn create_object_url(blob: &Blob) -> String;

    // --- Worker & Options ---
    pub type Worker;
    #[wasm_bindgen(constructor, js_class = "Worker")]
    pub fn new_with_options(url: &str, options: &js_sys::Object) -> Worker;


   #[wasm_bindgen(method, setter, js_name = onmessage)]
    pub fn set_onmessage(this: &Worker, callback: Option<&js_sys::Function>);

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Worker, message: &JsValue);

    #[wasm_bindgen(method, js_name = terminate)]
    pub fn terminate(this: &Worker);

}