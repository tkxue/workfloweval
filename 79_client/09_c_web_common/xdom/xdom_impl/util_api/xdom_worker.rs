#[allow(unused_imports)]
use super::*;
use crate::wb::JsValue;

/*
pub struct FIm_XdomA_Ww_T {}

/*
impl Fwd_XdomA_Ww_T for FIm_XdomA_Ww_T {
    fn new(&self, x: &wb::JsValue, name: String) -> XdomA_Ww {
        XdomA_Ww {
            inner: Arc::new(Xdom_Ww_Impl::new_str_blob_js(x, name)),
        }
    }
}

my_init! { XdomA_Ww::__set_fn__t(Arc::new(|| Arc::new(FIm_XdomA_Ww_T {}))); }

 */

// impl XdomA_Ww_T for Xdom_Ww_Impl

impl XdomA_Ww {
    pub fn ping(&self) -> Result<(), wb::JsValue> {
        self.inner.post_message(&"".into());
        Ok(())
    }

    pub fn set_on_message(&self, cb: Rc<dyn Fn(XdomA_Message_Event)>) {
        let cb = Rc::new(Dom_Util::wrap(move |e: my_ffi::message_event::MessageEvent| {
            (cb.as_ref())(XdomA_Message_Event { inner: e.into() })
        }));
        *self.cb.borrow_mut() = Some(cb.clone());
        self.inner.set_onmessage(Some(cb.as_ref().as_ref().unchecked_ref()));

    }
}

pub struct XdomA_Ww {
    ready: Cell<bool>,
    msgs: RefCell<Vec<Xos_L0_Msg>>,
    cb: RefCell<Option<Rc<Closure<dyn Fn(my_ffi::message_event::MessageEvent)>>>>,
    pub inner: my_ffi::blob::Worker,
    name: String,
}

impl Drop for XdomA_Ww {
    fn drop(&mut self) {
        /*
        wlog!("Xdom_Ww_Impl dropping {}", self.name);
        self.inner.set_onmessage(None);
        self.inner.terminate();

         */
        damn_it!("")
    }
}

impl XdomA_Ww {
    pub fn ready(&self) -> bool {
        self.ready.get()
    }

    /*

       XdomA_Ww {
           inner: Arc::new(Xdom_Ww_Impl::new_str_blob_js(x, name)),
       }

    */

    pub fn new(inner: my_ffi::blob::Worker, name: String) -> XdomA_Ww {
        XdomA_Ww {
            ready: Cell::new(false),
            msgs: RefCell::new(vec![]),
            cb: RefCell::new(None),
            inner,
            name,
        }

    }

    pub fn post_message(&self, msg: Xos_L0_Msg) {
        if self.ready.get() {
            self.send_immediate(msg)
        } else {
            // console_log!("Delaying msg: {:?}", msg);
            self.msgs.borrow_mut().push(msg)
        }
    }

    pub fn new_str_blob_str(js_code: &str, name: String) -> XdomA_Ww {
        let code = wb::JsValue::from_str(js_code);
        Self::new_str_blob_js(&code, name)
    }

    pub fn new_str_blob_js(code: &wb::JsValue, name: String) -> XdomA_Ww {
        let arr = js_sys::Array::new_with_length(1);
        arr.set(0, code.clone());

        /*
        let blob_property_bag = web_sys::BlobPropertyBag::new();
        blob_property_bag.set_type("application/javascript");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&arr, &blob_property_bag).un(err!(""));

         */


        let blob_opts = js_sys::Object::new();
        js_sys::Reflect::set(&blob_opts, &"type".into(), &"application/javascript".into()).ok();
        let blob_opts: my_ffi::blob::BlobPropertyBag = blob_opts.unchecked_into::<my_ffi::blob::BlobPropertyBag>();

        let blob = my_ffi::blob::Blob::new_with_parts(&arr, &blob_opts);


        /*
        let worker_options = my_ffi::blob::WorkerOptions::new();
        worker_options.set_type(my_ffi::blob::WorkerType::Module);

         */


        let worker_opts = js_sys::Object::new();
        js_sys::Reflect::set(&worker_opts, &"type".into(), &"module".into()).ok();



        let url = my_ffi::blob::URL::create_object_url(&blob);

        let worker = my_ffi::blob::Worker::new_with_options(&url, &worker_opts);

        /*
        let url = web_sys::Url::create_object_url_with_blob(&blob).un(err!(""));
        let worker = web_sys::Worker::new_with_options(&url, &worker_options).un(err!(""));

         */

        Self::new(worker, name)
    }

    /*
    pub fn new_str_blob(blob: web_sys::Blob, name: String) -> XdomA_Ww {
        let worker_options = web_sys::WorkerOptions::new();
        worker_options.set_type(web_sys::WorkerType::Module);

        let url = web_sys::Url::create_object_url_with_blob(&blob).un(err!(""));

        let worker = web_sys::Worker::new_with_options(&url, &worker_options).un(err!(""));

        Self::new(worker, name)
    }
    */

    pub fn new_str_blob_jsvalue(code: wb::JsValue, name: String) -> XdomA_Ww {
        damn_it!("")
        /*
        let arr = js_sys::Array::new_with_length(1);
        arr.set(0, code);
        let blob_property_bag = web_sys::BlobPropertyBag::new();
        blob_property_bag.set_type("application/javascript");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&arr, &blob_property_bag).un(err!(""));

        let worker_options = web_sys::WorkerOptions::new();
        worker_options.set_type(web_sys::WorkerType::Module);

        let url = web_sys::Url::create_object_url_with_blob(&blob).un(err!(""));

        let worker = web_sys::Worker::new_with_options(&url, &worker_options).un(err!(""));

        Self::new(worker, name)
        */
    }
}

impl XdomA_Ww {
    pub fn send_immediate(&self, msg: Xos_L0_Msg) {
        /*
        self.inner
            .post_message_with_transfer(&msg.full_msg(), msg.transfers())
            .un(err!(""));

         */
        damn_it!("")
    }

    pub fn set_ready_true(&self) {
        self.ready.set(true);
        let msgs = self.msgs.replace(vec![]);
        for msg in msgs {
            self.send_immediate(msg);
        }
    }
}


*/
