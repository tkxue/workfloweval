use super::*;

/*
pub enum Msg_0 {
    Fetch_Code { port: XdomA_Message_Port },
    Rust_Msg(Msg_0_Rust),
}

impl Msg_0 {
    pub fn make_cb(msg_code: Msg_Code) -> Rc<dyn Fn(XdomA_Message_Event)> {
        let msg_code = msg_code.clone();
        Rc::new(move |event: XdomA_Message_Event| {
            let incoming_msg: Result<Msg_0, String> = Msg_0::new(&event);
            match incoming_msg {
                Err(_err) => {
                    wlog!("msg_0 got error: {:?}", _err);
                }
                Ok(Msg_0::Fetch_Code { port }) => {
                    port.inner.post_message(&msg_code.raw); // .un(err!(""));
                }
                Ok(Msg_0::Rust_Msg(x)) => match &x {
                    Msg_0_Rust::Send_One_Message_Port {
                        from_proc_id,
                        dst: id_proc,
                        port,
                    } => {
                        /*
                        if (from_proc_id.should_log() && id_proc.should_log()) {
                            PO::log(RawTerm::String(
                                format!(
                                    "msg_0: make_cb: send_one_message_port: from {:?} to {:?}",
                                    from_proc_id, id_proc
                                )
                                .as_bytes()
                                .to_vec(),
                            ));
                        }

                         */
                        let msg = Msg_0::Rust_Msg(x);
                        XdomA::parent_post_message_with_transfer(&msg.to_js(), &msg.transfers()).un(err!(""));
                    }
                },
            }
        })
    }

    pub fn transfers(&self) -> wb::JsValue {
        match self {
            Msg_0::Fetch_Code { port } => To_Js::arr(vec![port.inner.clone().into()]),
            Msg_0::Rust_Msg(v) => To_Js::arr(v.transfers()),
        }
    }

    pub fn to_js(&self) -> wb::JsValue {
        match self {
            Msg_0::Fetch_Code { port } => To_Js::obj(vec![
                ("tag".to_string(), (0.0_f64).into()),
                (
                    "msg".to_string(),
                    To_Js::obj(vec![("port".to_string(), port.inner.clone().into())]),
                ),
            ]),
            Msg_0::Rust_Msg(msg) => To_Js::obj(vec![
                ("tag".to_string(), (1.0_f64).into()),
                (
                    "msg".to_string(),
                    L_JsData_Util::conv_xos_raw_msg(/* Xos_Msg_Id::Msg_0_Rust.to_u32(), */ msg)
                        .un(err!(""))
                        .to_js(),
                ),
            ]),
        }
    }

    pub fn new(event: &XdomA_Message_Event) -> Result<Msg_0, String> {
        let data: wb::JsValue =
            js_sys::Reflect::get(&event.inner, &"data".into()).map_err(|_| "get .data")?;
        let tag: wb::JsValue = js_sys::Reflect::get(&data, &"tag".into()).map_err(|_| "get .tag")?;
        let msg: wb::JsValue = js_sys::Reflect::get(&data, &"msg".into()).map_err(|_| "get .msg")?;
        if tag.as_f64() == Some(0.) {
            let port: wb::JsValue = js_sys::Reflect::get(&msg, &"port".into()).map_err(|_| "get .port")?;
            let port = XdomA_Message_Port::new(port);
            Ok(Msg_0::Fetch_Code { port: port })
        } else if tag.as_f64() == Some(1.) {
            match Xos_Msg::from_js(msg.clone()).ok() {
                None => {
                    Xdom_Logger::log_2(
                        &"h_index: Incoming_Msg: Failed wb::JsValue -> Xos_Raw_Msg".into(),
                        &msg,
                    );
                    // web_sys::console::log_2( .into(), &msg);
                    Err("h_index: Incoming_Msg: Failed wb::JsValue -> Xos_Raw_Msg".to_string())
                }
                Some(v) => match Xos_Msg::to_typed::<Msg_0_Rust>(&v) {
                    Err(_) => {
                        Xdom_Logger::log_2(&"h_index: Incoming_Msg: Failed Xos_Raw_Msg -> Msg_Level_0".into(), &msg);
                        Err("h_index: Incoming_Msg: Failed Xos_Raw_Msg -> Msg_Level_0".to_string())
                    }
                    Ok(v) => Ok(Msg_0::Rust_Msg(v)),
                },
            }
            // Ok(Incoming_MsRust_Msg(msg))
        } else {
            Xdom_Logger::log_2(&"Incoming_Msnew ".into(), &data);
            Err("unrecognized tag".to_string())
        }
    }
}

#[derive(JsData)]
pub enum Msg_0_Rust {
    Send_One_Message_Port {
        from_proc_id: Id_Proc,
        dst: Id_Proc,
        port: XdomA_Message_Port,
    },
}

impl Msg_0_Rust {
    pub fn transfers(&self) -> Vec<wb::JsValue> {
        match self {
            Msg_0_Rust::Send_One_Message_Port { port, .. } => my_ffi::Per_Actor_Message_Port_Util::transfers3(port),
        }
    }
}




 */
