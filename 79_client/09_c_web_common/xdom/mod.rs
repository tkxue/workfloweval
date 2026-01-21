use super::*;
use std::cell::Ref;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};

mod audio_context;
mod my_app_user_events;
mod to_from_js;
mod xdom_css_util;
mod xdom_gfx_event_loop;
mod xdom_iframe_urls;
mod xdom_impl;
mod xdom_spawn_every;

pub use audio_context::*;
pub use my_app_user_events::*;
pub use to_from_js::*;
pub use xdom_css_util::*;
pub use xdom_gfx_event_loop::*;
pub use xdom_iframe_urls::*;
pub use xdom_impl::*;
pub use xdom_spawn_every::*;

use wasm_bindgen::closure::Closure;

#[derive(Debug, Clone, Copy, PartialEq, JsData)]
pub struct Xdom_Window_Size {
    pub width: u32,
    pub height: u32,
}

impl Xdom_Window_Size {
    pub fn new() -> Xdom_Window_Size {
        let window = &my_ffi::window::WINDOW;
        let width = window.inner_width().max(1.0) as u32;
        let height = window.inner_height().max(1.0) as u32;
        Xdom_Window_Size { width, height }
    }
}

pub struct XdomA {}

#[derive(JsData, Clone)]
pub struct Xos_Fetch_File {
    pub headers: Arc<Vec<Result<(String, String), String>>>,
    pub data: Xos_Jab,
}

impl Xos_Fetch_File {
    pub fn get_last_modified(&self) -> Option<&str> {
        for x in self.headers.iter() {
            match x {
                Err(_) => {}
                Ok((k, v)) => {
                    if k == "last-modified" {
                        return Some(v);
                    }
                }
            }
        }
        None
    }
}

impl XdomA {
    /*
    pub fn post_request(url: &str, json_body: &[u8]) -> Result<web_sys::XmlHttpRequest, wb::JsValue> {
        damn_it!("")
        /*
        use web_sys::{Event, XmlHttpRequest};
        let xhr = XmlHttpRequest::new().map_err(|_| wb::JsValue::from_str("Failed to create XMLHttpRequest"))?;

        // Set up the request
        xhr.open_with_async("POST", url, true)?; // true for async
        xhr.set_request_header("Content-Type", "application/json")?;

        /*

        // Create a closure to handle the readystatechange event
        let onreadystatechange = Closure::wrap(Box::new({
            let xhr = xhr.clone();
            move |_: Event| {
                if xhr.ready_state() == XmlHttpRequest::DONE && xhr.status() == Ok(200) {
                    let response_text = xhr.response_text().unwrap_or_else(|_| Some("No response".to_string()));
                    web_sys::console::log_1(&format!("Success: {:?}", response_text).into());
                } else if xhr.ready_state() == XmlHttpRequest::DONE {
                    web_sys::console::log_1(&format!("Error: Status {:?}", xhr.status()).into());
                }
            }
        }) as Box<dyn FnMut(Event)>);

        xhr.set_onreadystatechange(Some(onreadystatechange.as_ref().unchecked_ref()));
        onreadystatechange.forget(); // Prevent the closure from being dropped prematurely
        */

