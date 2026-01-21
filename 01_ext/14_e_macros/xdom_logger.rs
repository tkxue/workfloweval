use super::*;

use wasm_bindgen::prelude::wasm_bindgen;

pub static _G_Xdom_Logger: OnceLock<Xdom_Logger> = OnceLock::new();

mod raw {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        // This binds to the global console.log function
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);

        #[wasm_bindgen(js_name = WorkerGlobalScope)]
        type WorkerGlobalScope;

        #[wasm_bindgen(js_name = globalThis)]
        pub static GLOBAL: JsValue;

        #[wasm_bindgen(js_name = Window)]
        type Window;

        // 3. Bind to the host property via the window.location namespace
        #[wasm_bindgen(js_namespace = ["window", "location"], js_name = host)]
        pub static WINDOW_HOST: JsValue;

    }
}

fn my_log(s: &str) {
    raw::log(s)
}

pub struct Xdom_Logger {
    pub name: String,
    pub log_s: Arc<dyn Fn(&Err_Stack) + Send + Sync>,
    pub log_1: Arc<dyn Fn(&wb::JsValue) + Send + Sync>,
    pub log_2: Arc<dyn Fn(&wb::JsValue, &wb::JsValue) + Send + Sync>,
    pub log_3: Arc<dyn Fn(&wb::JsValue, &wb::JsValue, &wb::JsValue) + Send + Sync>,
    pub log_4: Arc<dyn Fn(&wb::JsValue, &wb::JsValue, &wb::JsValue, &wb::JsValue) + Send + Sync>,
}

#[wasm_bindgen]
pub struct Xdom_Logger_Util {}

#[wasm_bindgen]
impl Xdom_Logger_Util {
pub fn __set_loggers__wasm(name: String) {
    Xdom_Logger::__set_loggers__wasm(name);

}

}


impl Xdom_Logger {
    pub fn __set_loggers__server(name: String) {
        let _ = _G_Xdom_Logger.set(Xdom_Logger {
            name: name,
            log_s: Arc::new(|x0| {
                println!("{} : {}", x0.orig.file, x0.orig.line);
                println!("{}", x0.orig.msg);
                for x in x0.context.iter() {
                    println!("{} : {}", x.file, x.line);
                    println!("{}", x.msg);
                }
            }),
            log_1: Arc::new(|x0| {
                println!("{:?}", x0);
            }),
            log_2: Arc::new(|x0, x1| {
                println!("{:?} {:?}", x0, x1);
            }),
            log_3: Arc::new(|x0, x1, x2| {
                println!("{:?} {:?} {:?}", x0, x1, x2);
            }),
            log_4: Arc::new(|x0, x1, x2, x3| {
                println!("{:?} {:?} {:?} {:?}", x0, x1, x2, x3);
            }),
        });
    }

    pub fn __set_loggers__wasm(name: String) {
        let _ = _G_Xdom_Logger.set(Xdom_Logger {
            name: name,
            log_s: Arc::new(|x0| {
                my_log(&Log_Err_Util::url_str(x0.orig.as_ref()));
                my_log(&format!("{}", x0.orig.msg));
                for x in x0.context.iter() {
                    my_log(&Log_Err_Util::url_str(x));
                    my_log(&format!("{} : {}", x.file, x.line));
                    my_log(&format!("{}", x.msg));
                }
            }),
            log_1: Arc::new(|x0| {
                my_log(&format!("{:?}", x0));
            }),
            log_2: Arc::new(|x0, x1| {
                my_log(&format!("{:?} {:?}", x0, x1));
            }),
            log_3: Arc::new(|x0, x1, x2| {
                my_log(&format!("{:?} {:?} {:?}", x0, x1, x2));
            }),
            log_4: Arc::new(|x0, x1, x2, x3| {
                my_log(&format!("{:?} {:?} {:?} {:?}", x0, x1, x2, x3));
            }),
        });
    }

    pub fn log_s(err_msg: &Err_Stack /* , msg: &str */) {
        let log_s = &_G_Xdom_Logger.get().unwrap().log_s;
        log_s(err_msg);
    }

    pub fn log_s_wasm(err_msg: &Err_Stack /* , msg: &str */) {
        let b = &_G_Xdom_Logger.get().unwrap();
        let mut out = vec![];
        out.push(format!("== {} ==", b.name));

        out.push(Log_Err_Util::url_str(&err_msg.orig));
        out.push(err_msg.orig.msg.clone());

        out.push("----".to_string());

        for x in err_msg.context.iter() {
            out.push(Log_Err_Util::url_str(&x));
            out.push(x.msg.clone());
        }

        let s = out.join("\n");
        (b.log_1)(&s.into())
    }

    pub fn log_2(v0: &wb::JsValue, v1: &wb::JsValue) {
        let b = &_G_Xdom_Logger.get().unwrap();
        let name = b.name.clone();
        (b.log_3)(&(name.into()), v0, v1);
    }
}
