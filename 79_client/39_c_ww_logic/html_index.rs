use super::*;

use e_api::*;
use web_common::*;

// use e_api::wasm_bindgen::prelude::wasm_bindgen;

use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{
    Id_GActor, Id_Proc, Msg_Code,
    Web_Root,
    /*
    Actor_Config_T, Actor_New_Args, Actor_New_T, Arc_Ac,
    Impl_Actor_T,
    RAc_H_Index,
    XdomA_Message_Port
    */
};

#[wasm_bindgen]
pub struct Rust_Index_Ffi {
    pub(crate) inner: Rust_Index_Main,
}

#[wasm_bindgen]
impl Rust_Index_Ffi {
    pub fn rust_index_ffi__create(name: String, msg_code: wb::JsValue, args: wb::JsValue) -> Rust_Index_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        let inner = Rust_Index_Main::main(name, msg_code, args);

        /*
            to: "h_gfx".to_string(),
            msg: Default::default(),
            transfer_list: vec![],
        });

         */

        Rust_Index_Ffi { inner }
    }
}

pub struct Rust_Index_Main {
    // pub(crate) ac_h_index: Arc_Ac<Ac_H_Index>,
}

impl Rust_Index_Main {
    pub fn reload_logic(&self, msg_code: wb::JsValue) {
        damn_it!("")
    }

    pub fn main(name: String, msg_code: wb::JsValue, args: wb::JsValue) -> Rust_Index_Main {
        Xdom_Logger::__set_loggers__wasm("h_index".to_string());

        let mut msg_code = Msg_Code::new(msg_code.clone());
        let args = args.clone();
        let name = name.clone();

        if !my_ffi::is_web_worker() {
            msg_code.window_location_host = Arc::new(my_ffi::get_window_host_safe().unwrap())
        };

        let root_url = Arc::new(Web_Root::new(
            msg_code.window_location_protocol.clone(),
            msg_code.window_location_host.clone(),
            msg_code.is_dev(),
            msg_code.wasm_version.clone(),
        ));

        match name.as_ref() {
            "h_index" => {
                // let ac_h_index = IAc_H_Index::start(msg_code, args);

                /*
                PO::log(eetf::Term::ByteList(eetf::ByteList {
                    bytes: "h_index init".as_bytes().to_vec(),
                }));

                 */

                H_Index::start(msg_code, args);

                Rust_Index_Main {}
            }
            // =============================================================================
            s => {
                damn_it!("Rust_Index_Main does not recognize: {:?}", s)
            }
        }
    }
}

// mod app_wws;
// pub use app_wws::*;
// use cd_core_api::ordered_float::OrderedFloat;
// use cd_xgpu_pub_api::RAc_Xgpu_Shader;

pub struct IAc_H_Index {
    // global_handlers: Xdom_Global_Handlers,
}

impl IAc_H_Index {
    pub fn send_resize() {
        let _ws = Xdom_Window_Size::new();
        use wasm_bindgen::JsValue;

        my_ffi::log(&JsValue::from("ignoring a send resize"));

        /*
        PO::call_t(&RAc_Xgpu_Shader::Handle_Resize);
        PO::call_t(&RAc_Ww_Desktop::Resize(Xdom_Window_Size::new()));
        PO::call_t(&RAc_Ww_Net::Resize(Xdom_Window_Size::new()));

         */
    }
}

/*
impl<'a> Actor_New_T<Ac_H_Index> for IAc_H_Index {
    fn new_iac(args: Actor_New_Args<Ac_H_Index>) -> Arc_Ac<Ac_H_Index> {
        let msg_code = args.init.msg_code;

        // let po_cbs = PO::__get_singleton();

        let h_index = IAc_H_Index {
            // global_handlers: Xdom_Global_Handlers::new(),
        };

        /*
        h_index.global_handlers.inner.set_on_message(Some(Rc::new({
            move |event: XdomA_Message_Event| {
                let incoming_msg: Result<Msg_0, String> = Msg_0::new(&event);
                match incoming_msg {
                    Err(err) => {
                        wlog!("h_index: got error: {}", err);
                    }
                    Ok(Msg_0::Fetch_Code { port }) => {
                        port.inner.post_message(&msg_code.raw); // .un(err!(""));
                    }
                    Ok(Msg_0::Rust_Msg(msg_0)) => match msg_0 {
                        Msg_0_Rust::Send_One_Message_Port { from_proc_id, dst: dst, port } => {
                            /*
                            if (from_proc_id.should_log() && dst.should_log()) {
                                PO::log(RawTerm::String(
                                    format!(
                                        "index: send_one_message_port: from: {:?} id_proc: {:?}",
                                        from_proc_id, dst
                                    )
                                    .as_bytes()
                                    .to_vec(),
                                ));
                            }

                             */
                            args.init.cpo.set_send_message_port(from_proc_id, dst, port);
                        }
                    },
                }
            }
        })));
        */

        // let gh = &h_index.global_handlers.inner;

        /*
        gh.set_on_keyboard(Some(Rc::new({
            move |event: Key_Event| {
                PO::call_t(&RAc_Ww_Desktop::Keyboard(event));
            }
        })));

         */

        /*
        gh.set_on_resize(Some(Rc::new({
            let po_cbs = po_cbs.clone();
            move |_e: XdomA_UI_Resize_Event| {
                Self::send_resize(&po_cbs);
            }
        })));

         */

        /*
        gh.set_on_mouse(Some(Rc::new({
            move |e: Mouse_Event_Raw| {
                PO::call_t(&RAc_Ww_Desktop::Mouse(e));
            }
        })));

         */

        // todo!();
        // Self::send_resize(&PO::__get_singleton());

        let h_index = Arc_Ac::new0("IAc_H_Index".to_string(), h_index);

        h_index
    }
}
*/

