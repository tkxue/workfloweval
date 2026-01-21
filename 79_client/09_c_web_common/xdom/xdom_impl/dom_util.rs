use super::*;
use wasm_bindgen::convert::FromWasmAbi;

pub struct Dom_Util {}

impl Dom_Util {
    pub fn wrap<E: 'static + FromWasmAbi, T: 'static + Fn(E)>(f: T) -> wasm_bindgen::closure::Closure<dyn Fn(E)> {
        wasm_bindgen::closure::Closure::wrap(Box::new(move |e: E| f(e)) as Box<dyn Fn(E)>)
    }

    pub fn wrap_0<T: 'static + Fn()>(f: T) -> wasm_bindgen::closure::Closure<dyn Fn()> {
        wasm_bindgen::closure::Closure::wrap(Box::new(move || f()) as Box<dyn Fn()>)
    }

    pub fn wrap_mut<E: 'static + FromWasmAbi, T: 'static + FnMut(E)>(f: T) -> wasm_bindgen::closure::Closure<dyn FnMut(E)> {
        let mut f = f;
        wasm_bindgen::closure::Closure::wrap(Box::new(move |e: E| f(e)) as Box<dyn FnMut(E)>)
    }

    /*
    pub fn get_window() -> web_sys::Window {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        window
    }


    pub async fn fetch_array_buffer(url: &str, request_mode: web_sys::RequestMode) -> Result<Xos_Jab, wb::JsValue> {
        if Self::is_web_page() {
            Self::window_fetch_array_buffer(url, request_mode).await
        } else {
            Self::worker_fetch_array_buffer(url, request_mode).await
        }
    }

    pub async fn fetch_array_buffer_retry(url: &str, request_mode: web_sys::RequestMode, millis: i32) -> Xos_Jab {
        if Self::is_web_page() {
            Self::window_fetch_array_buffer_retry(url, request_mode, millis).await
        } else {
            Self::worker_fetch_array_buffer_retry(url, request_mode, millis).await
        }
    }

    async fn window_fetch_array_buffer_retry(url: &str, request_mode: web_sys::RequestMode, _millis: i32) -> Xos_Jab {
        loop {
            let ab = Self::window_fetch_array_buffer(url, request_mode).await;
            match ab {
                Err(_err) => {
                    damn_it!("Error fetching: {:?}", url); /*
                                                           web_sys::console::log_2(&format!("DomUtil :: fetch :: {:?} failed, retrying", url).into(), &err);
                                                            */
                }
                Ok(ab) => {
                    return ab;
                }
            }
        }
    }

    async fn window_fetch_array_buffer(url: &str, request_mode: web_sys::RequestMode) -> Result<Xos_Jab, wb::JsValue> {
        damn_it!("")
        /*
        let opts = {
            let opts = web_sys::RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(request_mode);
            opts
        };
        let request = web_sys::Request::new_with_str_and_init(url, &opts)?;
        let window = web_sys::window().un(err!(""));
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<web_sys::Response>());
        let resp: web_sys::Response = resp_value.dyn_into()?;

        if resp.ok() {
            Ok(Xos_Jab::new_jsv(&wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?))
        } else {
            Err("DomUtil::fetch_array_buffer failed".into())
        }
        */
    }

    async fn worker_fetch_array_buffer_retry(url: &str, request_mode: web_sys::RequestMode, _millis: i32) -> Xos_Jab {
        loop {
            let ab = Dom_Util::fetch_array_buffer(url, request_mode).await;
            match ab {
                Err(_err) => {
                    damn_it!("Error fetching: {:?}", url);
                    /*
                    web_sys::console::log_2(&format!("DomUtil :: fetch :: {:?} failed, retrying", url).into(), &err);
                    Dom_Util::sleep_millis(millis).await;

                     */
                }
                Ok(ab) => {
                    return ab;
                }
            }
        }
    }

    async fn worker_fetch_array_buffer(url: &str, request_mode: web_sys::RequestMode) -> Result<Xos_Jab, wb::JsValue> {
        damn_it!("")
        /*
        let opts = {
            let opts = web_sys::RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(request_mode);
            opts
        };
        let request = web_sys::Request::new_with_str_and_init(url, &opts)?;
        let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global().dyn_into::<web_sys::DedicatedWorkerGlobalScope>().un(err!(""));
        let resp_value = wasm_bindgen_futures::JsFuture::from(g.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<web_sys::Response>());
        let resp: web_sys::Response = resp_value.dyn_into()?;

        if resp.ok() {
            Ok(Xos_Jab::new_jsv(&wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?))
        } else {
            Err("DomUtil::fetch_array_buffer failed".into())
        }
        */
    }
     */

    pub fn is_web_page() -> bool {
        let t: String = js_sys::global().to_string().into();
        if t == "[object Window]" {
            true
        } else {
            false
        }
    }

    pub fn is_worker_thread() -> bool {
        let t: String = js_sys::global().to_string().into();
        if t != "[object Window]" {
            true
        } else {
            false
        }
    }

    // pub async fn sleep_millis(_n: usize) {}
}
