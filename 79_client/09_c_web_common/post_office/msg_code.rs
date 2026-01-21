use super::*;

#[derive(Clone)]
pub struct Msg_Code {
    pub raw: wb::JsValue,
    pub window_location_host: Arc<String>,
    pub window_location_protocol: Arc<String>,
    pub wasm_version: Arc<String>,
}

impl Msg_Code {
    pub fn new(v: wb::JsValue) -> Msg_Code {
        let window_location_host = js_sys::Reflect::get(&v, &"window_location_host".into()).expect("can't get worker_template_js");
        let window_location_protocol = js_sys::Reflect::get(&v, &"window_location_protocol".into()).expect("can't get worker_template_js");
        let wasm_version = js_sys::Reflect::get(&v, &"wasm_version".into()).expect("can't get worker_template_js");

        Msg_Code {
            raw: v,
            window_location_host: Arc::new(window_location_host.as_string().expect("host not string")),
            window_location_protocol: Arc::new(window_location_protocol.as_string().expect("host not string")),
            wasm_version: Arc::new(wasm_version.as_string().expect("wasm_version not found")),
        }
    }

    pub fn is_dev(&self) -> bool {
        let r = self.window_location_host.starts_with("localhost");
        r
    }
}
