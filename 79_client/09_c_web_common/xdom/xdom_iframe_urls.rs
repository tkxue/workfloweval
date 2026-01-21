use super::*;

impl XdomA_IFrame_Urls {
    pub fn new(wasm_version: Arc<String>) -> XdomA_IFrame_Urls {
        let window = &my_ffi::window::WINDOW;
        let location = window.location();
        let href: String = location.href();
        if href.starts_with("http://localhost") {
            XdomA_IFrame_Urls::new_local(wasm_version)
        } else if href.starts_with("http://10.0.2.2") {
            XdomA_IFrame_Urls::new_mobile(wasm_version)
        } else {
            XdomA_IFrame_Urls::new_public(wasm_version)
        }
    }
}

pub trait XdomA_IFrame_Urls_T: Send + Sync {
    fn new(&self, wasm_version: Rc<String>) -> XdomA_IFrame_Urls;
}

pub static _G_XdomA_IFRame_Urls: OnceLock<XdomA_IFrame_Urls> = OnceLock::new();

/*
mm_lazy_singleton! {
    XdomA_IFrame_Urls ;
    Arc<dyn XdomA_IFrame_Urls_T>  ;
    __set_fn__t ;
    __get_val__t ;
}

 */

pub struct XdomA_IFrame_Urls {
    pub gfx: String,
    pub sound: String,
    pub ws: String,
}

impl XdomA_IFrame_Urls {
    pub fn new_local(wasm_version: Arc<String>) -> XdomA_IFrame_Urls {
        let _local = XdomA_IFrame_Urls {
            gfx: format!("http://localhost:3001/pub/static_{}/h_gfx.html", wasm_version.as_str()),
            sound: format!("http://localhost:3002/pub/static_{}/h_vid.html", wasm_version.as_str()),
            ws: "ws://localhost:3010/ws/".to_string(),
        };
        _local
    }

    pub fn new_mobile(wasm_version: Arc<String>) -> XdomA_IFrame_Urls {
        let _local = XdomA_IFrame_Urls {
            gfx: format!("http://10.0.2.2:3001/pub/static_{}/h_gfx.html", wasm_version.as_str()),
            sound: format!("http://10.0.2.2:3002/pub/static_{}/h_vid.html", wasm_version.as_str()),
            ws: "ws://10.0.2.2:3010/ws/".to_string(),
        };
        _local
    }

    pub fn new_public(wasm_version: Arc<String>) -> XdomA_IFrame_Urls {
        XdomA_IFrame_Urls {
            gfx: format!(
                "https://m1.npc-repl.pages.dev/pub/static_{}/h_gfx.html",
                wasm_version.as_str()
            ),
            sound: format!(
                "https://m1.npc-repl.pages.dev/pub/static_{}/h_vid.html",
                wasm_version.as_str()
            ),
            ws: "http://localhost:12345/".to_string(),
        }
    }
}
