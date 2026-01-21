#[allow(unused_imports)]
use super::*;

pub struct Xos_Util {}

impl Xos_Util {
    pub async fn sleep(millis: i32) {
        let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
            web_sys::window()
                .un(err!(""))
                .set_timeout_with_callback_and0_timeout_and_arguments_0(&resolve, millis)
                .un(err!(""));
        };
        let p = js_sys::Promise::new(&mut cb);
        wasm_bindgen_futures::JsFuture::from(p).await.un(err!(""));
    }

    pub async fn fetch_array_buffer_retry(url: &str, request_mode: web_sys::RequestMode, millis: i32) -> Xos_Js_ArrayBuffer {
        loop {
            let ab = Xos_Util::fetch_array_buffer(url, request_mode).await;
            match ab {
                Err(err) => {
                    web_sys::console::log_2(&format!("DomUtil :: fetch :: {:?} failed, retrying", url).into(), &err);
                    Xos_Util::sleep(millis).await;
                }
                Ok(ab) => {
                    return ab;
                }
            }
        }
    }

    pub async fn fetch_array_buffer(url: &str, request_mode: web_sys::RequestMode) -> Result<Xos_Js_ArrayBuffer, wb::JsValue> {
        let opts = {
            let mut opts = web_sys::RequestInit::new();
            opts.method("GET");
            opts.mode(request_mode);
            opts
        };
        let request = web_sys::Request::new_with_str_and_init(url, &opts)?;
        let window = web_sys::window().un(err!(""));
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;

        / `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<web_sys::Response>());
        let resp: web_sys::Response = resp_value.dyn_into()?;

        / Convert this other `Promise` into a rust `Future`.
        Ok(Xos_Js_ArrayBuffer::new_jsv(&wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?))
    }

    pub fn window_set_on_message(on_message_cb: &wasm_bindgen::closure::Closure<dyn FnMut(web_sys::MessageEvent)>) {
        let w = Self::get_window();
        w.set_onmessage(Some(on_message_cb.as_ref().unchecked_ref()));
    }

    pub fn dedicated_worker_global_scope_set_on_message(on_message_cb: &wasm_bindgen::closure::Closure<dyn FnMut(web_sys::MessageEvent)>) {
        let g: web_sys::DedicatedWorkerGlobalScope = js_sys::global().dyn_into::<web_sys::DedicatedWorkerGlobalScope>().un(err!(""));
        g.set_onmessage(Some(on_message_cb.as_ref().unchecked_ref()));
    }

    pub fn dedicated_worker_global_scope() -> web_sys::DedicatedWorkerGlobalScope {
        js_sys::global().dyn_into::<web_sys::DedicatedWorkerGlobalScope>().un(err!(""))
    }

    pub fn set_timeout(f: Rc<dyn Fn() -> Option<usize>>) {
        let timer_cb: Rc<RefCell<Option<wasm_bindgen::closure::Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));

        fn do_stuff(timer_cb: &Rc<RefCell<Option<wasm_bindgen::closure::Closure<dyn Fn()>>>>, work: &Rc<dyn Fn() -> Option<usize>>) {
            let t: Option<usize> = (work.as_ref())();

            match t {
                None => {
                    *timer_cb.as_ref().borrow_mut() = None;
                }
                Some(delay) => {
                    if let Some(js_cb) = timer_cb.as_ref().borrow().as_ref() {
                        Xos_Util::get_window()
                            .set_timeout_with_callback_and0_timeout_and_arguments_0(js_cb.as_ref().unchecked_ref(), delay as i32)
                            .un(err!(""));
                    }
                }
            }
        }

        let a: wasm_bindgen::closure::Closure<dyn Fn()> = {
            let t2 = timer_cb.clone();
            let f2 = f.clone();
            wasm_bindgen::closure::Closure::wrap(Box::new(move || do_stuff(&t2, &f2)) as Box<dyn Fn()>)
        };

        *timer_cb.as_ref().borrow_mut() = Some(a);

        do_stuff(&timer_cb, &f);
    }

    pub fn start_worker(url: &str) -> web_sys::Worker {
        let mut worker_options = web_sys::WorkerOptions::new();
        worker_options.type_(web_sys::WorkerType::Module);
        web_sys::Worker::new_with_options(url, &worker_options).un(err!(""))
    }

    pub fn get_window() -> web_sys::Window {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        window
    }


    pub fn get_window_location_href() -> String {
        let window: web_sys::Window = web_sys::window().expect("can't get window");
        let location: web_sys::Location = window.location();
        let href = location.href().expect("can't get href of location");
        href
    }

    pub fn base_name(s: &str) -> Option<&str> {
        let p = s.rfind('/')?;
        Some(&s[p..])
    }

    pub fn request_animation_frame(f: &wasm_bindgen::closure::Closure<dyn FnMut()>) {
        Self::get_window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }
}