        xhr.send_with_opt_u8_array(Some(json_body))?;
        Ok(xhr)
        */
    }
    */

    pub fn window_focus() {
        /*
        let window = web_sys::window().un(err!(""));
        let _ = window.focus();

         */
        damn_it!("")
    }

    /*
    pub fn get_main_canvas() -> web_sys::HtmlCanvasElement {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        let document: web_sys::Document = window.document().expect("can't get document");
        let main_canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id("main_canvas")
            .expect("can't get main_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("can't convert main_canvas into HTMLCanvasElement");
        main_canvas
    }
    */

    /*
    pub fn host() -> String {
        let window = web_sys::window().un(err!(""));
        let location = window.location();
        location.host().un(err!(""))
    }

    pub fn protocol() -> String {
        let window = web_sys::window().un(err!(""));
        let location = window.location();
        location.protocol().un(err!(""))
    }

     */

    pub async fn fetch_file(url: &str) -> Res<Xos_Fetch_File> {
        Self::fetch_file_inner(url)
            .await
            .attach_context(err!("Fetching URL: {}", url))
    }

    pub fn default_fetch_handler(x: &str, _err: Err_Stack) {
        wlog!("failed to fetch url: {:?}", x);
    }

    pub async fn fetch_file_loop(
        url: &str,
        delay_milli: u32,
        on_fail: Rc<impl Fn(&str, Err_Stack) -> ()>,
    ) -> Xos_Fetch_File {
        loop {
            let out = Self::fetch_file(url).await;
            match out {
                Ok(x) => {
                    return x;
                }
                Err(x) => {
                    (on_fail.as_ref())(url, x);
                    XdomA::sleep_millis(delay_milli as usize).await;
                }
            }
        }
    }

    pub fn set_field(obj: &wb::JsValue, name: &str, v: &wb::JsValue) {
        let _ = js_sys::Reflect::set(obj, &wb::JsValue::from(name), v);
    }

    pub fn get_field(obj: &wb::JsValue, name: &str) -> Option<wb::JsValue> {
        js_sys::Reflect::get(obj, &wb::JsValue::from(name)).ok()
    }

    pub fn set_global(name: &str, v: &wb::JsValue) {
        /*
        if XdomA::is_iframe() {
            let p = web_sys::window().un(err!(""));
            Self::set_field(p.as_ref(), name, v);
        } else {
            let p: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            Self::set_field(p.as_ref(), name, v);
        };

         */
        damn_it!("")
    }

    pub fn get_global(name: &str) -> Option<wb::JsValue> {
        /*
        if XdomA::is_iframe() {
            let p = web_sys::window().un(err!(""));
            Self::get_field(p.as_ref(), name)
        } else {
            let p: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            Self::get_field(p.as_ref(), name)
        }

         */
        damn_it!("")
    }

    async fn fetch_file_inner(url: &str) -> Res<Xos_Fetch_File> {
        damn_it!("")
        /*
        let opts = RequestInit::new();
        opts.set_method("GET");

        let request = Request::new_with_str_and_init(&url, &opts).map_err(err_jsv!())?;

        let resp_value = if XdomA::is_iframe() {
            let p = web_sys::window().un(err!(""));
            JsFuture::from(p.fetch_with_request(&request))
                .await
                .map_err(err_jsv!())?
        } else {
            let p: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            JsFuture::from(p.fetch_with_request(&request))
                .await
                .map_err(err_jsv!())?
        };

        let resp: Response = resp_value.dyn_into().map_err(err_jsv!())?; // .unwrap();

        if resp.ok() {
            let array_buffer = match JsFuture::from(resp.array_buffer().map_err(err_jsv!())?).await {
                Ok(x) => x,
                Err(e) => {
                    damn_it!("failed to parse JSON on url: {:?}, error: {:?}", url, e);
                }
            };
            let headers: Headers = resp.headers();

            let mut out = vec![];
            for x in headers.entries() {
                match x {
                    Ok(x) => match Xos_Js_Array::new_jsv(&x) {
                        Ok(x) => {
                            let x = x.to_vec();
                            if x.len() != 2 {
                                out.push(Err(format!("{:?}", x)));
                            } else {
                                match (x[0].as_string(), x[1].as_string()) {
                                    (Some(x), Some(y)) => {
                                        out.push(Ok((x, y)));
                                    }
                                    _ => {
                                        out.push(Ok((format!("{:?}", x[0]), format!("{:?}", x[1]))));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            out.push(Err(format!("{:?}", e)));
                        }
                    },
                    Err(e) => out.push(Err(format!("{:?}", e))),
                }
            }

            Ok(Xos_Fetch_File {
                headers: Arc::new(out),
                data: Xos_Jab::new_jsv(&array_buffer),
            })
        } else {
            let array_buffer = JsFuture::from(resp.array_buffer().map_err(err_jsv!())?)
                .await
                .expect("failed to parse JSON");
            let jab = Xos_Jab::new_jsv(&array_buffer);
            let s = String::from_utf8_lossy(&jab.to_vec()).to_string();
            Err(err!("{}", s))?
        }
        */
    }

    pub fn time_origin() -> f64 {
        /*
        use wasm_bindgen::JsCast;
        let performance = if XdomA::is_iframe() {
            let p = web_sys::window().un(err!("")); // .parent().un(err!("")).un(err!(""));
            p.performance().unwrap()
        } else {
            let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            g.performance().unwrap()
        };
        performance.time_origin()

         */
        damn_it!("")
    }

    pub fn performance_now_milli() -> f64 {
        damn_it!("")
        /*
        use wasm_bindgen::JsCast;
        let performance = if XdomA::is_iframe() {
            let p = web_sys::window().un(err!("")); // .parent().un(err!("")).un(err!(""));
            p.performance().unwrap()
        } else {
            let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            g.performance().unwrap()
        };
        performance.now()
        */
    }

    pub fn now_f64_milli() -> f64 {
        damn_it!("")
        /*
        use wasm_bindgen::JsCast;
        let performance = if XdomA::is_iframe() {
            let p = web_sys::window().un(err!("")); // .parent().un(err!("")).un(err!(""));
            p.performance().unwrap()
        } else {
            let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            g.performance().unwrap()
        };
        let time_origin = performance.time_origin();
        let now = performance.now();
        (now + time_origin)
        */
    }

    pub fn goto_url(s: &str) {
        damn_it!("")
        // let _ = web_sys::window().expect("should have a Window").location().set_href(&s);
    }

    pub fn make_iframe(s: &str) -> String {
        let opts = vec![
            ("style", "pointer-events: all"),
            ("allow", "cross-origin-isolated"),
            ("credentialless", ""),
            ("width", "100%"),
            ("height", "100%"),
            ("scrolling", "no"),
            ("src", s),
        ];

        let mut s = vec![];
        for (k, v) in opts.iter() {
            write!(s, "{}='{}' ", k, v).un(err!(""));
        }

        String::from_utf8_lossy(&s).to_string()
    }

    /*
    pub fn body_set_inner_html(wasm_version: Arc<String>) {
        let config = XdomA_IFrame_Urls::new(wasm_version);
        let s = format!(
            "<style>{}</style> \
            ",
            XdomA_Css_Util::full_css(),
            Self::make_iframe(&config.sound),
            Self::make_iframe(&config.gfx),
        );

        let body = my_ffi::window::DOCUMENT
            .body()
            .ok_or_else(|| wb::JsValue::from_str("document.body not found"))
            .unwrap();

        body.set_inner_html(&s);
    }
    */

    pub fn spawn_local(x: Pin<Box<dyn Future<Output = ()>>>) {
        wasm_bindgen_futures::spawn_local(x);
    }

    pub fn request_fullscreen() {
        damn_it!("")
        /*
        use web_sys::{window, Document, Element};

        let window = window().expect("should have a window in the browser");
        let document = window.document().expect("should have a document");
        let element: Element = document.document_element().expect("should have a document element");

        let _ = element.request_fullscreen();
        */
    }

    pub fn is_iframe() -> bool {
        let t: String = js_sys::global().to_string().into();
        if t == "[object Window]" { true } else { false }
    }

    pub fn parent_post_message_with_transfer(msg: &wb::JsValue, transfers: &wb::JsValue) -> Result<(), wb::JsValue> {
        /*
        use wasm_bindgen::JsCast;
        if my_ffi::is_web_worker() {

            let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                .un(err!(""));
            g.post_message_with_transfer(msg, transfers)
        } else {

            let p = web_sys::window().un(err!("")).parent().un(err!("")).un(err!(""));
            p.post_message_with_transfer(msg, "*", transfers)
        }
        */

        use wasm_bindgen::JsCast;

        // Convert the generic JsValue transfers into the required js_sys::Array
        let transfers_arr: &js_sys::Array = transfers
            .dyn_ref::<js_sys::Array>()
            .ok_or_else(|| wb::JsValue::from_str("transfers must be an Array"))?;

        if my_ffi::is_web_worker() {
            // Get global scope and cast to our Worker FFI type
            let g: my_ffi::window::DedicatedWorkerGlobalScope =
                js_sys::global().unchecked_into::<my_ffi::window::DedicatedWorkerGlobalScope>();

            g.worker_post_message_with_transfer(msg, transfers_arr);
            Ok(())
        } else {
            // Get window.parent
            let win = &my_ffi::window::WINDOW;
            let p = win
                .parent()
                .ok_or_else(|| wb::JsValue::from_str("Could not access window.parent"))?;

            // In JS: p.postMessage(msg, "*", transfers)
            p.post_message_with_transfer(msg, "*", transfers_arr);
            Ok(())
        }
    }

    /*
    pub fn sleep_millis(n: usize) -> Pin<Box<dyn Future<Output = ()>>> {

        let (sender, receiver) = futures::channel::oneshot::channel::<()>();

        // We use Closure::once because the timer fires exactly once.
        // This allows wasm-bindgen to clean up the memory automatically after execution.
        let closure = Closure::once(move || {
            let _ = sender.send(());
        });

        // Pass the closure's function reference to JavaScript
        set_timeout(closure.as_ref().unchecked_ref(), n);

        // We must 'forget' the closure so it isn't dropped before the timer fires.
        // Because it is a Closure::once, it will still be cleaned up after it is called.
        closure.forget();

        // Wait for the JS callback to signal the channel
        let _ = receiver.await;


        /*
    let t = async move {
        if Dom_Util::is_web_page() {
            let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
                js_sys::global()
                    .dyn_into::<web_sys::Window>()
                    .un(err!(""))
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, n as i32)
                    .un(err!(""));
            };
            let p = js_sys::Promise::new(&mut cb);
            wasm_bindgen_futures::JsFuture::from(p).await.un(err!(""));
        } else {
            let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
                let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global()
                    .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
                    .un(err!(""));
                g.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, n as i32)
                    .un(err!(""));
            };
            let p = js_sys::Promise::new(&mut cb);
            wasm_bindgen_futures::JsFuture::from(p).await.un(err!(""));
        }
    };
    Box::pin(t)

         */
    }
    */

    pub fn sleep_millis(ms: usize) -> impl Future<Output = ()> {
        my_ffi::sleep(ms)
    }

    pub fn request_pointer_lock() {
        /*
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        let document: web_sys::Document = window.document().expect("can't get document");
        let elem: web_sys::Element = document.get_element_by_id("div1").expect("can't get main_canvas");
        elem.request_pointer_lock();

         */
        damn_it!("")
    }

    pub fn request_animation_frame(cb: &Closure<dyn Fn()>) {
        /*
        use wasm_bindgen::JsCast;
        let w = web_sys::window().un(err!("can't get window"));
        let _ = w.request_animation_frame(cb.as_ref().unchecked_ref());

         */
        damn_it!("")
    }

    pub fn raf_leak(cb: Rc<dyn Fn()>) {
        let t: Arc<RefCell<Option<Closure<dyn Fn()>>>> = Arc::new(RefCell::new(None));

        let t2 = t.clone();
        let c: Closure<dyn Fn()> = wasm_bindgen::prelude::Closure::new(move || {
            //  let start = instant::Instant::now();
            let _s = instant::Instant::now();
            cb();
            let _e = instant::Instant::now();
            // wlog!("diff: {:?}", (e - s).as_millis());
            // let end = instant::Instant::now();
            // wlog!("raf millis: {:?}", (start - end).as_millis());
            let t2 = t2.clone();

            let f = t2.deref().borrow();
            match f.as_ref() {
                None => {
                    damn_it!("impossible: raf_leak RefCell None");
                }
                Some(x) => {
                    Self::request_animation_frame(&x);
                }
            }
        });

        t.borrow_mut().replace(c);

        Self::request_animation_frame(t.borrow_mut().as_ref().unwrap());
    }

    pub fn speak(s: &str) {
        /*
        let s = web_sys::SpeechSynthesisUtterance::new_with_text(s).unwrap();
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        window.speech_synthesis().unwrap().cancel();
        window.speech_synthesis().unwrap().speak(&s);

         */
        damn_it!("")
    }

    pub fn speech_synthesis__is_speaking() -> bool {
        damn_it!("")
        /*
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        window.speech_synthesis().unwrap().speaking()

         */
    }

    pub fn speech_synthesis__is_pending() -> bool {
        damn_it!("")
        /*
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        window.speech_synthesis().unwrap().pending()

         */
    }
}

