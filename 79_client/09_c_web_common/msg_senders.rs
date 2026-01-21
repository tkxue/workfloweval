use super::*;

/*
pub struct Msg_Senders {
    _self_Id: Id_Proc,
    senders: HashMap<Id_Proc, Msg_Sender>,
}

impl Msg_Senders {
    fn raw_send(&self, proc_id: Id_Proc, msg: Xos_Msg) {
        self.senders.get(&proc_id).unwrap().send(msg);
    }

    pub fn new(x: Id_Proc) -> Msg_Senders {
        let mut out = HashMap::new();
        for k in Id_Proc::iter() {
            out.insert(k, Msg_Sender::new(x, k));
        }
        Msg_Senders { _self_Id: x, senders: out }
    }
}

impl Msg_Senders {
    pub fn set_sender(&self, id_proc: Id_Proc, message_port: XdomA_Message_Port) {
        // let from = PO::__get_singleton().id_proc;

        /*
        if from.should_log() && id_proc.should_log() {
            PO::log(RawTerm::String(
                format!("msg_senders: dst: {:?}, from: {:?}", from, id_proc)
                    .as_bytes()
                    .to_vec(),
            ));
        }

         */

        self.senders.get(&id_proc).unwrap().set_port(message_port);
    }

    pub fn send(&self, msg: Msg_Po) {
        self.raw_send(msg.dst, L_JsData_Util::conv_xos_raw_msg(&msg).un(err!("")));
    }
}

struct Msg_Sender_Inner {
    to_send: Vec<Xos_Msg>,
    port: Option<XdomA_Message_Port>,
}

impl Msg_Sender_Inner {
    pub fn new() -> Msg_Sender_Inner {
        Msg_Sender_Inner { to_send: vec![], port: None }
    }
}

pub struct Msg_Sender {
    inner: ArcState<Msg_Sender_Inner>,
}

impl Msg_Sender {
    pub fn new(from: Id_Proc, to: Id_Proc) -> Msg_Sender {
        Msg_Sender {
            inner: ArcState::new(Msg_Sender_Inner::new()),
        }
    }

    pub fn reset(&self) {
        todo!()

        // self.inner.borrow_mut().port = None;
    }

    pub fn set_port(&self, p: XdomA_Message_Port) {
        todo!()
        /*
        self.inner.borrow_mut().port = Some(p);
        self.flush();

         */
    }

    pub fn flush(&self) {
        todo!()
        /*
        let t = self.inner.borrow_mut().port.clone();
        match t {
            None => {
                // web_sys::console::log_1(&format!("Msg_Sender::flush waiting\n  from: {:?}\n  to: {:?}", self._from_id, self._to_id).into());
            }
            Some(port) => {
                let v = std::mem::replace(&mut self.inner.borrow_mut().to_send, vec![]);
                for t in v.iter() {
                    port.inner
                        .post_message_with_transfer(&t.to_js(), &t.get_transfers());
                }
            }
        }

         */
    }

    pub fn send(&self, t: Xos_Msg) {
        /*
        self.inner.borrow_mut().to_send.push(t);
        self.flush();

         */
        todo!()
    }
}


 */