/*
impl<'a> Impl_Actor_T<Ac_H_Index> for IAc_H_Index {
    fn handle_local(&mut self, _ra: &Arc_Ac<Ac_H_Index>, _t: <Ac_H_Index as Actor_Config_T>::Local_In) {
        /*
        match _t {
            Ac_H_Index__Local_In::Reload_Logic(rl) => {
                self.workers.hot_reload(rl);
                // wlog!("IAc_H_Index :: handle_local :: reload_logic")
            }
        }

         */
        todo!()
    }

    fn handle_remote(&mut self, _ra: &Arc_Ac<Ac_H_Index>, t: <Ac_H_Index as Actor_Config_T>::Remote_In) {
        todo!()
        /*
        match t {
            RAc_H_Index::Hide_Sound_Div => {
                wlog!("hiding sound div");
                XdomA::window_focus();
            }
            RAc_H_Index::Init => {
                // wlog!("focus");
            }
        }

         */
    }

    fn id_gactor() -> Option<Id_GActor> {
        Some(Id_GActor::H_Index)
    }
}

impl IAc_H_Index {
}
*/

pub struct H_Index {}

impl H_Index {
    pub fn start(msg_code: Msg_Code, _args: wb::JsValue) /* -> Arc_Ac<Ac_H_Index> */
    {
        let wasm_version = msg_code.wasm_version.clone();
    }
}

// =========================

pub struct Ac_H_Index {}

#[derive(Clone)]
pub struct Ac_H_Index__Args_New {
    pub msg_code: Msg_Code,
    // pub cpo: Rc<Collect_Postoffice>,
}

pub enum Ac_H_Index__Local_In {
    Reload_Logic(Msg_Code),
}

pub enum Ac_H_Index__Local_Out {
    Goto_Url(String),
    Heartbeat,
}

/*
impl<'a> Actor_Config_T<'a> for Ac_H_Index {
    type Local_In = Ac_H_Index__Local_In;
    type Remote_In = RAc_H_Index;
    type Args_New = Ac_H_Index__Args_New;
}

 */

// ==================================

/*
pub struct Collect_Postoffice {
    pub waiting_on: RefCell<HashSet<Id_Proc>>,
    pub index: Arc<PO>,
}

impl Collect_Postoffice {
    pub async fn check_po_status(obj: Rc<Self>) {
        /*
        let mut cnt = 0;
        while !obj.waiting_on.borrow().is_empty() {
            cnt = cnt + 1;

            if cnt % 10 == 0 {
                wlog!("postoffice {:?} waiting on: {:?}", cnt, obj.waiting_on.borrow());
            }

            XdomA::sleep_millis(1000).await;
        }
        wlog!("**********\npostoffice done; time: {:?}\n**********", cnt);
        */
    }

    pub fn new(index: Arc<PO>) -> Collect_Postoffice {
        Collect_Postoffice {
            waiting_on: RefCell::new(Id_Proc::iter().collect::<HashSet<_>>()),
            index,
        }
    }

    pub fn set_send_message_port(&self, from_proc_id: Id_Proc, dst: Id_Proc, v: XdomA_Message_Port) {
        todo!()
        /*
        self.waiting_on.borrow_mut().remove(&from_proc_id);

        fn to_show(x: Id_Proc) -> bool {
            x == Id_Proc::H_Gfx || x == Id_Proc::Ww_Desktop
        }

        /*
        if from_proc_id.should_log() && dst.should_log() {
            PO::log(RawTerm::String(
                format!("cpo :: from {:?} dst {:?}", from_proc_id, dst)
                    .as_bytes()
                    .to_vec(),
            ));
        }
         */

        match dst {
            Id_Proc::H_Index => {
                self.index.senders.set_sender(from_proc_id, v);
            }
            dst => {
                self.index.senders.send(Msg_Po {
                    dst: dst,
                    inner: Msg_Po_Inner::Send_Sender((from_proc_id, v)),
                });
            }
        }
        */
    }
}
*/