#[derive(Clone, Debug)]
pub struct XdomA_UI_Resize_Event {
    pub inner: wb::JsValue,
}

#[derive(Clone)]
pub struct XdomA_Pointer_Event {
    pub inner: wb::JsValue,
}

pub struct Jsv {
    inner: wb::JsValue,
}

impl Jsv {
    pub fn new(v: wb::JsValue) -> Jsv {
        Jsv { inner: v }
    }

    pub fn get_field(&self, s: &str) -> Result<Jsv, Js_Parse_Error> {
        match js_sys::Reflect::get(&self.inner, &s.into()) {
            Ok(t) => Ok(Jsv::new(t)),
            Err(_) => Err(Js_Parse_Error {
                error: format!("Jsv::get_field fail: {}", s),
                v: self.inner.clone(),
            }),
        }
    }

    pub fn to_typed<T: T_JsData_>(&self) -> Result<T, Js_Parse_Error> {
        let t = self.inner.clone();
        let x = Xos_Msg::from_js(t.clone()).ok().ok_or(Js_Parse_Error {
            error: "err: #.data -> Xos_Raw_Msg".to_string(),
            v: self.inner.clone(),
        })?;
        let t = x.to_typed::<T>().map_err(|_| Js_Parse_Error {
            error: "err: #.data -> Xos_Raw_Msg > T".to_string(),
            v: self.inner.clone(),
        })?;
        Ok(t)
    }
}
