use super::*;

pub struct Ws_Client_Once {
    /*
    pub raw_ws: web_sys::WebSocket,
    _on_message_closure: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _on_open_closure: Closure<dyn FnMut()>,
    _on_error_closure: Closure<dyn FnMut(web_sys::ErrorEvent)>,
    _on_close_closure: Closure<dyn FnMut(web_sys::CloseEvent)>,

     */
}

impl Debug for Ws_Client_Once {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ws_Client_Once")
    }
}

impl Drop for Ws_Client_Once {
    fn drop(&mut self) {
        /*
        self.raw_ws.set_onmessage(None);
        self.raw_ws.set_onopen(None);
        self.raw_ws.set_onerror(None);
        self.raw_ws.set_onclose(None);
        self.raw_ws.close();

         */
        damn_it!("")
    }
}

impl Ws_Client_Once {
    pub fn close(&self) {
        damn_it!("")
        // let _ = self.raw_ws.close();
    }

    pub fn is_open(&self) -> bool {
        damn_it!("")
        // self.raw_ws.ready_state() == 1
    }

    pub fn send<'a>(&self, msg: Ws_Msg_Out<'a>) -> Result<(), Ws_Client_Err> {
        damn_it!("")
        /*
        let ready_state = self.raw_ws.ready_state();
        if ready_state == web_sys::WebSocket::OPEN {
            match msg {
                Ws_Msg_Out::Binary(x) => {
                    self.raw_ws
                        .send_with_u8_array(x)
                        .map_err(|x| Ws_Client_Err::Unknown(format!("{:?}", x)))?;
                    Ok(())
                }
                Ws_Msg_Out::Text(x) => {
                    self.raw_ws
                        .send_with_str(x.as_str())
                        .map_err(|x| Ws_Client_Err::Unknown(format!("{:?}", x)))?;
                    Ok(())
                }
                Ws_Msg_Out::Unknown(_) => Err(Ws_Client_Err::Unknown("unknown".to_string())),
                Ws_Msg_Out::Blob(_) => Err(Ws_Client_Err::Unknown("block".to_string())),
            }
        } else {
            Err(Ws_Client_Err::Not_Ready)
        }
        */
    }

    pub fn new(init: &Ws_Client_Init) -> Res<Ws_Client_Once> {
        damn_it!("")
        /*
        // wlog!("Ws_Client_Once::new {:?}", init.url);
        let raw_ws = web_sys::WebSocket::new(&init.url).map_err(|_x| err!("Can't connect to {:?}", init.url))?;
        raw_ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let on_message_closure = {
            let cb = init.cb.clone();
            Closure::new(move |e: web_sys::MessageEvent| {
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let array = js_sys::Uint8Array::new(&abuf);
                    let xos = Xos_Jab::new_jsv(&array.into());
                    let v = xos.to_vec();
                    cb.call(Ws_Event::Msg(Ws_Msg_In::Binary(Rc::new(v))));
                } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                    cb.call(Ws_Event::Msg(Ws_Msg_In::Blob(blob)));
                } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    let x: String = txt.into();
                    cb.call(Ws_Event::Msg(Ws_Msg_In::Text(Rc::new(x))));
                } else {
                    cb.call(Ws_Event::Msg(Ws_Msg_In::Unknown(e)));
                }
            })
        };

        let on_open_closure = {
            let cb = init.cb.clone();
            Closure::new(move || {
                cb.call(Ws_Event::Open);
            })
        };

        let on_error_closure = {
            let cb = init.cb.clone();
            Closure::new(move |err: web_sys::ErrorEvent| {
                web_sys::console::log_1(&err);
                cb.call(Ws_Event::Error(format!("{:?}", err)));
            })
        };

        let on_close_closure = {
            let cb = init.cb.clone();
            Closure::new(move |err| {
                cb.call(Ws_Event::Close(err));
            })
        };

        raw_ws.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        raw_ws.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
        raw_ws.set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));
        raw_ws.set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));

        Ok(Ws_Client_Once {
            raw_ws,
            _on_message_closure: on_message_closure,
            _on_open_closure: on_open_closure,
            _on_error_closure: on_error_closure,
            _on_close_closure: on_close_closure,
        })
        */
    }
}
